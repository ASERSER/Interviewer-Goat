# Meeting Copilot MVP 🎤

A privacy-first desktop meeting copilot that provides real-time assistance without joining calls or recording conversations. Built with Tauri, Rust, and React for macOS.

## 🎯 Features

- **Real-time Suggestions**: AI-powered assistance during live conversations
- **Privacy-First**: All processing happens locally, no data leaves your device
- **Smart Guardrails**: Auto-pauses during exams/proctoring contexts
- **Meeting Notes**: Automatic note-taking and action item extraction
- **Searchable Memory**: Query past meeting insights
- **HUD Overlay**: Non-intrusive suggestions with hotkey control

## 🏗️ Architecture

```
Audio Input → VAD → Whisper ASR → Intent Router → Local LLM → HUD Overlay
                                      ↓
                               Vector Store (RAG) ← File Connectors
```

**Tech Stack:**
- **Backend**: Rust with Tauri framework
- **Frontend**: React with TypeScript
- **Audio**: whisper.cpp for ASR, Silero VAD
- **AI**: llama.cpp with quantized models (8-13B)
- **Storage**: SQLite with vector search extensions
- **Privacy**: Local-first processing, encrypted storage

## 🚀 Quick Start

### Prerequisites
- macOS 12+ (Apple Silicon recommended)
- Rust 1.70+
- Node.js 18+

### Installation

```bash
# Clone the repository
git clone https://github.com/ASERSER/Interviewer-Goat.git
cd Interviewer-Goat

# Install dependencies
npm install
cargo build

# Download models (coming soon)
# ./scripts/download-models.sh

# Run in development
npm run tauri dev
```

## 🛡️ Privacy & Security

- **Local Processing**: All audio processing and AI inference happens on-device
- **No Call Recording**: Listens via microphone, never records or stores audio
- **Encrypted Storage**: SQLite database encrypted at rest
- **Guardrails**: Automatic pause during exam/proctoring contexts
- **Consent Indicators**: Clear visual indicators when active

## 📋 Development Roadmap

### Sprint 1: Core Audio Pipeline (Weeks 1-2)
- [x] Project scaffolding and architecture
- [ ] VAD engine implementation
- [ ] Whisper.cpp integration
- [ ] Basic audio → transcript flow

### Sprint 2: Intelligence & HUD (Weeks 3-4)
- [ ] Intent classification and routing
- [ ] Local LLM integration (llama.cpp)
- [ ] React HUD overlay
- [ ] Real-time suggestion system

### Sprint 3: Guardrails & Polish (Weeks 5-6)
- [ ] Privacy guardrails engine
- [ ] Meeting notes and export
- [ ] File connector (local + Google Drive)
- [ ] macOS packaging and distribution

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run latency benchmarks
cargo bench --bench latency

# Manual QA checklist
npm run test:qa
```

## 📦 Building for Production

```bash
# Build optimized release
npm run tauri build

# Package for macOS distribution
npm run package:macos
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙋‍♂️ Support

For questions or support, please open an issue on GitHub.

---

**⚠️ Important**: This is an MVP focused on privacy-first meeting assistance. Always respect meeting participants' consent and local privacy laws.
