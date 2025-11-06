# MCP Server (Model Context Protocol)

A JSON-RPC 2.0 server built with Rust and Axum that provides AI agent functionality using Google's Gemini API. This server manages multiple specialized AI agents and processes user requests with context-aware responses.

## 📚 Table of Contents

- [What It Does](#-what-it-does)
- [Architecture](#-architecture)
- [Features](#-features)
- [AI Agents](#-ai-agents)
- [Prerequisites](#-prerequisites)
- [Quick Start](#-quick-start)
- [JSON-RPC Methods](#-json-rpc-methods)
- [Testing](#-testing-examples)
- [Configuration](#-configuration)
- [Code Documentation](#-code-documentation)
- [Development](#-development)
- [Troubleshooting](#-troubleshooting)

## 🎯 What It Does

The MCP Server is the AI brain of the system:

1. **Manages AI Agents**: Four specialized agents with different expertise
2. **Processes Requests**: Handles JSON-RPC 2.0 protocol for agent interactions
3. **Interfaces with Gemini**: Uses Google's Gemini 2.0 Flash model for AI responses
4. **Provides Context**: Each agent has specialized system instructions
5. **Returns Metadata**: Includes processing stats like tokens used and timing

## 🏗️ Architecture

```
┌─────────────────┐
│   MCP API       │
│  (Port 8000)    │
└────────┬────────┘
         │ JSON-RPC 2.0
         │ {"method": "process_text", "params": {...}}
         ▼
┌──────────────────────────────────────────┐
│       MCP Server (Port 3000)             │
│  ┌────────────────────────────────────┐  │
│  │  Agent Router                      │  │
│  │  - General Assistant (agent_001)   │  │
│  │  - Web3 Expert (agent_002)         │  │
│  │  - Voice Assistant (agent_003)     │  │
│  │  - Code Helper (agent_004)         │  │
│  └────────────────────────────────────┘  │
└────────┬─────────────────────────────────┘
         │ HTTPS
         │ POST with system_instruction
         ▼
┌──────────────────┐
│   Google Gemini  │
│  gemini-2.0-flash│
│    -exp (v1beta) │
└──────────────────┘
```

## ✨ Features

- **JSON-RPC 2.0 Protocol**: Standard protocol for remote procedure calls
- **Multiple AI Agents**: Four specialized agents with unique capabilities
- **Gemini Integration**: Powered by Google's latest Gemini 2.0 model
- **System Instructions**: Each agent has tailored behavior and expertise
- **Metadata Tracking**: Returns tokens used, processing time, and confidence
- **Error Handling**: Comprehensive error responses with details
- **Async Performance**: High-throughput request handling

## 🤖 Available Agents

### Agent 001 - General Assistant
**ID:** `agent_001`  
**Expertise:** General-purpose assistance  
**Best for:** General questions, everyday tasks, broad knowledge queries

### Agent 002 - Web3 Expert
**ID:** `agent_002`  
**Expertise:** Blockchain, cryptocurrency, DeFi, NFTs, smart contracts  
**Best for:** Web3 technology questions, blockchain explanations, crypto advice

### Agent 003 - Voice Assistant
**ID:** `agent_003`  
**Expertise:** Natural conversation, voice interactions  
**Best for:** Conversational responses, audio-based interactions

### Agent 004 - Code Helper
**ID:** `agent_004`  
**Expertise:** Programming, debugging, code review, best practices  
**Best for:** Code questions, bug fixes, architecture advice

## 📋 Prerequisites

- **Rust** 1.70 or higher ([Install Rust](https://rustup.rs/))
- **Google Gemini API Key** ([Get your key](https://aistudio.google.com/app/apikey))

## 🚀 Quick Start

### 1. Get Your Gemini API Key

1. Go to [Google AI Studio](https://aistudio.google.com/app/apikey)
2. Click **"Get API Key"** or **"Create API Key"**
3. Copy the generated API key

### 2. Configure Environment

Create a `.env` file in the `mcp-server` directory:

```env
# Google Gemini API Configuration
GEMINI_API_KEY=your_gemini_api_key_here

# Logging Configuration
RUST_LOG=info
```

### 3. Build and Run

```powershell
# Navigate to the directory
cd mcp-server

# Build the project (release mode for production)
cargo build --release

# Run the server
cargo run --release
```

The server will start on `http://127.0.0.1:3000`

### 4. Verify It's Running

```powershell
# List agents
Invoke-RestMethod -Uri "http://localhost:3000" `
  -Method POST `
  -ContentType "application/json" `
  -Body '{
    "jsonrpc": "2.0",
    "method": "list_agents",
    "params": {},
    "id": 1
  }'
```

## 📡 JSON-RPC Methods

### Method: `list_agents`

List all available AI agents.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "list_agents",
  "params": {},
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "agents": [
      {
        "id": "agent_001",
        "name": "General Assistant",
        "description": "A helpful general-purpose AI assistant",
        "model": "gemini-2.0-flash-exp"
      },
      {
        "id": "agent_002",
        "name": "Web3 Expert",
        "description": "Specialized in blockchain, cryptocurrency, and Web3 technologies",
        "model": "gemini-2.0-flash-exp"
      },
      {
        "id": "agent_003",
        "name": "Voice Assistant",
        "description": "Optimized for voice interactions and conversational responses",
        "model": "gemini-2.0-flash-exp"
      },
      {
        "id": "agent_004",
        "name": "Code Helper",
        "description": "Expert in programming, debugging, and code review",
        "model": "gemini-2.0-flash-exp"
      }
    ]
  },
  "id": 1
}
```

---

### Method: `process_text`

Send text to an AI agent and get a response.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "method": "process_text",
  "params": {
    "agent_id": "agent_002",
    "user_text": "What is a blockchain?"
  },
  "id": 1
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "result": {
    "agent_id": "agent_002",
    "reply_text": "A blockchain is a distributed, immutable ledger that records transactions across multiple computers...",
    "metadata": {
      "model": "gemini-2.0-flash-exp",
      "tokens_used": 245,
      "processing_time_ms": 1523,
      "confidence": 0.95
    }
  },
  "id": 1
}
```

---

### Error Response

When an error occurs:

```json
{
  "jsonrpc": "2.0",
  "error": {
    "code": -32600,
    "message": "Invalid agent_id: agent_999"
  },
  "id": 1
}
```

## 🔧 Testing with PowerShell

### Test Agent Listing

```powershell
$body = @{
    jsonrpc = "2.0"
    method = "list_agents"
    params = @{}
    id = 1
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000" `
  -Method POST `
  -ContentType "application/json" `
  -Body $body
```

### Test Text Processing

```powershell
$body = @{
    jsonrpc = "2.0"
    method = "process_text"
    params = @{
        agent_id = "agent_002"
        user_text = "Explain smart contracts"
    }
    id = 1
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000" `
  -Method POST `
  -ContentType "application/json" `
  -Body $body
```

### Test All Agents

```powershell
$agents = @("agent_001", "agent_002", "agent_003", "agent_004")
$question = "What can you help me with?"

foreach ($agent in $agents) {
    Write-Host "`n=== Testing $agent ===" -ForegroundColor Cyan
    
    $body = @{
        jsonrpc = "2.0"
        method = "process_text"
        params = @{
            agent_id = $agent
            user_text = $question
        }
        id = 1
    } | ConvertTo-Json
    
    $response = Invoke-RestMethod -Uri "http://localhost:3000" `
      -Method POST `
      -ContentType "application/json" `
      -Body $body
    
    Write-Host "Response: $($response.result.reply_text.Substring(0, 100))..." -ForegroundColor Green
    Write-Host "Tokens: $($response.result.metadata.tokens_used), Time: $($response.result.metadata.processing_time_ms)ms" -ForegroundColor Yellow
}
```

## 📊 Project Structure

```
mcp-server/
├── src/
│   └── main.rs         # All server code (routes, handlers, Gemini API)
├── .env                # Environment configuration
├── Cargo.toml          # Rust dependencies
└── README.md           # This file
```

## 🔧 Configuration Details

### Gemini API Settings

- **Endpoint:** `https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash-exp:generateContent`
- **Authentication:** API key via `x-goog-api-key` header
- **Model:** `gemini-2.0-flash-exp` (latest Gemini model)
- **API Version:** `v1beta` (required for system_instruction support)

### System Instructions

Each agent has a unique system instruction that defines its behavior:

```rust
// Example: Web3 Expert
"You are a Web3 and blockchain technology expert. Provide detailed, 
accurate information about blockchain, cryptocurrency, DeFi, NFTs, 
smart contracts, and related technologies. Use technical terms when 
appropriate but explain them clearly."
```

## 🐛 Troubleshooting

### Server won't start

**Error:** `GEMINI_API_KEY must be set in .env file`
- **Solution:** Create a `.env` file with your Gemini API key

**Error:** `Failed to bind to address`
- **Solution:** Port 3000 is in use. Kill the existing process or change the port in `main.rs`

### API Errors

**Error:** `400 Bad Request - Invalid JSON payload`
- **Solution:** Check your request format matches JSON-RPC 2.0 spec
- **Verify:** `jsonrpc: "2.0"`, `method`, `params`, and `id` fields are present

**Error:** `401 Unauthorized`
- **Solution:** Verify your Gemini API key is correct
- **Check:** Key is active at [Google AI Studio](https://aistudio.google.com/)

**Error:** `404 Not Found - model not found`
- **Solution:** Make sure you're using the v1beta endpoint
- **Verify:** URL includes `/v1beta/` not `/v1/`

**Error:** `Invalid agent_id`
- **Solution:** Use one of: `agent_001`, `agent_002`, `agent_003`, `agent_004`

### Performance Issues

**Slow responses:**
- **Normal:** First request may take 2-3 seconds (cold start)
- **Expected:** Subsequent requests typically 1-2 seconds
- **Solution:** If consistently slow, check your internet connection

**High token usage:**
- **Solution:** Shorter user_text = fewer tokens consumed
- **Monitor:** Check [Google AI Studio](https://aistudio.google.com/) for quota

## 📈 Response Metadata

Each response includes useful metadata:

```json
"metadata": {
  "model": "gemini-2.0-flash-exp",
  "tokens_used": 245,            // Total tokens (prompt + completion)
  "processing_time_ms": 1523,    // Server processing time
  "confidence": 0.95             // Currently hardcoded, future enhancement
}
```

## 🔗 Dependencies

- **axum** 0.8 - High-performance web framework
- **tokio** 1.0 - Async runtime for concurrent operations
- **reqwest** 0.12 - HTTP client for Gemini API communication
- **serde** / **serde_json** - JSON serialization/deserialization
- **tower-http** 0.6 - CORS middleware support
- **tracing** / **tracing-subscriber** - Structured logging
- **dotenv** 0.15 - Environment variable management

### Why These Dependencies?

- **Axum**: Type-safe, fast, and ergonomic web framework built on Tower
- **Tokio**: Industry-standard async runtime with excellent performance
- **Reqwest**: Feature-rich HTTP client with JSON support
- **Tower**: Modular middleware ecosystem for cross-cutting concerns
- **Tracing**: Production-ready structured logging that scales


## 📝 Development

### Project Structure

After modularization, the code is organized as follows:

```
mcp-server/src/
├── main.rs         # Server initialization and startup
├── models.rs       # All data structures (JSON-RPC, Gemini API, agents)
├── agents.rs       # Agent definitions and management
├── gemini.rs       # Gemini API client and communication
└── handlers.rs     # JSON-RPC request handlers
```

### Building for Development

```powershell
cargo build
cargo run
```

### Running with Debug Logs

```powershell
$env:RUST_LOG="mcp_server=debug"
cargo run
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

### Adding a New Agent

1. Open `src/agents.rs`
2. Add a new agent to the `get_agents()` function:

```rust
Agent {
    id: "agent_005".to_string(),
    name: "Your Agent Name".to_string(),
    description: "What this agent does".to_string(),
    capabilities: vec!["capability1".to_string(), "capability2".to_string()],
    model: "gemini-2.0-flash-exp".to_string(),
    system_prompt: "Your custom system instruction here...".to_string(),
}
```

3. Rebuild and test:
```powershell
cargo build
cargo run
```

---

## 📖 Code Documentation

This project includes comprehensive inline documentation for all modules, functions, and types. You can view the full documentation in your browser:

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

- **Main Module** (`src/main.rs`)
  - Server architecture overview
  - Startup process and initialization
  - Application state management

- **Models Module** (`src/models.rs`)
  - `JsonRpcRequest` / `JsonRpcResponse` - JSON-RPC 2.0 protocol types
  - `Agent` - Agent configuration and metadata
  - `GeminiRequest` / `GeminiResponse` - Gemini API types
  - `ProcessTextParams` / `ProcessTextResult` - Method parameters and results
  - All struct fields documented with descriptions

- **Agents Module** (`src/agents.rs`)
  - `get_agents()` - Returns all available agents
  - `find_agent_by_id()` - Lookup agent by ID
  - Complete agent definitions (4 specialized agents)

- **Gemini Module** (`src/gemini.rs`)
  - `process_with_gemini()` - Main Gemini API communication function
  - Request building, HTTP handling, response parsing
  - Error handling and token usage tracking

- **Handlers Module** (`src/handlers.rs`)
  - `handle_jsonrpc()` - Main JSON-RPC router
  - `handle_list_agents()` - List all agents
  - `handle_process_text()` - Process text through an agent
  - Complete parameter and error documentation

### Inline Documentation Features

✅ **Module-level documentation** - Overview of each file's purpose  
✅ **Function documentation** - Parameters, returns, errors, examples  
✅ **Struct documentation** - Field descriptions and usage  
✅ **Example code** - Usage examples for public APIs  
✅ **Cross-references** - Links between related items  
✅ **Error documentation** - What errors can occur and why  

### Reading the Documentation

1. **Navigate by module** - Start with the crate root and drill down into modules
2. **Search functionality** - Use the search bar to find specific items quickly
3. **Type signatures** - Click on types to see their complete definitions
4. **Source code links** - View the actual implementation code
5. **Example sections** - Copy-paste ready code examples

---

## 🌟 Example Flow

1. **MCP API receives** user text: "What is DeFi?"
2. **MCP API sends JSON-RPC** to MCP Server with `agent_id: "agent_002"`
3. **MCP Server routes** to Web3 Expert agent
4. **System instruction applied**: "You are a Web3 expert..."
5. **Gemini API called** with user text + system instruction
6. **Gemini generates** detailed DeFi explanation
7. **MCP Server extracts** response text and metadata
8. **JSON-RPC response** returned to MCP API
9. **MCP API converts** to speech and sends to client

## 🔐 Security Notes

- ⚠️ **Never commit `.env`** with your API key to version control
- ✅ Add `.env` to `.gitignore`
- ✅ Use environment variables in production
- ✅ Rotate API keys periodically

---

## 📚 Related Documentation

- **[MCP API Documentation](../mcp-api/README.md)** - REST API frontend for this server
- **[Frontend Integration Guide](../FRONTEND_INTEGRATION.md)** - Complete guide for frontend developers
- **[Code Documentation](#-code-documentation)** - View inline Rust documentation with `cargo doc --open`

---

## 📄 License

This project is part of the web3-valet system.

## 🤝 Related Projects

- **mcp-api** - REST API frontend for this server
- **web3-minting** - NFT minting functionality

---

**Need help?** Enable debug logging with `RUST_LOG=debug` to see detailed request/response information.
