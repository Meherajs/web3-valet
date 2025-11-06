# AI Chat Assistant - Frontend

A beautiful, animated chat interface with real-time audio recording and voice visualization built with React, TypeScript, and Framer Motion. Connects to the MCP API backend for AI-powered conversations.

## Features

- **Real-time Audio Recording** - Record voice messages with live audio visualization
- **AI Agent Selection** - Choose from multiple specialized AI agents
- **Backend Integration** - Full integration with MCP API and AI services
- **Smooth Animations** - Framer Motion powered message animations
- **Audio Playback** - Listen to AI responses with custom audio player
- **Thinking Indicator** - Beautiful Lottie animation while processing
- **File Upload** - Support for audio file uploads
- **TypeScript** - Fully typed for better developer experience
- **Audio Visualizer** - 32-bar frequency visualizer that reacts to your voice

## Prerequisites

- Node.js (v18 or higher)
- npm or yarn
- **MCP API Server** running on port 8000
- **MCP Server** running on port 3000

> **Important:** Make sure the backend services are running before starting the frontend. See [Backend Setup](#backend-setup) below.

## Backend Setup

Before running the frontend, you need to start the backend services:

### 1. Start MCP Server (AI Agent Backend)

```powershell
cd ../mcp-server
cargo run --release
```

This starts the AI agent server on **http://localhost:3000**

### 2. Start MCP API (REST API Middleware)

```powershell
cd ../mcp-api
cargo run --release
```

This starts the REST API on **http://localhost:8000**

## Installation

1. Install dependencies:

```bash
npm install
```

Or if you encounter PowerShell restrictions:

```bash
npm.cmd install
```

2. Configure environment variables:

Create or verify the `.env` file in the `frontend` directory:

```env
VITE_API_BASE_URL=http://localhost:8000
```

> **Note:** The `.env.example` file is provided as a template.

## Development

Run the development server:

```bash
npm run dev
```

Or:

```bash
npm.cmd run dev
```

The app will be available at `http://localhost:5173`

## How to Use

1. **Select an Agent**: Choose from 4 specialized AI agents:
   - **General Assistant** - General questions and tasks
   - **Web3 Expert** - Blockchain and cryptocurrency expertise
   - **Voice Assistant** - Natural conversational interactions
   - **Code Helper** - Programming and debugging assistance

2. **Type a Message**: Enter text in the input area and press Enter or click the send button

3. **Record Audio**: Click the microphone icon to start recording
   - Allow microphone permission when prompted
   - Watch the live audio visualizer react to your voice
   - Click the stop button when finished
   - The audio will be transcribed and sent to the AI

4. **Upload Audio**: Click the paperclip icon to upload an audio file (WAV or MP3)

5. **View Responses**: 
   - Messages animate in smoothly
   - AI responses include both text and audio
   - Click play on the audio player to hear the response

6. **Mint NFT** (Optional): Click "Mint this Result" on any agent response to mint it as an NFT

## Project Structure

```
src/
├── components/
│   ├── Chat/
│   │   ├── ChatView.tsx          # Main chat interface
│   │   ├── AgentSelector.tsx     # Agent selection UI
│   │   ├── AgentAudioPlayer.tsx  # Audio playback component
│   │   └── MintModal.tsx         # NFT minting modal
│   └── InputArea/
│       ├── InputArea.tsx         # Input component with recording
│       ├── RecordingUI.tsx       # Live audio visualizer
│       └── inputArea.css         # Styles
├── services/
│   └── api.ts                    # Backend API service
├── App.tsx                       # Root component
├── main.tsx                      # Entry point
└── index.css                     # Global styles
```

## API Integration

The frontend communicates with the backend using the API service (`src/services/api.ts`).

### Available API Functions

```typescript
// Fetch available agents
const agents = await getAgents();

// Send text to an agent
const response = await sendTextInput(agentId, userText);

// Send audio to an agent
const response = await sendAudioInput(agentId, audioFile);

// Get full audio URL
const fullUrl = getAudioUrl(audioPath);
```

### API Endpoints

- `GET /agents` - Fetch available agents
- `POST /input/text` - Send text input
- `POST /input/audio` - Send audio file
- `GET /public/audio/{filename}` - Serve audio files

For detailed API documentation, see [FRONTEND_INTEGRATION.md](../FRONTEND_INTEGRATION.md).

## Tech Stack

- **React 19** - UI framework
- **TypeScript** - Type safety
- **Framer Motion** - Animations
- **react-lottie-player** - Lottie animations
- **react-icons** - Icon library
- **Tailwind CSS** - Styling
- **Vite** - Build tool
- **Web Audio API** - Audio visualization
- **MediaRecorder API** - Audio recording

## Building for Production

```bash
npm run build
```

This creates optimized files in the `dist/` directory.

Preview the production build:

```bash
npm run preview
```

## Environment Variables

- `VITE_API_BASE_URL` - Backend API URL (default: `http://localhost:8000`)

For production, update this to your deployed API URL.

## Troubleshooting

### Backend Connection Issues

**Problem:** "Failed to load agents" or "Error processing request"

**Solution:**
1. Verify both backend services are running:
   - MCP Server on port 3000
   - MCP API on port 8000
2. Test backend health: `Invoke-RestMethod -Uri "http://localhost:8000/health"`
3. Check browser console for detailed errors
4. Verify `.env` file has correct API URL

### Microphone Access Issues

**Problem:** Recording doesn't work

**Solution:**
1. Grant microphone permissions in browser settings
2. Use HTTPS or localhost (required for mediaDevices API)
3. Check microphone is connected and working
4. Try a different browser if issues persist

### Audio Playback Issues

**Problem:** Can't hear AI responses

**Solution:**
1. Check browser supports MP3 format
2. Verify audio files exist: `http://localhost:8000/public/audio/`
3. Check browser volume settings
4. Look for errors in browser DevTools console

## Development Tips

### Hot Module Replacement

Vite provides instant HMR - changes to React components update immediately without losing state.

### TypeScript Strict Mode

The project uses strict TypeScript settings for better type safety. All API responses are properly typed.

### Component State Management

- **ChatView** - Manages conversation state and agent selection
- **InputArea** - Handles user input (text, recording, upload)
- **AgentSelector** - Fetches and displays available agents

## Related Documentation

- **[Frontend Integration Guide](../FRONTEND_INTEGRATION.md)** - Detailed integration documentation
- **[MCP API Documentation](../mcp-api/README.md)** - Backend API reference
- **[MCP Server Documentation](../mcp-server/README.md)** - AI agent system documentation
- **[Main Project README](../README.md)** - Full system overview

## Future Enhancements

- [ ] Add conversation history persistence
- [ ] Implement user authentication
- [ ] Add voice activity detection (VAD)
- [ ] Support for agent context/memory
- [ ] Real-time agent switching during conversation
- [ ] Message editing and deletion
- [ ] Dark/light theme toggle
- [ ] Export conversation as PDF/text

## Contributing

Contributions are welcome! Please ensure:
1. TypeScript types are maintained
2. Components are properly documented
3. API integration follows existing patterns
4. Code is formatted with Prettier

---

**Built with ❤️ for seamless AI interactions**

