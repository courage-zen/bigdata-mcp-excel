mod auth;
mod config;
mod s3keys;
mod server;
mod storage;

fn main() -> anyhow::Result<()> {
    let cfg = config::Config::from_env();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();
    tracing::info!(host = %cfg.host, port = cfg.port, "excel-mcp starting");
    Ok(())
}
