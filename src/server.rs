//! Router assembly: /mcp (rmcp StreamableHttpService, attached by main.rs)
//! behind the auth layer; /health exempt from auth.

use axum::Json;
use axum::Router;
use axum::extract::State;
use axum::middleware::from_fn_with_state;
use axum::routing::get;
use serde_json::json;

use crate::auth::require_auth;
use crate::config::Config;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

async fn health(State(cfg): State<Config>) -> Json<serde_json::Value> {
    Json(json!({
        "ok": true,
        "version": VERSION,
        "s3": if cfg.s3.is_some() { "configured" } else { "not-configured" },
    }))
}

/// Build the full application router: `/health` open, `mcp` (an axum
/// router mounting the rmcp service at /mcp) behind the Bearer layer.
pub fn router(cfg: Config, mcp: Option<Router>) -> Router {
    let auth_keys = cfg.auth_keys.clone();
    let mut app = Router::new().route("/health", get(health)).with_state(cfg);
    if let Some(mcp) = mcp {
        let mcp = mcp.route_layer(from_fn_with_state(auth_keys, require_auth));
        app = app.merge(mcp);
    }
    app
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // oneshot

    fn test_config() -> Config {
        Config {
            host: "127.0.0.1".into(),
            port: 0,
            auth_keys: vec!["k".into()],
            s3: None,
            output_prefix: "excel-mcp-out/".into(),
            inline_max_bytes: 2048,
            max_file_bytes: 1024,
        }
    }

    #[tokio::test]
    async fn health_is_exempt_from_auth() {
        let app = router(test_config(), None);
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let v: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(v["ok"], json!(true));
        assert_eq!(v["s3"], json!("not-configured"));
        assert!(v["version"].is_string());
    }

    #[tokio::test]
    async fn mcp_requires_auth() {
        let mcp = Router::new().route("/mcp", get(|| async { "ok" }));
        let app = router(test_config(), Some(mcp));
        let resp = app
            .oneshot(Request::builder().uri("/mcp").body(Body::empty()).unwrap())
            .await
            .unwrap();
        // No token: 401 before reaching the stub route.
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn mcp_passes_with_token() {
        let mcp = Router::new().route("/mcp", get(|| async { "ok" }));
        let app = router(test_config(), Some(mcp));
        let resp = app
            .oneshot(
                Request::builder()
                    .uri("/mcp")
                    .header("authorization", "Bearer k")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn auth_disabled_when_no_keys() {
        let mcp = Router::new().route("/mcp", get(|| async { "ok" }));
        let app = router(
            Config {
                auth_keys: vec![],
                ..test_config()
            },
            Some(mcp),
        );
        let resp = app
            .oneshot(Request::builder().uri("/mcp").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
