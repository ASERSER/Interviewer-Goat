use anyhow::Result;
use std::sync::mpsc;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct VadEngine {
    threshold: f32,
    frame_size: usize,
}

impl VadEngine {
    pub fn new() -> Self {
        Self {
            threshold: 0.01, // Voice activity threshold
            frame_size: 1024,
        }
    }
    
    pub fn detect_voice(&self, audio_data: &[f32]) -> bool {
        let energy = audio_data.iter()
            .map(|&sample| sample * sample)
            .sum::<f32>() / audio_data.len() as f32;
        
        energy > self.threshold
    }
    
    pub async fn start_detection(&self, tx: mpsc::Sender<Vec<f32>>) -> Result<()> {
        let host = cpal::default_host();
        let device = host.default_input_device()
            .ok_or_else(|| anyhow::anyhow!("No input device available"))?;
        
        let config = device.default_input_config()?;
        
        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if self.detect_voice(data) {
                    let _ = tx.send(data.to_vec());
                }
            },
            |err| eprintln!("Audio input error: {}", err),
            None,
        )?;
        
        stream.play()?;
        
        // Keep stream alive
        std::thread::sleep(std::time::Duration::from_secs(3600));
        
        Ok(())
    }
}
