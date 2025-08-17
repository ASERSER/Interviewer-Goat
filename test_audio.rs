use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, error};

#[path = "src/audio.rs"]
mod audio;
#[path = "src/vad.rs"]
mod vad;
#[path = "src/asr.rs"]
mod asr;

use audio::AudioPipeline;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("Starting Meeting Copilot audio pipeline test...");
    
    // Create and start audio pipeline
    let mut pipeline = AudioPipeline::new()?;
    info!("Audio pipeline created successfully");
    
    let mut transcript_rx = pipeline.start_streaming().await?;
    info!("Audio pipeline started, listening for transcripts...");
    
    // Listen for transcripts for 30 seconds
    let timeout = sleep(Duration::from_secs(30));
    tokio::pin!(timeout);
    
    loop {
        tokio::select! {
            result = transcript_rx.recv() => {
                match result {
                    Ok(transcript) => {
                        println!("📝 TRANSCRIPT: '{}' (confidence: {:.2})", 
                                transcript.text, transcript.confidence);
                    },
                    Err(e) => {
                        error!("Error receiving transcript: {}", e);
                        break;
                    }
                }
            },
            _ = &mut timeout => {
                info!("Test timeout reached");
                break;
            }
        }
    }
    
    pipeline.stop();
    info!("Audio pipeline test completed");
    
    Ok(())
}
