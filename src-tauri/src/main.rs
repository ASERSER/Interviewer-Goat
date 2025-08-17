use tauri::Manager;

mod audio;
mod vad;
mod asr;
mod guardrails;
mod router;
mod orchestrator;
mod storage;

use audio::AudioPipeline;
use asr::TranscriptResult;
use tracing_subscriber;

#[derive(Clone, serde::Serialize)]
struct SuggestionPayload {
    id: String,
    content: String,
    suggestion_type: String,
    confidence: f32,
}

#[tauri::command]
async fn start_listening(app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut audio_pipeline = AudioPipeline::new().map_err(|e| e.to_string())?;
    
    // Start audio processing and get transcript stream
    let mut transcript_rx = audio_pipeline.start_streaming().await.map_err(|e| e.to_string())?;
    
    // Forward transcripts to frontend
    let app_handle_clone = app_handle.clone();
    tokio::spawn(async move {
        while let Ok(transcript) = transcript_rx.recv().await {
            let payload = SuggestionPayload {
                id: uuid::Uuid::new_v4().to_string(),
                content: transcript.text,
                suggestion_type: "transcript".to_string(),
                confidence: transcript.confidence,
            };
            
            if let Err(e) = app_handle_clone.emit_all("transcript", &payload) {
                eprintln!("Failed to emit transcript: {}", e);
            }
        }
    });
    
    Ok(())
}

#[tauri::command]
async fn stop_listening() -> Result<(), String> {
    // Stop audio processing
    Ok(())
}

#[tauri::command]
async fn copy_suggestion(suggestion_id: String) -> Result<(), String> {
    // Copy suggestion to clipboard
    Ok(())
}

fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_listening,
            stop_listening,
            copy_suggestion
        ])
        .setup(|app| {
            tracing::info!("Meeting Copilot MVP starting up...");
            
            // Request microphone permissions on macOS
            #[cfg(target_os = "macos")]
            {
                tracing::info!("Requesting microphone permissions...");
                // macOS specific permission handling will be added
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
