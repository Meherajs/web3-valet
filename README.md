# Web3 Valet - Voice-First AI Concierge System

A comprehensive voice-first Web3 concierge application that combines AI-powered conversational agents with blockchain minting capabilities. This system enables users to interact with specialized AI agents through voice or text, receive intelligent responses, and optionally mint those interactions as NFTs.

## 🎯 Overview

Web3 Valet is a full-stack application that seamlessly integrates:

- **Voice/Text Input** - Natural conversation interface with live audio visualization
- **Multi-Agent AI System** - Specialized AI agents powered by Google's Gemini 2.0
- **Speech Processing** - Real-time speech-to-text and text-to-speech via ElevenLabs
- **NFT Minting** - Blockchain integration for minting conversation artifacts
- **Modern Frontend** - Beautiful React UI with smooth animations

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Frontend (React)                         │
│                      http://localhost:5173                      │
│  - Voice recording with live visualizer                         │
│  - Agent selection interface                                    │
│  - Chat UI with animations                                      │
│  - NFT minting modal                                            │
└────────────────┬────────────────────────────────────────────────┘
                 │ HTTP REST API
                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                    MCP API Server (Rust)                        │
│                      http://localhost:8000                      │
│  - REST endpoints for text/audio input                          │
│  - ElevenLabs STT/TTS integration                               │
│  - Audio file storage and serving                               │
└────────┬───────────────────────────────┬────────────────────────┘
         │ JSON-RPC 2.0                  │ HTTPS
         ▼                               ▼
┌─────────────────────┐         ┌──────────────────┐
│   MCP Server (Rust) │         │   ElevenLabs API │
│  http://localhost:3000│       │   STT & TTS      │
│  - 4 AI agents      │         └──────────────────┘
│  - Gemini 2.0 API   │
└─────────────────────┘
         
┌─────────────────────────────────────────────────────────────────┐
│              Web3 Minting Service (Rust)                        │
│                      http://localhost:8081                      │
│  - Metadata preparation                                         │
│  - IPFS/Arweave storage                                         │
│  - Blockchain NFT minting                                       │
└─────────────────────────────────────────────────────────────────┘
```

## 📦 Components

### 1. Frontend (`frontend/`)

**Tech Stack:** React 19, TypeScript, Vite, Tailwind CSS, Framer Motion

A beautiful, animated chat interface with:
- Real-time audio recording with 32-bar frequency visualizer
- Agent selection UI
- Smooth message animations
- NFT minting modal
- File upload support

**Port:** 5173  
**[📖 Documentation](frontend/README.md)**

### 2. MCP API Server (`mcp-api/`)

**Tech Stack:** Rust, Axum, Reqwest, Tokio

REST API middleware that:
- Provides HTTP endpoints for text and audio input
- Converts speech to text using ElevenLabs STT
- Routes requests to MCP Server via JSON-RPC
- Converts AI responses to speech using ElevenLabs TTS
- Serves generated audio files

**Port:** 8000  
**[📖 Documentation](mcp-api/README.md)**

### 3. MCP Server (`mcp-server/`)

**Tech Stack:** Rust, Axum, Google Gemini API

JSON-RPC 2.0 server with:
- 4 specialized AI agents (General, Web3 Expert, Voice, Code Helper)
- Google Gemini 2.0 Flash integration
- System instruction customization per agent
- Response metadata (tokens, processing time)

**Port:** 3000  
**[📖 Documentation](mcp-server/README.md)**

### 4. Web3 Minting Service (`web3-minting/`)

**Tech Stack:** Rust, Blockchain libraries, IPFS

NFT minting backend that:
- Accepts mint requests with metadata
- Uploads assets to decentralized storage
- Mints NFTs on blockchain
- Returns token ID and transaction hash

**Port:** 8081  
**[📖 Documentation](web3-minting/README.md)**

## 🤖 AI Agents

### Agent 001 - General Assistant
**Best for:** General questions, everyday tasks, broad knowledge queries

### Agent 002 - Web3 Expert
**Best for:** Blockchain, cryptocurrency, DeFi, NFTs, smart contracts

### Agent 003 - Voice Assistant
**Best for:** Natural conversation, voice interactions, audio-based queries

### Agent 004 - Code Helper
**Best for:** Programming questions, debugging, code review, architecture

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18+ (for frontend)
- **Rust** 1.70+ ([Install Rust](https://rustup.rs/))
- **Google Gemini API Key** ([Get key](https://aistudio.google.com/app/apikey))
- **ElevenLabs API Key** ([Get key](https://elevenlabs.io/))

### 1. Clone the Repository

```bash
git clone https://github.com/Meherajs/web3-valet.git
cd web3-valet
```

### 2. Configure Environment Variables

**MCP Server (.env in `mcp-server/`):**
```env
GEMINI_API_KEY=your_gemini_api_key_here
RUST_LOG=info
```

**MCP API (.env in `mcp-api/`):**
```env
MCP_SERVER_URL=http://localhost:3000
ELEVENLABS_API_KEY=your_elevenlabs_api_key_here
AUDIO_DIR=public/audio
RUST_LOG=info
```

**Web3 Minting (.env in `web3-minting/`):**
```env
RPC_URL=your_blockchain_rpc_url
WALLET_PRIVATE_KEY=your_wallet_private_key
CONTRACT_ADDRESS=your_nft_contract_address
```

### 3. Start the Backend Services

**Terminal 1 - MCP Server:**
```bash
cd mcp-server
cargo run --release
```

**Terminal 2 - MCP API:**
```bash
cd mcp-api
cargo run --release
```

**Terminal 3 - Web3 Minting (Optional):**
```bash
cd web3-minting
cargo run --release
```

### 4. Start the Frontend

**Terminal 4 - Frontend:**
```bash
cd frontend
npm install
npm run dev
```

### 5. Open the Application

Navigate to **http://localhost:5173** in your browser.

## 🎮 How to Use

1. **Select an Agent** - Choose from 4 specialized AI agents
2. **Interact** - Type a message or click the microphone to record voice
3. **Receive Response** - Get text and audio responses from the AI
4. **Mint NFT** (Optional) - Mint your conversation as an NFT

## 📊 Project Structure

```
web3-valet/
├── frontend/              # React frontend application
│   ├── src/
│   │   ├── components/    # UI components
│   │   ├── App.tsx        # Main app component
│   │   └── main.tsx       # Entry point
│   └── package.json
├── mcp-api/               # REST API server
│   ├── src/
│   │   ├── main.rs        # Server setup
│   │   ├── handlers.rs    # Request handlers
│   │   └── models.rs      # Data models
│   └── Cargo.toml
├── mcp-server/            # JSON-RPC AI agent server
│   ├── src/
│   │   ├── main.rs        # Server initialization
│   │   ├── agents.rs      # Agent definitions
│   │   ├── gemini.rs      # Gemini API client
│   │   ├── handlers.rs    # RPC handlers
│   │   └── models.rs      # Data structures
│   └── Cargo.toml
└── web3-minting/          # NFT minting service
    ├── src/
    │   ├── main.rs        # Service entry point
    │   ├── blockchain.rs  # Blockchain interaction
    │   └── storage.rs     # IPFS/storage logic
    └── Cargo.toml
```

## 🔧 API Endpoints

### MCP API Server (Port 8000)

- `GET /health` - Health check
- `GET /agents` - List all available agents
- `POST /input/text` - Process text input
- `POST /input/audio` - Process audio input
- `GET /public/audio/{filename}` - Serve audio files

### MCP Server (Port 3000)

**JSON-RPC 2.0 Methods:**
- `list_agents` - Get all agents
- `process_text` - Send text to an agent

### Web3 Minting Service (Port 8081)

- `POST /mint` - Mint NFT with metadata
- `GET /status/{token_id}` - Check minting status
- `GET /assets` - List minted assets

## 🧪 Testing

### Test MCP API

```powershell
# List agents
Invoke-RestMethod -Uri "http://localhost:8000/agents"

# Send text
Invoke-RestMethod -Uri "http://localhost:8000/input/text" `
  -Method POST `
  -ContentType "application/json" `
  -Body '{"agent_id":"agent_002","user_text":"What is blockchain?"}'
```

### Test MCP Server

```powershell
$body = @{
    jsonrpc = "2.0"
    method = "process_text"
    params = @{
        agent_id = "agent_001"
        user_text = "Hello!"
    }
    id = 1
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000" `
  -Method POST `
  -ContentType "application/json" `
  -Body $body
```

## 📖 Documentation

Each component has detailed documentation:

- **[Frontend Documentation](frontend/README.md)** - React UI setup and usage
- **[MCP API Documentation](mcp-api/README.md)** - REST API reference
- **[MCP Server Documentation](mcp-server/README.md)** - AI agent system
- **[Web3 Minting Documentation](web3-minting/README.md)** - NFT minting service

### Code Documentation

Generate Rust documentation:

```bash
cd mcp-server  # or mcp-api or web3-minting
cargo doc --open
```

## 🛠️ Development

### Frontend Development

```bash
cd frontend
npm run dev      # Start dev server
npm run build    # Build for production
npm run preview  # Preview production build
```

### Backend Development

```bash
# Any Rust service
cargo build      # Debug build
cargo build --release  # Production build
cargo test       # Run tests
cargo fmt        # Format code
cargo clippy     # Lint code
```

## 🐛 Troubleshooting

### Common Issues

**Frontend can't connect to backend:**
- Verify all backend services are running
- Check CORS configuration in MCP API
- Ensure correct URLs in frontend config

**Audio not working:**
- Check microphone permissions in browser
- Verify ElevenLabs API key is valid
- Ensure `public/audio` directory exists

**Agent responses failing:**
- Verify Gemini API key is correct
- Check MCP Server is running on port 3000
- Review logs with `RUST_LOG=debug`

**Minting fails:**
- Verify blockchain RPC URL is accessible
- Check wallet has sufficient funds
- Ensure contract address is correct

## 🔒 Security

- ⚠️ **Never commit `.env` files** with API keys
- ✅ Add `.env` to `.gitignore`
- ✅ Use environment variables in production
- ✅ Rotate API keys periodically
- ✅ Use secure RPC endpoints for blockchain

## 🚧 Future Enhancements

- [ ] Add conversation history persistence
- [ ] Implement user authentication
- [ ] Support multiple blockchain networks
- [ ] Add voice activity detection (VAD)
- [ ] Implement agent memory/context
- [ ] Add batch minting capabilities
- [ ] Support for multiple languages
- [ ] Real-time agent switching during conversation

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📧 Contact

Project Repository: [https://github.com/Meherajs/web3-valet](https://github.com/Meherajs/web3-valet)

## 🙏 Acknowledgements

- **Google Gemini** - AI model powering the agents
- **ElevenLabs** - Speech-to-text and text-to-speech services
- **Rust Community** - Excellent async ecosystem (Tokio, Axum)
- **React Community** - UI libraries and tooling
- **Web3 Community** - Blockchain integration tools

---

**Built with ❤️ for the future of voice-first Web3 interactions**
