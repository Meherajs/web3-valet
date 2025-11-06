//! Data models for the MCP server.
//!
//! This module contains all the data structures used throughout the server,
//! including JSON-RPC protocol types, agent definitions, Gemini API types,
//! and processing results.

use serde::{Deserialize, Serialize};

/// JSON-RPC 2.0 request structure.
///
/// Represents an incoming JSON-RPC request following the 2.0 specification.
///
/// # Type Parameters
///
/// * `T` - The type of the params field
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest<T> {
    /// Protocol version, must be "2.0"
    pub jsonrpc: String,
    /// Name of the method to call
    pub method: String,
    /// Optional parameters for the method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
    /// Request identifier for matching responses
    pub id: serde_json::Value,
}

/// JSON-RPC 2.0 response structure.
///
/// Represents an outgoing JSON-RPC response following the 2.0 specification.
///
/// # Type Parameters
///
/// * `T` - The type of the result field
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponse<T> {
    /// Protocol version, always "2.0"
    pub jsonrpc: String,
    /// Optional result data on success
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    /// Optional error object on failure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request identifier matching the original request
    pub id: serde_json::Value,
}

/// JSON-RPC 2.0 error object.
///
/// Represents an error in JSON-RPC response.
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Optional additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Information about an AI agent.
///
/// Represents a specialized AI agent with unique capabilities and system instructions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agent {
    /// Unique identifier for the agent
    pub id: String,
    /// Human-readable name
    pub name: String,
    /// Brief description of the agent's purpose
    pub description: String,
    /// List of capabilities (e.g., "text", "web3", "coding")
    pub capabilities: Vec<String>,
    /// AI model used by this agent (e.g., "gemini-2.0-flash-exp")
    pub model: String,
    /// System prompt that defines the agent's behavior
    pub system_prompt: String,
}

/// Result of the list_agents JSON-RPC method.
#[derive(Debug, Serialize, Deserialize)]
pub struct ListAgentsResult {
    /// List of all available agents
    pub agents: Vec<Agent>,
}

/// Parameters for the process_text JSON-RPC method.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessTextParams {
    /// ID of the agent to process the text
    pub agent_id: String,
    /// User's text input
    pub user_text: String,
    /// Optional conversation history for context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_history: Option<Vec<Message>>,
}

/// A message in the conversation history.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    /// Role of the message sender ("user" or "assistant")
    pub role: String,
    /// Content of the message
    pub content: String,
}

/// Result of the process_text JSON-RPC method.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessTextResult {
    /// ID of the agent that processed the text
    pub agent_id: String,
    /// Agent's text response
    pub reply_text: String,
    /// Metadata about the processing
    pub metadata: ProcessingMetadata,
}

/// Metadata about text processing.
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessingMetadata {
    /// AI model used
    pub model: String,
    /// Number of tokens consumed (if available)
    pub tokens_used: Option<u32>,
    /// Processing time in milliseconds
    pub processing_time_ms: u64,
    /// Confidence score (currently hardcoded)
    pub confidence: f64,
}

/// Request structure for Google Gemini API.
///
/// Represents a request to the Gemini generateContent endpoint.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiRequest {
    /// List of conversation contents (messages)
    pub contents: Vec<GeminiContent>,
    /// Optional system instruction to define agent behavior
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_instruction: Option<GeminiSystemInstruction>,
}

/// A single message/content in the Gemini conversation.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiContent {
    /// Role of the message sender ("user" or "model")
    pub role: String,
    /// Parts of the message (text, images, etc.)
    pub parts: Vec<GeminiPart>,
}

/// A part of a Gemini message (currently only text).
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiPart {
    /// Text content of the message part
    pub text: String,
}

/// System instruction for Gemini to define agent behavior.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiSystemInstruction {
    /// Parts containing the system instruction text
    pub parts: Vec<GeminiPart>,
}

/// Response structure from Google Gemini API.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiResponse {
    /// List of candidate responses (usually one)
    pub candidates: Vec<GeminiCandidate>,
    /// Optional metadata about token usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_metadata: Option<GeminiUsageMetadata>,
}

/// A single candidate response from Gemini.
#[derive(Debug, Serialize, Deserialize)]
pub struct GeminiCandidate {
    /// Content of the candidate response
    pub content: GeminiContent,
}

/// Metadata about token usage in the Gemini API call.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeminiUsageMetadata {
    /// Number of tokens in the prompt
    pub prompt_token_count: Option<u32>,
    /// Number of tokens in the generated response
    pub candidates_token_count: Option<u32>,
    /// Total tokens used (prompt + response)
    pub total_token_count: Option<u32>,
}
