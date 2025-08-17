use anyhow::Result;
use tokio::sync::broadcast;
use tracing::{info, error};
use crate::asr::TranscriptResult;

pub struct AudioPipeline {
    is_running: bool,
}

impl AudioPipeline {
    pub fn new() -> Result<Self> {
        Ok(Self {
            is_running: false,
        })
    }
    
    pub async fn start_streaming(&mut self) -> Result<broadcast::Receiver<TranscriptResult>> {
        if self.is_running {
            return Err(anyhow::anyhow!("Audio pipeline already running"));
        }
        
        let (transcript_tx, transcript_rx) = broadcast::channel::<TranscriptResult>(100);
        
        info!("Starting simplified audio pipeline...");
        
        // Simulate audio processing with mock transcripts
        let tx_clone = transcript_tx.clone();
        tokio::spawn(async move {
            let mut counter = 0;
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                
                let mock_texts = [
                    "Hello, this is a test of the meeting copilot",
                    "The audio pipeline is working correctly",
                    "Real-time transcription is functioning",
                    "Privacy-first processing is active",
                    "All systems are operational",
                ];
                
                let text = mock_texts[counter % mock_texts.len()];
                let result = TranscriptResult {
                    text: text.to_string(),
                    confidence: 0.85 + (counter as f32 * 0.02) % 0.15,
                    start_time: 0.0,
                    end_time: 2.0,
                    language: "en".to_string(),
                };
                
                if tx_clone.send(result).is_err() {
                    break; // No more receivers
                }
                
                counter += 1;
            }
        });
        
        self.is_running = true;
        info!("Audio pipeline started successfully (mock mode)");
        Ok(transcript_rx)
    }
    
    pub fn stop(&mut self) {
        self.is_running = false;
        info!("Audio pipeline stopped");
    }
    
    pub fn is_running(&self) -> bool {
        self.is_running
    }
}
