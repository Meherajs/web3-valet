//! Request handlers for JSON-RPC methods.
//!
//! This module contains the HTTP request handlers that process incoming JSON-RPC
//! requests and route them to the appropriate functionality.

use crate::agents::{find_agent_by_id, get_agents};
use crate::gemini::process_with_gemini;
use crate::models::*;
use crate::AppState;
use axum::{extract::State, response::Json};
use std::sync::Arc;

/// Main JSON-RPC 2.0 request handler.
///
/// Routes incoming JSON-RPC requests to the appropriate handler based on the method name.
/// Validates the JSON-RPC version and returns appropriate error responses for invalid requests.
///
/// # Supported Methods
///
/// - `list_agents` - Lists all available agents
/// - `process_text` - Processes user text through an agent
///
/// # Arguments
///
/// * `state` - Shared application state
/// * `request` - JSON-RPC request with dynamic params
///
/// # Returns
///
/// A JSON-RPC response with either result or error
pub async fn handle_jsonrpc(
    State(state): State<Arc<AppState>>,
    Json(request): Json<JsonRpcRequest<serde_json::Value>>,
) -> Json<JsonRpcResponse<serde_json::Value>> {
    tracing::info!("Received JSON-RPC request: method={}", request.method);

    // Validate JSON-RPC version
    if request.jsonrpc != "2.0" {
        return Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32600,
                message: "Invalid Request: jsonrpc must be '2.0'".to_string(),
                data: None,
            }),
            id: request.id,
        });
    }

    // Route to the appropriate handler
    match request.method.as_str() {
        "list_agents" => handle_list_agents(request).await,
        "process_text" => handle_process_text(State(state), request).await,
        _ => Json(JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
                data: None,
            }),
            id: request.id,
        }),
    }
}

/// Handles the `list_agents` JSON-RPC method.
///
/// Returns a list of all available AI agents with their metadata.
///
/// # Arguments
///
/// * `request` - The JSON-RPC request
///
/// # Returns
///
/// A JSON-RPC response containing the list of agents
pub async fn handle_list_agents(
    request: JsonRpcRequest<serde_json::Value>,
) -> Json<JsonRpcResponse<serde_json::Value>> {
    let agents = get_agents();
    let result = ListAgentsResult { agents };

    Json(JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(serde_json::to_value(result).unwrap()),
        error: None,
        id: request.id,
    })
}

/// Handles the `process_text` JSON-RPC method.
///
/// Processes user text through a specified agent using Google's Gemini API.
/// This includes:
/// 1. Validating the agent ID
/// 2. Building the Gemini API request with system instructions and conversation history
/// 3. Calling the Gemini API
/// 4. Parsing the response and extracting the agent's reply
/// 5. Returning metadata about tokens used and processing time
///
/// # Arguments
///
/// * `state` - Shared application state containing the HTTP client and API key
/// * `request` - JSON-RPC request containing agent_id, user_text, and optional conversation history
///
/// # Returns
///
/// A JSON-RPC response containing the agent's reply and metadata, or an error
///
/// # Errors
///
/// Returns JSON-RPC errors for:
/// - Invalid parameters
/// - Unknown agent ID
/// - Gemini API failures
/// - Response parsing errors
pub async fn handle_process_text(
    State(state): State<Arc<AppState>>,
    request: JsonRpcRequest<serde_json::Value>,
) -> Json<JsonRpcResponse<serde_json::Value>> {
    // Parse the parameters
    let params: ProcessTextParams = match request.params {
        Some(ref p) => match serde_json::from_value(p.clone()) {
            Ok(params) => params,
            Err(e) => {
                return Json(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: format!("Invalid params: {}", e),
                        data: None,
                    }),
                    id: request.id,
                });
            }
        },
        None => {
            return Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: "Invalid params: agent_id and user_text are required".to_string(),
                    data: None,
                }),
                id: request.id,
            });
        }
    };

    // Find the requested agent
    let agent = match find_agent_by_id(&params.agent_id) {
        Some(a) => a,
        None => {
            return Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32602,
                    message: format!("Agent not found: {}", params.agent_id),
                    data: None,
                }),
                id: request.id,
            });
        }
    };

    // Start timing
    let start_time = std::time::Instant::now();

    // Process the text with Gemini
    let (reply_text, tokens_used) = match process_with_gemini(
        &state.http_client,
        &state.gemini_api_key,
        &agent,
        params.user_text,
        params.conversation_history,
    )
    .await
    {
        Ok(result) => result,
        Err(err_msg) => {
            tracing::error!("Gemini processing error: {}", err_msg);
            return Json(JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32603,
                    message: "Internal error: Gemini API processing failed".to_string(),
                    data: Some(serde_json::json!({ "details": err_msg })),
                }),
                id: request.id,
            });
        }
    };

    let processing_time = start_time.elapsed().as_millis() as u64;

    // Build the result
    let result = ProcessTextResult {
        agent_id: params.agent_id,
        reply_text,
        metadata: ProcessingMetadata {
            model: agent.model.clone(),
            tokens_used,
            processing_time_ms: processing_time,
            confidence: 0.95,
        },
    };

    Json(JsonRpcResponse {
        jsonrpc: "2.0".to_string(),
        result: Some(serde_json::to_value(result).unwrap()),
        error: None,
        id: request.id,
    })
}
