# MCP API Server

A high-performance REST API server built with Rust and Axum that provides a user-friendly interface for AI agent interactions. This server handles text and audio inputs, integrates with ElevenLabs for speech processing, and communicates with the MCP (Model Context Protocol) server for AI agent responses.

## 📚 Table of Contents

- [What It Does](#-what-it-does)
- [Architecture](#-architecture)
- [Features](#-features)
- [Prerequisites](#-prerequisites)
- [Quick Start](#-quick-start)
- [API Endpoints](#-api-endpoints)
- [Configuration](#-configuration-details)
- [Code Documentation](#-code-documentation)
- [Troubleshooting](#-troubleshooting)
- [Development](#-development)
- [Example Workflow](#-example-workflow)

The MCP API Server acts as a middleware layer that:

1. **Receives user input** (text or audio) via REST API endpoints
2. **Processes audio** using ElevenLabs Speech-to-Text (STT) when needed
3. **Routes requests** to the appropriate AI agent via the MCP server
4. **Converts responses** to speech using ElevenLabs Text-to-Speech (TTS)
5. **Returns results** with both text and audio URLs

## 🏗️ Architecture

```
┌─────────────┐
│   Client    │
│  (Frontend) │
└──────┬──────┘
       │ HTTP POST
       ▼
┌─────────────────────────────────────────┐
│         MCP API Server (Port 8000)      │
│  ┌─────────────────────────────────┐   │
│  │  /agents - List all agents      │   │
│  │  /input/text - Text processing  │   │
│  │  /input/audio - Audio processing│   │
│  └─────────────────────────────────┘   │
└──────┬──────────────────┬───────────────┘
       │                  │
       │ JSON-RPC         │ HTTPS
       ▼                  ▼
┌─────────────┐    ┌──────────────┐
│ MCP Server  │    │  ElevenLabs  │
│ (Port 3000) │    │     API      │
│   Gemini AI │    │  STT & TTS   │
└─────────────┘    └──────────────┘
```

## ✨ Features

- **REST API Endpoints**: Simple HTTP interface for easy integration
- **Audio Processing**: Full speech-to-text and text-to-speech pipeline
- **Agent Management**: List and interact with multiple AI agents
- **File Storage**: Local audio file storage with public access
- **CORS Support**: Ready for cross-origin frontend requests
- **Async/Await**: High-performance concurrent request handling
- **Type-Safe**: Strongly typed Rust for reliability

## 📋 Prerequisites

- **Rust** 1.70 or higher ([Install Rust](https://rustup.rs/))
- **ElevenLabs API Key** ([Get your key](https://elevenlabs.io/))
- **MCP Server** running on port 3000 (see `mcp-server/README.md`)

## 🚀 Quick Start

### 1. Configure Environment

Create a `.env` file in the `mcp-api` directory:

```env
# MCP Server Configuration
MCP_SERVER_URL=http://localhost:3000

# ElevenLabs API Configuration
ELEVENLABS_API_KEY=your_elevenlabs_api_key_here

# Audio Storage Configuration
AUDIO_DIR=public/audio

# Logging Configuration
RUST_LOG=info
```

### 2. Get Your ElevenLabs API Key

1. Sign up at [elevenlabs.io](https://elevenlabs.io/)
2. Go to **Profile Settings** → **API Keys**
3. Copy your API key and paste it in the `.env` file

### 3. Build and Run

```powershell
# Navigate to the directory
cd mcp-api

# Build the project (release mode for production)
cargo build --release

# Run the server
cargo run --release
```

The server will start on `http://127.0.0.1:8000`

### 4. Verify It's Running

```powershell
# Health check
Invoke-RestMethod -Uri "http://localhost:8000/health"
# Should return: OK
```

## 📡 API Endpoints

### GET `/health`
Health check endpoint.

**Response:**
```
OK
```

---

### GET `/agents`
List all available AI agents.

**Response:**
```json
[
  {
    "id": "agent_001",
    "name": "General Assistant",
    "description": "A helpful general-purpose AI assistant"
  },
  {
    "id": "agent_002",
    "name": "Web3 Expert",
    "description": "Specialized in blockchain, cryptocurrency, and Web3 technologies"
  },
  {
    "id": "agent_003",
    "name": "Voice Assistant",
    "description": "Optimized for voice interactions and conversational responses"
  },
  {
    "id": "agent_004",
    "name": "Code Helper",
    "description": "Expert in programming, debugging, and code review"
  }
]
```

---

### POST `/input/text`
Process text input and get agent response with audio.

**Request:**
```json
{
  "agent_id": "agent_002",
  "user_text": "What is blockchain?"
}
```

**Response:**
```json
{
  "reply_text": "A blockchain is a distributed, immutable ledger...",
  "audio_url": "/public/audio/550e8400-e29b-41d4-a716-446655440000.mp3"
}
```

**PowerShell Example:**
```powershell
Invoke-RestMethod -Uri "http://localhost:8000/input/text" `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"agent_id":"agent_002","user_text":"What is blockchain?"}'
```

---

### POST `/input/audio`
Process audio input, transcribe it, and get agent response with audio.

**Request:** Multipart form data
- `audio_file`: Audio file (MP3, WAV, etc.)
- `agent_id`: String (e.g., "agent_003")

**Response:**
```json
{
  "reply_text": "Based on what you said...",
  "audio_url": "/public/audio/660e8400-e29b-41d4-a716-446655440000.mp3"
}
```

**PowerShell Example:**
```powershell
$form = @{
    audio_file = Get-Item -Path "C:\path\to\audio.mp3"
    agent_id = "agent_003"
}
Invoke-RestMethod -Uri "http://localhost:8000/input/audio" `
  -Method POST `
  -Form $form
```

---

### GET `/public/audio/{filename}`
Access generated audio files.

**Example:**
```
http://localhost:8000/public/audio/550e8400-e29b-41d4-a716-446655440000.mp3
```

## 🔧 Configuration Details

### ElevenLabs Settings

**Text-to-Speech (TTS):**
- Model: `eleven_multilingual_v2` (free tier compatible)
- Voice: Rachel (ID: `21m00Tcm4TlvDq8ikWAM`)
- Output: MP3 at 44.1kHz, 128kbps

**Speech-to-Text (STT):**
- Model: `scribe_v1` (only supported STT model)
- Language: English with auto-detection
- Features: Audio event tagging (detects laughter, applause, etc.)

### Customizing the Voice

To change the TTS voice, edit `src/handlers.rs`:

```rust
// Line ~230 and ~502
let tts_url = "https://api.elevenlabs.io/v1/text-to-speech/YOUR_VOICE_ID_HERE";
```

Find available voices at [ElevenLabs Voice Library](https://elevenlabs.io/app/voice-library).

## 📊 Project Structure

```
mcp-api/
├── src/
│   ├── main.rs         # Server setup and routing
│   ├── handlers.rs     # Request handlers for all endpoints
│   └── models.rs       # Data structures and types
├── public/
│   └── audio/          # Generated audio files stored here
├── .env                # Environment configuration
├── Cargo.toml          # Rust dependencies
└── README.md           # This file
```

## 🐛 Troubleshooting

### Server won't start

**Error:** `ELEVENLABS_API_KEY must be set in .env file`
- **Solution:** Make sure you created a `.env` file with your API key

**Error:** `Failed to bind to address`
- **Solution:** Port 8000 is already in use. Kill the existing process or change the port in `main.rs`

### API Errors

**Error:** `Error from MCP service`
- **Solution:** Make sure the MCP server is running on port 3000
- Start it with: `cd mcp-server && cargo run --release`

**Error:** `401 Unauthorized` from ElevenLabs
- **Solution:** Check your API key is correct in `.env`
- Verify your account is active at [elevenlabs.io](https://elevenlabs.io/)

**Error:** `Quota exceeded`
- **Solution:** You've used your free tier limit
- Check usage at [ElevenLabs Dashboard](https://elevenlabs.io/app/usage)
- Upgrade to a paid plan or wait for monthly reset

### Audio Issues

**Audio files not accessible:**
- **Solution:** Make sure the `public/audio` directory exists
- The server creates it automatically on startup

**Poor audio quality:**
- **Solution:** Try different voices from the ElevenLabs library
- Adjust output format in the code if needed

## 🔗 Dependencies

- **axum** 0.8 - High-performance web framework
- **tokio** 1.0 - Async runtime for concurrent operations
- **reqwest** 0.12 - HTTP client for external APIs (ElevenLabs, MCP)
- **serde** / **serde_json** - JSON serialization/deserialization
- **tower-http** 0.6 - CORS and static file serving middleware
- **tracing** / **tracing-subscriber** - Structured logging
- **dotenv** 0.15 - Environment variable management
- **uuid** 1.0 - Unique filename generation for audio files
- **tokio-util** 0.7 - Async I/O utilities

### Why These Dependencies?

- **Axum**: Type-safe, fast, and ergonomic web framework built on Tower
- **Tokio**: Industry-standard async runtime with excellent performance
- **Reqwest**: Feature-rich HTTP client with multipart form support for file uploads
- **Tower**: Modular middleware ecosystem for cross-cutting concerns (CORS, logging)
- **Tracing**: Production-ready structured logging that scales


## 📝 Development

### Building for Development

```powershell
cargo build
cargo run
```

### Running with Debug Logs

```powershell
$env:RUST_LOG="mcp_api=debug"
cargo run
```

### Running Tests

```powershell
cargo test
```

### Code Formatting

```powershell
# Format code
cargo fmt

# Check formatting without making changes
cargo fmt --check
```

### Linting

```powershell
# Run clippy for code quality checks
cargo clippy

# Run clippy with all warnings
cargo clippy -- -W clippy::all
```

---

## 📖 Code Documentation

This project includes comprehensive inline documentation for all code components. You can view the full documentation in your browser:

### Generate and View Documentation

```powershell
# Generate and open documentation in your browser
cargo doc --open

# Generate documentation without opening
cargo doc --no-deps

# Generate documentation with private items
cargo doc --document-private-items
```

### Documentation Structure

The generated documentation includes:

- **Module Overview** (`src/main.rs`)
  - Server architecture explanation
  - Endpoint descriptions
  - Initialization process

- **Handlers Module** (`src/handlers.rs`)
  - `get_agents_list()` - Fetches agents from MCP server
  - `handle_text_input()` - Text processing with TTS
  - `handle_audio_input()` - Audio processing (STT → MCP → TTS)
  - Full parameter and return type documentation
  - Error handling details

- **Models Module** (`src/models.rs`)
  - `AgentInfo` - Agent representation
  - `InputTextRequest` - Text input structure
  - `AgentReplyResponse` - Response format
  - `JsonRpcRequest` / `JsonRpcResponse` - RPC protocol types
  - All struct fields documented with examples

### Inline Documentation Features

✅ **Module-level documentation** - Overview of each file's purpose  
✅ **Function documentation** - Parameters, returns, errors, examples  
✅ **Struct documentation** - Field descriptions and usage  
✅ **Example code** - JSON request/response examples  
✅ **Cross-references** - Links between related items  

### Reading the Documentation

1. **Navigate by module** - Start with the main page and drill down
2. **Search functionality** - Use the search bar to find specific items
3. **Type signatures** - Click on types to see their definitions
4. **Source code links** - View the actual implementation

---

## 🌟 Example Workflow

1. **Client sends text:** "Explain smart contracts"
2. **MCP API receives** the request at `/input/text`
3. **MCP API forwards** to MCP server via JSON-RPC
4. **MCP server** queries Gemini AI with Web3 Expert agent
5. **Gemini returns** detailed explanation
6. **MCP API converts** text to speech via ElevenLabs
7. **Audio file saved** to `public/audio/`
8. **Client receives** both text and audio URL

## 📄 License

This project is part of the web3-valet system.

---

## 📚 Related Documentation

- **[Frontend Integration Guide](../FRONTEND_INTEGRATION.md)** - Complete guide for integrating with web frontends (React, Vue, Vanilla JS)
- **[MCP Server Documentation](../mcp-server/README.md)** - Backend AI agent server documentation
- **[Code Documentation](#-code-documentation)** - View inline Rust documentation with `cargo doc --open`

---

## 🤝 Related Projects

- **mcp-server** - The AI agent backend using Gemini API
- **web3-minting** - NFT minting functionality

---

**Need help?** Check the logs with `RUST_LOG=debug` for detailed information.
