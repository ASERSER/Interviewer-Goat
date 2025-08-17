use tauri::{Manager, AppHandle, Emitter};

mod audio;
mod vad;
mod asr;

use audio::AudioPipeline;
use tracing_subscriber;

#[derive(Clone, serde::Serialize)]
struct SuggestionPayload {
    id: String,
    content: String,
    suggestion_type: String,
    confidence: f32,
}

#[tauri::command]
async fn start_listening(app: AppHandle) -> Result<String, String> {
    let mut pipeline = AudioPipeline::new().map_err(|e| e.to_string())?;
    
    match pipeline.start_streaming().await {
        Ok(mut rx) => {
            // Spawn task to listen for transcripts and emit to frontend
            tokio::spawn(async move {
                while let Ok(transcript) = rx.recv().await {
                    let _ = app.emit("transcript", transcript);
                }
            });
            Ok("Started listening".to_string())
        }
        Err(e) => Err(format!("Failed to start listening: {}", e))
    }
}

#[tauri::command]
async fn stop_listening() -> Result<(), String> {
    // Stop audio processing
    Ok(())
}

#[tauri::command]
async fn copy_suggestion(_suggestion_id: String) -> Result<(), String> {
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
        .setup(|_app| {
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
