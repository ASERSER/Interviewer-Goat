import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/tauri';
import { listen } from '@tauri-apps/api/event';

interface Suggestion {
  id: string;
  content: string;
  suggestion_type: string;
  confidence: number;
}

export function HUD() {
  const [suggestions, setSuggestions] = useState<Suggestion[]>([]);
  const [isListening, setIsListening] = useState(false);
  const [isPinned, setIsPinned] = useState(false);

  useEffect(() => {
    // Listen for transcript events from Rust backend
    const unlistenTranscript = listen('transcript', (event: any) => {
      const transcript = event.payload as Suggestion;
      setSuggestions(prev => [transcript, ...prev.slice(0, 4)]); // Keep last 5
    });

    return () => {
      unlistenTranscript.then(fn => fn());
    };
  }, []);

  const handleStartListening = async () => {
    try {
      await invoke('start_listening');
      setIsListening(true);
    } catch (error) {
      console.error('Failed to start listening:', error);
    }
  };

  const handleStopListening = async () => {
    try {
      await invoke('stop_listening');
      setIsListening(false);
    } catch (error) {
      console.error('Failed to stop listening:', error);
    }
  };

  const handleCopySuggestion = async (suggestionId: string) => {
    try {
      await invoke('copy_suggestion', { suggestionId });
      // Visual feedback for copy action
    } catch (error) {
      console.error('Failed to copy suggestion:', error);
    }
  };

  return (
    <div className={`hud-overlay ${isPinned ? 'pinned' : ''}`}>
      <div className="hud-header">
        <h2>Meeting Copilot</h2>
        <div className="controls">
          <button 
            onClick={isListening ? handleStopListening : handleStartListening}
            className={`listen-btn ${isListening ? 'active' : ''}`}
          >
            {isListening ? '🔴 Stop' : '🎤 Start'}
          </button>
          <button 
            onClick={() => setIsPinned(!isPinned)}
            className="pin-btn"
          >
            📌
          </button>
        </div>
      </div>
      
      <div className="status">
        {isListening ? (
          <span className="listening">🎧 Listening for voice...</span>
        ) : (
          <span className="idle">Click Start to begin transcription</span>
        )}
      </div>
      
      <div className="suggestions-container">
        {suggestions.length === 0 ? (
          <div className="empty-state">
            <p>No transcripts yet</p>
            <p>Start listening to see real-time transcription</p>
          </div>
        ) : (
          suggestions.map((suggestion) => (
            <div key={suggestion.id} className="suggestion-card">
              <div className="suggestion-header">
                <span className="suggestion-type">{suggestion.suggestion_type}</span>
                <span className="confidence">
                  {Math.round(suggestion.confidence * 100)}%
                </span>
              </div>
              <div className="suggestion-content">{suggestion.content}</div>
              <div className="suggestion-actions">
                <button 
                  onClick={() => handleCopySuggestion(suggestion.id)}
                  className="copy-btn"
                >
                  📋 Copy
                </button>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}
