//! Data models and types for the MCP API server.
//!
//! This module defines all the request and response types used in the API,
//! including JSON-RPC protocol structures for communication with the MCP server.

use serde::{Deserialize, Serialize};

/// Information about an AI agent available in the system.
///
/// This struct represents an agent that can process user requests and generate responses.
/// Agents are retrieved from the MCP server via the `GET /agents` endpoint.
///
/// # Fields
///
/// * `id` - Unique identifier for the agent
/// * `name` - Human-readable name of the agent
/// * `description` - Brief description of the agent's purpose and capabilities
#[derive(Serialize, Deserialize, Clone)]
pub struct AgentInfo {
    pub id: String,
    pub name: String,
    pub description: String,
}

/// Response from the MCP server's list_agents method.
///
/// This struct wraps the array of agents returned by the MCP server.
///
/// # Fields
///
/// * `agents` - List of available agents
#[derive(Serialize, Deserialize)]
pub struct ListAgentsResult {
    pub agents: Vec<AgentInfo>,
}

/// Response from the MCP server's process_text method.
///
/// This struct represents the result of processing user text through an agent.
///
/// # Fields
///
/// * `agent_id` - ID of the agent that processed the text
/// * `reply_text` - The agent's text response
/// * `metadata` - Additional metadata about the processing
#[derive(Serialize, Deserialize)]
pub struct ProcessTextResult {
    pub agent_id: String,
    pub reply_text: String,
    pub metadata: ProcessingMetadata,
}

/// Metadata about the agent's text processing.
///
/// # Fields
///
/// * `model` - The AI model used
/// * `tokens_used` - Number of tokens consumed (if available)
/// * `processing_time_ms` - Time taken to process in milliseconds
/// * `confidence` - Confidence score of the response
#[derive(Serialize, Deserialize)]
pub struct ProcessingMetadata {
    pub model: String,
    pub tokens_used: Option<u32>,
    pub processing_time_ms: u64,
    pub confidence: f64,
}

/// Request payload for text input from the user.
///
/// This struct is deserialized from JSON when clients send text to the
/// `POST /input/text` endpoint.
///
/// # Fields
///
/// * `agent_id` - ID of the agent that should process the text
/// * `user_text` - The actual text input from the user
///
/// # Example
///
/// ```json
/// {
///   "agent_id": "agent_001",
///   "user_text": "Hello, how are you?"
/// }
/// ```
#[derive(Deserialize)]
pub struct InputTextRequest {
    pub agent_id: String,
    pub user_text: String,
}

/// Response containing the agent's reply in both text and audio formats.
///
/// This struct is returned by both `POST /input/text` and `POST /input/audio`
/// endpoints, providing the agent's response as text and a URL to the audio version.
///
/// # Fields
///
/// * `reply_text` - The agent's text response
/// * `audio_url` - URL to the audio file containing the spoken response
///
/// # Example
///
/// ```json
/// {
///   "reply_text": "I'm doing great! How can I help you?",
///   "audio_url": "https://example.com/audio/response.mp3"
/// }
/// ```
#[derive(Serialize)]
pub struct AgentReplyResponse {
    pub reply_text: String,
    pub audio_url: String,
}

/// Generic JSON-RPC 2.0 request structure.
///
/// This struct is used to construct requests to the MCP server following the
/// JSON-RPC 2.0 specification. The params field is generic to allow different
/// parameter types for different RPC methods.
///
/// # Type Parameters
///
/// * `T` - The type of the params field, typically a JSON object
///
/// # Fields
///
/// * `jsonrpc` - Protocol version, always "2.0"
/// * `method` - Name of the RPC method to call
/// * `params` - Parameters for the method call
/// * `id` - Request identifier for matching responses
///
/// # Example
///
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "method": "call_agent",
///   "params": {
///     "agent_id": "agent_001",
///     "text": "Hello"
///   },
///   "id": 1
/// }
/// ```
#[derive(Serialize)]
pub struct JsonRpcRequest<T> {
    pub jsonrpc: &'static str,
    pub method: &'static str,
    pub params: T,
    pub id: u32,
}

/// Generic JSON-RPC 2.0 response structure.
///
/// This struct represents responses received from the MCP server following the
/// JSON-RPC 2.0 specification. The result field is generic to accommodate
/// different response types for different RPC methods.
///
/// # Type Parameters
///
/// * `T` - The type of the result field
///
/// # Fields
///
/// * `jsonrpc` - Protocol version, always "2.0"
/// * `result` - The result data from the RPC call
/// * `id` - Request identifier matching the original request
///
/// # Note
///
/// This implementation does not include error handling fields. A production
/// implementation should include a `JsonRpcError` variant.
///
/// # Example
///
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "result": {
///     "reply": "Agent response text"
///   },
///   "id": 1
/// }
/// ```
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct JsonRpcResponse<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: u32,
}
