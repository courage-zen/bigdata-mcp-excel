mod auth;
mod config;
mod engine;
mod s3keys;
mod server;
mod storage;
mod tools;
mod util;

use rmcp::transport::streamable_http_server::{
    StreamableHttpServerConfig, StreamableHttpService, session::local::LocalSessionManager,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::Config::from_env();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let s3_status = if cfg.s3.is_some() {
        "configured"
    } else {
        "not-configured"
    };
    tracing::info!(host = %cfg.host, port = cfg.port, s3 = s3_status, "excel-mcp starting");
    let cfg_for_sessions = cfg.clone();

    let service = StreamableHttpService::new(
        move || Ok(tools::ExcelMcp::new(cfg_for_sessions.clone())),
        LocalSessionManager::default().into(),
        StreamableHttpServerConfig::default(),
    );
    let mcp_router = axum::Router::new().nest_service("/mcp", service);
    let app = server::router(cfg, Some(mcp_router));

    let addr = format!(
        "{}:{}",
        app_addr_host(),
        std::env::var("MCP_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080)
    );
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}

fn app_addr_host() -> String {
    std::env::var("MCP_HOST").unwrap_or_else(|_| "0.0.0.0".into())
}
