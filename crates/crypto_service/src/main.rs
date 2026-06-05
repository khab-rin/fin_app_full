use axum::{routing::post, Router};
use std::sync::Arc;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use shared_lib::service::api_routes::implements::CryptoApiRoutes;

pub mod person_snils;

use person_snils::checker::verify_signature_handler;

pub struct AppState {
    pub cryptcp_path: String,
}

#[tokio::main]
async fn main() {

    let file_appender = tracing_appender::rolling::daily("logs", "crypto_servise.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let console_formatter = fmt::layer()
        .with_writer(std::io::stdout)
        .pretty()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true);

    let file_formatter = fmt::layer()
        .with_writer(non_blocking)
        .pretty()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(true);

    let log_filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| {
        "crypto_service=debug,crypto_service_bin=debug,tower_http=debug,axum::rejection=trace".into()
    });

    tracing_subscriber::registry()
        .with(log_filter)
        .with(console_formatter)
        .with(file_formatter)
        .init();
    
    let cryptcp_path = std::env::var("CRYPTCP_PATH")
        .unwrap_or_else(|_| "/opt/cprocsp/bin/amd64/cryptcp".to_string());

    let state = Arc::new(AppState { cryptcp_path });

    let app = Router::new()
        .route(
            CryptoApiRoutes::CryptoVerifyPerson.get_path(), 
            post(verify_signature_handler))
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081);

    let addr = format!("0.0.0.0:{}", port);

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(err) => {
            tracing::error!("Failed to bind to {}: {:?}", addr, err);
            return;
        }
    };

    tracing::info!("Crypto service successfully started and listening on {}", addr);
    
    
    if let Err(err) = axum::serve(listener, app).await {
        tracing::error!("Server error: {:?}", err);
    }

}
