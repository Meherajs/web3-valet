use crate::models::{ErrorResponse, Metadata, MintRequest, MintResponse};
use axum::{http::StatusCode, response::IntoResponse, Json};

pub async fn mint(Json(payload): Json<MintRequest>) -> impl IntoResponse {
    tracing::info!(request = ?payload, "/mint called");

    // Build metadata
    let metadata = Metadata {
        name: payload.name.clone(),
        description: payload.description.clone(),
        asset_url: payload.asset_url.clone(),
    };

    // Upload metadata
    let upload = match crate::storage::upload_metadata(&metadata).await {
        Ok(u) => u,
        Err(e) => {
            tracing::error!(error = %e, "metadata upload failed");
            let body = ErrorResponse {
                error: format!("upload error: {}", e),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response();
        }
    };

    // Determine recipient
    let recipient = payload
        .recipient
        .clone()
        .unwrap_or_else(|| "default-recipient-address".to_string());

    // Mint token
    let mint = match crate::blockchain::mint_token(&upload.url, &recipient).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!(error = %e, "mint call failed");
            let body = ErrorResponse {
                error: format!("mint error: {}", e),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response();
        }
    };

    let resp = MintResponse {
        status: "success".to_string(),
        upload,
        mint,
    };

    tracing::info!(response = ?resp, "/mint completed");
    (StatusCode::OK, Json(resp)).into_response()
}
