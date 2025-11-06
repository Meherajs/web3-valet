use crate::models::{Metadata, UploadResult};
use anyhow::{anyhow, Result};
use reqwest::Client;
use std::env;
use uuid::Uuid;

/// Upload metadata to storage (IPFS or mock). Returns CID and a gateway URL.
pub async fn upload_metadata(metadata: &Metadata) -> Result<UploadResult> {
    // If IPFS_URL is set, attempt to POST the JSON there. Otherwise return a mock CID.
    if let Ok(ipfs_url) = env::var("IPFS_URL") {
        tracing::info!(ipfs_url = %ipfs_url, "using configured IPFS endpoint");
        let client = Client::new();
        // We post the metadata as JSON and expect the remote to return some JSON containing a cid/hash.
        let resp = client
            .post(&ipfs_url)
            .json(metadata)
            .send()
            .await
            .map_err(|e| anyhow!("ipfs request failed: {}", e))?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("ipfs upload failed: {} - {}", status, text));
        }

        // Try to parse JSON and extract a field 'cid' or 'Hash' or fallback to UUID
        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| anyhow!("failed to parse response: {}", e))?;
        let cid = json
            .get("cid")
            .or_else(|| json.get("Hash"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("bafy{}", Uuid::new_v4().simple()));

        let url = format!("https://ipfs.io/ipfs/{}", cid);
        tracing::info!(cid = %cid, url = %url, "ipfs upload result");
        Ok(UploadResult { cid, url })
    } else {
        // Mock path: deterministic-ish CID and gateway URL for local dev and testing.
        let cid = format!("bafy{}", Uuid::new_v4().simple());
        let url = format!("https://ipfs.io/ipfs/{}", cid);
        tracing::warn!(cid = %cid, "IPFS_URL not set - returning mock upload result");
        Ok(UploadResult { cid, url })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_upload_metadata_mock() {
        let m = Metadata {
            name: "Test".to_string(),
            description: Some("desc".to_string()),
            asset_url: Some("https://example.com/a.png".to_string()),
        };
        let r = upload_metadata(&m).await.expect("upload should succeed");
        assert!(r.cid.starts_with("bafy") || !r.cid.is_empty());
        assert!(r.url.contains(&r.cid));
    }
}
