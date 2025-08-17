use tauri::Manager;

mod audio;
mod guardrails;
mod llm;
mod storage;
mod rag;

use audio::AudioPipeline;
use guardrails::GuardrailEngine;
use llm::LLMOrchestrator;
use storage::Database;

#[derive(Clone, serde::Serialize)]
struct SuggestionPayload {
    id: String,
    content: String,
    suggestion_type: String,
    confidence: f32,
}

#[tauri::command]
async fn start_listening(app_handle: tauri::AppHandle) -> Result<(), String> {
    let audio_pipeline = AudioPipeline::new().map_err(|e| e.to_string())?;
    let guardrail_engine = GuardrailEngine::new().map_err(|e| e.to_string())?;
    
    // Start audio processing in background
    tokio::spawn(async move {
        if let Err(e) = audio_pipeline.start_streaming().await {
            eprintln!("Audio pipeline error: {}", e);
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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            start_listening,
            stop_listening,
            copy_suggestion
        ])
        .setup(|app| {
            // Initialize database
            let db = Database::new("meeting_copilot.db")?;
            app.manage(db);
            
            // Request microphone permissions
            #[cfg(target_os = "macos")]
            {
                // macOS specific permission handling
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
