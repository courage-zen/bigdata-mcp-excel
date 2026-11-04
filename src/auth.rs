//! Bearer multi-key auth, mirroring markitdown-mcp's ASGI middleware as an
//! axum layer. /health stays exempt (it is routed outside the layer).

use axum::body::Body;
use axum::extract::State;
use axum::http::{Request, StatusCode, header::AUTHORIZATION};
use axum::middleware::Next;
use axum::response::{IntoResponse, Json, Response};
use serde_json::json;

/// Reject when the bearer token is absent or not in the configured key set.
/// An empty key set disables auth (loopback-only deployments).
pub async fn require_auth(
    State(keys): State<Vec<String>>,
    req: Request<Body>,
    next: Next,
) -> Response {
    if keys.is_empty() {
        return next.run(req).await;
    }
    let header = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let token = header.strip_prefix("Bearer ").map(str::trim).unwrap_or("");
    if token.is_empty() || !keys.iter().any(|k| k == token) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "error": "unauthorized",
                "message": "Invalid or missing authentication key"
            })),
        )
            .into_response();
    }
    next.run(req).await
}
