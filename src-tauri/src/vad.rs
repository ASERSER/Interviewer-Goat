use anyhow::Result;
use std::sync::mpsc;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, SampleRate, StreamConfig};
use tracing::{info, warn, error};

pub struct VadEngine {
    threshold: f32,
    frame_size: usize,
    sample_rate: u32,
    device: Option<Device>,
}

#[derive(Debug, Clone)]
pub struct AudioChunk {
    pub data: Vec<f32>,
    pub sample_rate: u32,
    pub timestamp: std::time::Instant,
    pub has_voice: bool,
}

impl VadEngine {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;
        
        info!("Audio device: {}", device.name().unwrap_or("Unknown".to_string()));
        
        Ok(Self {
            threshold: 0.005, // Lowered threshold for better sensitivity
            frame_size: 1024,
            sample_rate: 16000, // Standard for Whisper
            device: Some(device),
        })
    }
    
    pub fn detect_voice(&self, audio_data: &[f32]) -> bool {
        if audio_data.is_empty() {
            return false;
        }
        
        // Calculate RMS energy
        let rms = (audio_data.iter()
            .map(|&sample| sample * sample)
            .sum::<f32>() / audio_data.len() as f32)
            .sqrt();
        
        // Simple spectral centroid for better voice detection
        let mut spectral_energy = 0.0f32;
        for i in 1..audio_data.len().min(512) {
            spectral_energy += (audio_data[i] - audio_data[i-1]).abs();
        }
        spectral_energy /= audio_data.len().min(512) as f32;
        
        let voice_detected = rms > self.threshold && spectral_energy > 0.001;
        
        if voice_detected {
            info!("Voice detected - RMS: {:.4}, Spectral: {:.4}", rms, spectral_energy);
        }
        
        voice_detected
    }
    
    pub async fn start_detection(&self, tx: mpsc::Sender<AudioChunk>) -> Result<()> {
        let device = self.device.as_ref()
            .ok_or_else(|| anyhow::anyhow!("No audio device configured"))?;
        
        let supported_configs = device.supported_input_configs()?;
        info!("Supported audio configs: {:?}", supported_configs.collect::<Vec<_>>());
        
        // Try to get 16kHz mono config for Whisper compatibility
        let config = device.default_input_config()?;
        let sample_format = config.sample_format();
        let channels = config.channels();
        
        info!("Audio config - Sample rate: {}, Channels: {}, Format: {:?}", 
              config.sample_rate().0, channels, sample_format);
        
        let config: StreamConfig = StreamConfig {
            channels,
            sample_rate: SampleRate(16000), // Force 16kHz for Whisper
            buffer_size: cpal::BufferSize::Fixed(self.frame_size as u32),
        };
        
        let tx_clone = tx.clone();
        let threshold = self.threshold;
        
        let stream = match sample_format {
            SampleFormat::F32 => {
                device.build_input_stream(
                    &config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        let chunk = AudioChunk {
                            data: data.to_vec(),
                            sample_rate: 16000,
                            timestamp: std::time::Instant::now(),
                            has_voice: Self::detect_voice_static(data, threshold),
                        };
                        
                        if chunk.has_voice {
                            if let Err(e) = tx_clone.send(chunk) {
                                error!("Failed to send audio chunk: {}", e);
                            }
                        }
                    },
                    |err| error!("Audio input error: {}", err),
                    None,
                )?
            },
            SampleFormat::I16 => {
                device.build_input_stream(
                    &config,
                    move |data: &[i16], _: &cpal::InputCallbackInfo| {
                        let f32_data: Vec<f32> = data.iter()
                            .map(|&sample| sample as f32 / i16::MAX as f32)
                            .collect();
                        
                        let chunk = AudioChunk {
                            data: f32_data.clone(),
                            sample_rate: 16000,
                            timestamp: std::time::Instant::now(),
                            has_voice: Self::detect_voice_static(&f32_data, threshold),
                        };
                        
                        if chunk.has_voice {
                            if let Err(e) = tx_clone.send(chunk) {
                                error!("Failed to send audio chunk: {}", e);
                            }
                        }
                    },
                    |err| error!("Audio input error: {}", err),
                    None,
                )?
            },
            _ => return Err(anyhow::anyhow!("Unsupported sample format: {:?}", sample_format)),
        };
        
        stream.play()?;
        info!("Audio stream started successfully");
        
        // Keep stream alive - in production this would be managed differently
        tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
        
        Ok(())
    }
    
    fn detect_voice_static(audio_data: &[f32], threshold: f32) -> bool {
        if audio_data.is_empty() {
            return false;
        }
        
        let rms = (audio_data.iter()
            .map(|&sample| sample * sample)
            .sum::<f32>() / audio_data.len() as f32)
            .sqrt();
        
        rms > threshold
    }
}
