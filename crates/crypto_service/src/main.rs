use axum::{routing::post, Router};
use std::sync::Arc;

pub mod check_sign;

use check_sign::verify_signature_handler;

pub struct AppState {
    pub cryptcp_path: String,
}

#[tokio::main]
async fn main() {

    // 1. Создаем логгер в файл
    // 1.1 создаватель ежедневных файлов
    let file_appender = tracing_appender::rolling::daily("logs", "back_api.log");

    // 1.2 делаем асинхронный записыватель
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 2. Делаем форматтер для консоли (райтер, цветной, имя файла, строки, модуля)
    let console_formatter = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true);

    // 3. Делаем черно белый форматтер для записи в файл (райтер, чб, имя файла, строки, модуля)
    let file_formatter = fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(true);

    // 4. Задаем фильтры для консоли и записи в файл с помощью строки
    let log_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "back_api=debug,tower_http=debug,axum::rejection=trace".into());

    // 5. Регистрируем все вместе
    tracing_subscriber::registry()
        .with(log_filter)
        .with(console_formatter)
        .with(file_formatter)
        .init();

    let cryptcp_path = std::env::var("CRYPTCP_PATH")
        .unwrap_or_else(|_| "/opt/cprocsp/bin/amd64/cryptcp".to_string());

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8081);

    tracing::info!("Crypto service is initializing with cryptcp path: {}", cryptcp_path);

    let state = Arc::new(AppState { cryptcp_path });

    let app = Router::new()
        .route("/verify", post(verify_signature_handler))
        .with_state(state);

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
