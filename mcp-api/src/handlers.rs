//! Request handlers for all API endpoints.
//!
//! This module contains the handler functions for each API endpoint. All handlers
//! integrate with external services including the MCP server and ElevenLabs APIs.
//!
//! # ElevenLabs Integration
//!
//! - **TTS Model**: `eleven_multilingual_v2` (free tier compatible)
//! - **STT Model**: `scribe_v1` (only supported model)
//! - **Voice**: Rachel (ID: `21m00Tcm4TlvDq8ikWAM`)
//!
//! # Handler Functions
//!
//! - [`get_agents_list`] - Retrieves available agents from MCP server
//! - [`handle_text_input`] - Processes text input through MCP and generates audio via TTS
//! - [`handle_audio_input`] - Transcribes audio via STT, processes through MCP, and generates audio response

use crate::AppState;
use crate::models::{
    AgentInfo, AgentReplyResponse, InputTextRequest, JsonRpcRequest, JsonRpcResponse, 
    ListAgentsResult, ProcessTextResult,
};
use axum::{
    Json,
    extract::{Multipart, State},
    http::StatusCode,
};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

/// Retrieves a list of all available AI agents from the MCP server.
///
/// This handler makes a JSON-RPC call to the MCP server's `list_agents` method
/// and returns the list of agents to the client.
///
/// # Arguments
///
/// * `state` - Shared application state containing the HTTP client
///
/// # Returns
///
/// * `Ok(Json<Vec<AgentInfo>>)` - List of available agents on success
/// * `Err((StatusCode, Json<String>))` - Error message with appropriate status code
///
/// # Errors
///
/// Returns `INTERNAL_SERVER_ERROR` if:
/// - The MCP server is unreachable
/// - The MCP server returns an error response
/// - Response deserialization fails
///
/// # Environment Variables
///
/// Requires `MCP_SERVER_URL` to be set.
///
/// # Example Response
///
/// ```json
/// [
///   {
///     "id": "agent_001",
///     "name": "Assistant Agent",
///     "description": "A helpful AI assistant"
///   }
/// ]
/// ```
pub async fn get_agents_list(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<AgentInfo>>, (StatusCode, Json<String>)> {
    tracing::info!("Handler called: get_agents_list (REAL)");

    let mcp_url = std::env::var("MCP_SERVER_URL").expect("MCP_SERVER_URL not set");

    let rpc_request = JsonRpcRequest {
        jsonrpc: "2.0",
        method: "list_agents",
        params: serde_json::json!({}),
        id: 1,
    };

    let mcp_response = state
        .http_client
        .post(mcp_url)
        .json(&rpc_request)
        .send()
        .await;

    match mcp_response {
        Ok(response) => {
            if response.status().is_success() {
                let rpc_response: JsonRpcResponse<ListAgentsResult> = response.json().await.unwrap();

                tracing::info!("Got {} agents from MCP", rpc_response.result.agents.len());
                Ok(Json(rpc_response.result.agents))
            } else {
                tracing::error!("MCP /list_agents returned error: {:?}", response.status());
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Error from MCP service".to_string()),
                ))
            }
        }
        Err(e) => {
            tracing::error!("Failed to call MCP /list_agents: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to call MCP service".to_string()),
            ))
        }
    }
}

/// Processes text input through the MCP agent and generates an audio response.
///
/// This handler orchestrates a multi-step process:
/// 1. Sends user text to the MCP server for agent processing
/// 2. Receives the agent's text response
/// 3. Converts the response to audio using TTS API
/// 4. Returns both text and audio URL to the client
///
/// # Arguments
///
/// * `state` - Shared application state containing the HTTP client
/// * `payload` - JSON payload containing agent_id and user_text
///
/// # Returns
///
/// * `Ok((StatusCode::CREATED, Json<AgentReplyResponse>))` - Agent response with audio on success
/// * `Err((StatusCode, Json<String>))` - Error message with appropriate status code
///
/// # Errors
///
/// Returns `INTERNAL_SERVER_ERROR` if:
/// - The MCP server is unreachable or returns an error
/// - OpenAI TTS API fails or returns an error
/// - Audio file cannot be created or written
/// - Any response deserialization fails
///
/// # Environment Variables
///
/// Requires:
/// - `MCP_SERVER_URL` - URL of the MCP server
/// - `ELEVENLABS_API_KEY` - ElevenLabs API key
/// - `AUDIO_DIR` - Directory for storing audio files (optional, defaults to "public/audio")
///
/// # Request Example
///
/// ```json
/// {
///   "agent_id": "agent_001",
///   "user_text": "Hello, how are you?"
/// }
/// ```
///
/// # Response Example
///
/// ```json
/// {
///   "reply_text": "I'm doing great! How can I help you?",
///   "audio_url": "https://example.com/audio/response.mp3"
/// }
/// ```
pub async fn handle_text_input(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<InputTextRequest>,
) -> Result<(StatusCode, Json<AgentReplyResponse>), (StatusCode, Json<String>)> {
    tracing::info!(
        "Handler called: handle_text_input (REAL) for agent: {}",
        payload.agent_id
    );

    let mcp_url = std::env::var("MCP_SERVER_URL").expect("MCP_SERVER_URL not set");

    let rpc_request = JsonRpcRequest {
        jsonrpc: "2.0",
        method: "process_text",
        params: serde_json::json!({
            "agent_id": payload.agent_id,
            "user_text": payload.user_text,
        }),
        id: 1,
    };

    let mcp_response = state
        .http_client
        .post(mcp_url)
        .json(&rpc_request)
        .send()
        .await;

    let agent_reply_text: String = match mcp_response {
        Ok(response) => {
            if response.status().is_success() {
                let response_text = response.text().await.unwrap();
                tracing::info!("MCP response: {}", response_text);
                
                match serde_json::from_str::<JsonRpcResponse<ProcessTextResult>>(&response_text) {
                    Ok(rpc_response) => {
                        let reply = rpc_response.result.reply_text;
                        tracing::info!("Got agent reply from MCP: {}", reply);
                        reply
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse MCP response: {:?}", e);
                        tracing::error!("Raw response was: {}", response_text);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json("Error parsing MCP response".to_string()),
                        ));
                    }
                }
            } else {
                tracing::error!("MCP /process_text returned error: {:?}", response.status());
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Error from MCP service".to_string()),
                ));
            }
        }
        Err(e) => {
            tracing::error!("Failed to call MCP /process_text: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to call MCP service".to_string()),
            ));
        }
    };

    tracing::info!("Calling ElevenLabs TTS API for agent's reply");

    // ElevenLabs TTS API - using default voice "Rachel" (21m00Tcm4TlvDq8ikWAM)
    let tts_url = "https://api.elevenlabs.io/v1/text-to-speech/21m00Tcm4TlvDq8ikWAM";
    
    let tts_payload = serde_json::json!({
        "text": agent_reply_text,
        "model_id": "eleven_multilingual_v2",
        "output_format": "mp3_44100_128"
    });

    let tts_response = state
        .http_client
        .post(tts_url)
        .header("xi-api-key", &state.elevenlabs_api_key)
        .header("Content-Type", "application/json")
        .json(&tts_payload)
        .send()
        .await;

    let audio_bytes = match tts_response {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(e) => {
                        tracing::error!("Failed to read TTS audio bytes: {:?}", e);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json("Failed to read TTS audio".to_string()),
                        ));
                    }
                }
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                tracing::error!("ElevenLabs TTS API error {}: {}", status, error_text);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Error from TTS service: {}", error_text)),
                ));
            }
        }
        Err(e) => {
            tracing::error!("Failed to call ElevenLabs TTS API: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to call TTS service".to_string()),
            ));
        }
    };

    let filename = format!("{}.mp3", Uuid::new_v4());
    let filepath = PathBuf::from(&state.audio_dir).join(&filename);

    match File::create(&filepath).await {
        Ok(mut file) => {
            if let Err(e) = file.write_all(&audio_bytes).await {
                tracing::error!("Failed to write audio file: {:?}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Failed to save audio file".to_string()),
                ));
            }
        }
        Err(e) => {
            tracing::error!("Failed to create audio file: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to create audio file".to_string()),
            ));
        }
    }

    let audio_url = format!("/public/audio/{}", filename);
    tracing::info!("Audio saved to: {}", audio_url);

    let final_reply = AgentReplyResponse {
        reply_text: agent_reply_text,
        audio_url,
    };
    Ok((StatusCode::CREATED, Json(final_reply)))
}

/// Processes audio input through the complete STT → MCP → TTS pipeline.
///
/// This handler orchestrates the full audio processing pipeline:
/// 1. Receives audio file from client via multipart form data
/// 2. Transcribes audio to text using STT API
/// 3. Sends transcribed text to MCP agent for processing
/// 4. Converts agent's response to audio using TTS API
/// 5. Returns both text and audio URL to the client
///
/// # Arguments
///
/// * `state` - Shared application state containing the HTTP client
/// * `multipart` - Multipart form data containing audio file and agent_id
///
/// # Returns
///
/// * `Ok((StatusCode::CREATED, Json<AgentReplyResponse>))` - Agent response with audio on success
/// * `Err((StatusCode, Json<String>))` - Error message with appropriate status code
///
/// # Errors
///
/// Returns `BAD_REQUEST` if:
/// - Required form fields are missing (audio_file or agent_id)
///
/// Returns `INTERNAL_SERVER_ERROR` if:
/// - OpenAI Whisper API fails or returns an error
/// - The MCP server is unreachable or returns an error
/// - OpenAI TTS API fails or returns an error
/// - Audio file cannot be created or written
/// - Any response deserialization fails
///
/// # Environment Variables
///
/// Requires:
/// - `ELEVENLABS_API_KEY` - ElevenLabs API key
/// - `MCP_SERVER_URL` - URL of the MCP server
/// - `AUDIO_DIR` - Directory for storing audio files (optional, defaults to "public/audio")
///
/// # Request Format
///
/// Multipart form data with fields:
/// - `audio_file`: Audio file (MP3, WAV, or other supported formats)
/// - `agent_id`: String identifying the target agent
///
/// # Response Example
///
/// ```json
/// {
///   "reply_text": "I heard you say: Hello. Here's my response...",
///   "audio_url": "https://example.com/audio/response.mp3"
/// }
/// ```
pub async fn handle_audio_input(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<AgentReplyResponse>), (StatusCode, Json<String>)> {
    tracing::info!("Handler called: handle_audio_input (REAL)");

    let mut audio_data: Option<Vec<u8>> = None;
    let mut agent_id: Option<String> = None;
    let mut filename: Option<String> = None;
    
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unknown").to_string();
        if name == "audio_file" {
            filename = field.file_name().map(|s| s.to_string());
            audio_data = Some(field.bytes().await.unwrap().to_vec());
        } else if name == "agent_id" {
            agent_id = Some(field.text().await.unwrap());
        }
    }
    
    let (audio_data, agent_id) = match (audio_data, agent_id) {
        (Some(data), Some(id)) => (data, id),
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json("Missing 'audio_file' or 'agent_id'".to_string()),
            ));
        }
    };
    tracing::info!("Got agent_id: {} and audio file", agent_id);

    tracing::info!("Calling ElevenLabs Speech-to-Text API...");

    let original_filename = filename.unwrap_or_else(|| "audio.mp3".to_string());

    // ElevenLabs STT API
    let stt_url = "https://api.elevenlabs.io/v1/speech-to-text";
    
    let form = reqwest::multipart::Form::new()
        .part("audio", reqwest::multipart::Part::bytes(audio_data.clone())
            .file_name(original_filename)
            .mime_str("audio/mpeg").unwrap()
        )
        .text("model_id", "scribe_v1")
        .text("language_code", "eng")
        .text("tag_audio_events", "true");

    let stt_response = state
        .http_client
        .post(stt_url)
        .header("xi-api-key", &state.elevenlabs_api_key)
        .multipart(form)
        .send()
        .await;

    let user_text = match stt_response {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<serde_json::Value>().await {
                    Ok(json) => {
                        let text = json["text"].as_str().unwrap_or("").to_string();
                        tracing::info!("ElevenLabs transcribed text: {}", text);
                        text
                    }
                    Err(e) => {
                        tracing::error!("Failed to parse STT response: {:?}", e);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json("Failed to parse STT response".to_string()),
                        ));
                    }
                }
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                tracing::error!("ElevenLabs STT API error {}: {}", status, error_text);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Error from STT service: {}", error_text)),
                ));
            }
        }
        Err(e) => {
            tracing::error!("Failed to call ElevenLabs STT API: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to call STT service".to_string()),
            ));
        }
    };

    let mcp_url = std::env::var("MCP_SERVER_URL").expect("MCP_SERVER_URL not set");
    tracing::info!("Calling MCP /process_text...");

    let rpc_request = JsonRpcRequest {
        jsonrpc: "2.0",
        method: "process_text",
        params: serde_json::json!({
            "agent_id": agent_id,
            "user_text": user_text,
        }),
        id: 1,
    };

    let mcp_response = state
        .http_client
        .post(mcp_url)
        .json(&rpc_request)
        .send()
        .await;

    let agent_reply_text: String = match mcp_response {
        Ok(response) => {
            if response.status().is_success() {
                let rpc_response: JsonRpcResponse<ProcessTextResult> =
                    response.json().await.unwrap();
                let reply = rpc_response.result.reply_text;
                tracing::info!("Got agent reply from MCP: {}", reply);
                reply
            } else {
                tracing::error!("MCP /process_text returned error: {:?}", response.status());
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Error from MCP service".to_string()),
                ));
            }
        }
        Err(e) => {
            tracing::error!("Failed to call MCP /process_text: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to call MCP service".to_string()),
            ));
        }
    };

    tracing::info!("Calling ElevenLabs TTS API for agent's reply...");

    // ElevenLabs TTS API - using default voice "Rachel" (21m00Tcm4TlvDq8ikWAM)
    let tts_url = "https://api.elevenlabs.io/v1/text-to-speech/21m00Tcm4TlvDq8ikWAM";
    
    let tts_payload = serde_json::json!({
        "text": agent_reply_text,
        "model_id": "eleven_multilingual_v2",
        "output_format": "mp3_44100_128"
    });

    let tts_response = state
        .http_client
        .post(tts_url)
        .header("xi-api-key", &state.elevenlabs_api_key)
        .header("Content-Type", "application/json")
        .json(&tts_payload)
        .send()
        .await;

    let audio_bytes = match tts_response {
        Ok(response) => {
            if response.status().is_success() {
                match response.bytes().await {
                    Ok(bytes) => bytes.to_vec(),
                    Err(e) => {
                        tracing::error!("Failed to read TTS audio bytes: {:?}", e);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json("Failed to read TTS audio".to_string()),
                        ));
                    }
                }
            } else {
                let status = response.status();
                let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                tracing::error!("ElevenLabs TTS API error {}: {}", status, error_text);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(format!("Error from TTS service: {}", error_text)),
                ));
            }
        }
        Err(e) => {
            tracing::error!("Failed to call ElevenLabs TTS API: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to call TTS service".to_string()),
            ));
        }
    };

    let output_filename = format!("{}.mp3", Uuid::new_v4());
    let filepath = PathBuf::from(&state.audio_dir).join(&output_filename);

    match File::create(&filepath).await {
        Ok(mut file) => {
            if let Err(e) = file.write_all(&audio_bytes).await {
                tracing::error!("Failed to write audio file: {:?}", e);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json("Failed to save audio file".to_string()),
                ));
            }
        }
        Err(e) => {
            tracing::error!("Failed to create audio file: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Failed to create audio file".to_string()),
            ));
        }
    }

    let audio_url = format!("/public/audio/{}", output_filename);
    tracing::info!("Audio saved to: {}", audio_url);

    let final_reply = AgentReplyResponse {
        reply_text: agent_reply_text,
        audio_url,
    };
    Ok((StatusCode::CREATED, Json(final_reply)))
}
