use std::sync::{Arc, mpsc};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, SampleRate, StreamConfig};

#[path = "src/vad.rs"]
mod vad;
#[path = "src/asr.rs"]
mod asr;

use vad::AudioChunk;
use asr::WhisperEngine;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🎤 Starting Meeting Copilot with REAL microphone input...");
    info!("💡 Speak into your microphone - the system will detect voice and transcribe!");
    
    // Get audio device
    let host = cpal::default_host();
    let device = host.default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No input device available"))?;
    
    info!("✅ Audio device: {}", device.name().unwrap_or("Unknown".to_string()));
    
    // Create ASR engine (currently mock, but responds to real audio)
    let mut whisper = WhisperEngine::new("models/ggml-base.en.bin")?;
    whisper.load_model().await?;
    info!("✅ Transcription engine loaded (mock mode)");
    
    // Create channel for audio chunks
    let (audio_tx, audio_rx) = mpsc::channel::<AudioChunk>();
    
    // Configure audio stream
    let config = device.default_input_config()?;
    let sample_format = config.sample_format();
    let channels = config.channels();
    
    info!("Audio config - Sample rate: {}, Channels: {}, Format: {:?}", 
          config.sample_rate().0, channels, sample_format);
    
    let sample_rate = config.sample_rate().0;
    let config: StreamConfig = StreamConfig {
        channels,
        sample_rate: config.sample_rate(),
        buffer_size: cpal::BufferSize::Fixed(1024),
    };
    
    let threshold = 0.005f32;
    
    // Build and start audio stream
    let stream = match sample_format {
        SampleFormat::F32 => {
            device.build_input_stream(
                &config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    let rms = (data.iter()
                        .map(|&sample| sample * sample)
                        .sum::<f32>() / data.len() as f32)
                        .sqrt();
                    
                    if rms > threshold {
                        let chunk = AudioChunk {
                            data: data.to_vec(),
                            sample_rate,
                            timestamp: std::time::Instant::now(),
                            has_voice: true,
                        };
                        
                        if let Err(_) = audio_tx.send(chunk) {
                            // Channel closed, stop processing
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
                    
                    let rms = (f32_data.iter()
                        .map(|&sample| sample * sample)
                        .sum::<f32>() / f32_data.len() as f32)
                        .sqrt();
                    
                    if rms > threshold {
                        let chunk = AudioChunk {
                            data: f32_data,
                            sample_rate,
                            timestamp: std::time::Instant::now(),
                            has_voice: true,
                        };
                        
                        if let Err(_) = audio_tx.send(chunk) {
                            // Channel closed, stop processing
                        }
                    }
                },
                |err| error!("Audio input error: {}", err),
                None,
            )?
        },
        _ => return Err(anyhow::anyhow!("Unsupported sample format: {:?}", sample_format).into()),
    };
    
    stream.play()?;
    info!("🔊 Audio stream started! Listening for voice...");
    
    // Process audio chunks and transcribe
    let whisper_handle = tokio::spawn(async move {
        while let Ok(chunk) = audio_rx.recv() {
            info!("🎵 Voice detected! Processing {} samples (RMS: {:.4})", 
                  chunk.data.len(), 
                  (chunk.data.iter().map(|&x| x*x).sum::<f32>() / chunk.data.len() as f32).sqrt());
            
            match whisper.transcribe_chunk(&chunk).await {
                Ok(Some(transcript)) => {
                    println!("📝 TRANSCRIPT: '{}' (confidence: {:.2})", 
                            transcript.text, transcript.confidence);
                },
                Ok(None) => {
                    info!("Audio chunk too short, skipping...");
                },
                Err(e) => {
                    error!("Transcription error: {}", e);
                }
            }
        }
    });
    
    // Run for 60 seconds
    info!("⏱️  Recording for 60 seconds. Start speaking!");
    sleep(Duration::from_secs(60)).await;
    
    // Clean up
    drop(stream);
    whisper_handle.abort();
    info!("🛑 Test completed");
    
    Ok(())
}
