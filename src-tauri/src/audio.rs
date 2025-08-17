use anyhow::Result;
use std::sync::mpsc;
use tokio::sync::broadcast;
use tracing::{info, error};
use crate::vad::{VadEngine, AudioChunk};
use crate::asr::{WhisperEngine, TranscriptResult};

pub struct AudioPipeline {
    vad_engine: VadEngine,
    whisper_engine: WhisperEngine,
    transcript_tx: Option<broadcast::Sender<TranscriptResult>>,
    is_running: bool,
}

impl AudioPipeline {
    pub fn new() -> Result<Self> {
        let vad_engine = VadEngine::new()?;
        let whisper_engine = WhisperEngine::new("models/ggml-base.en.bin")?;
        
        Ok(Self {
            vad_engine,
            whisper_engine,
            transcript_tx: None,
            is_running: false,
        })
    }
    
    pub async fn start_streaming(&mut self) -> Result<broadcast::Receiver<TranscriptResult>> {
        if self.is_running {
            return Err(anyhow::anyhow!("Audio pipeline already running"));
        }
        
        // Load Whisper model
        let mut whisper_engine = WhisperEngine::new("models/ggml-base.en.bin")?;
        whisper_engine.load_model().await?;
        
        let (audio_tx, audio_rx) = mpsc::channel::<AudioChunk>();
        let (transcript_tx, transcript_rx) = broadcast::channel::<TranscriptResult>(100);
        
        info!("Starting audio pipeline...");
        
        // Start VAD engine in background
        let vad_engine = VadEngine::new()?;
        tokio::spawn(async move {
            if let Err(e) = vad_engine.start_detection(audio_tx).await {
                error!("VAD engine error: {}", e);
            }
        });
        
        // Start ASR processing pipeline
        let transcript_tx_clone = transcript_tx.clone();
        tokio::spawn(async move {
            info!("ASR pipeline started, waiting for audio chunks...");
            
            while let Ok(audio_chunk) = audio_rx.recv() {
                match whisper_engine.transcribe_chunk(&audio_chunk).await {
                    Ok(Some(result)) => {
                        info!("Transcription: '{}'", result.text);
                        if let Err(e) = transcript_tx_clone.send(result) {
                            error!("Failed to send transcript: {}", e);
                        }
                    },
                    Ok(None) => {
                        // Audio chunk too short, skip
                    },
                    Err(e) => {
                        error!("Transcription error: {}", e);
                    }
                }
            }
        });
        
        self.transcript_tx = Some(transcript_tx);
        self.is_running = true;
        
        info!("Audio pipeline started successfully");
        Ok(transcript_rx)
    }
    
    pub fn stop(&mut self) {
        self.is_running = false;
        self.transcript_tx = None;
        info!("Audio pipeline stopped");
    }
    
    pub fn is_running(&self) -> bool {
        self.is_running
    }
    
    pub fn get_status(&self) -> String {
        format!("Pipeline running: {}, VAD: OK, ASR: {}", 
                self.is_running,
                self.whisper_engine.get_model_info())
    }
}
