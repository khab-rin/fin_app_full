mod db;
mod handlers;
mod config;
mod models;

use std::sync::Arc;
use sqlx::PgPool;
use axum::{routing::post, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::static_data::init_re;

use crate::handlers::service::process::bank_statement::handler::auto_add_company_handler;
use crate::handlers::service::auth_service::handler::{
    restore_by_token_handler,
    restore_by_password_handler,
    make_session_token_by_tell_call_handler,
    register_user_by_crypto_handler
};


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

    init_re();

    let base_url = &config::Config::global().data_base.database_url;

    let mut attemps = 0;

    let pool = loop {
        attemps += 1;
        match PgPool::connect(base_url).await {
            Ok(p) => {break p;}
            Err(er) => { 
                tracing::warn!("Попытка подключения к БД №{} не удалась", attemps);
                if attemps == 5 { 
                    tracing::error!("ФАТАЛЬНАЯ ОШИБКА: Не удалось подключиться к PostgreSQL после 5 попыток. Завершение работы.");

                    panic!("Database connection failed: {}", er);
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    };

    let state = Arc::new(config::BackApiState {
        pool,
        config: config::Config::global()
    });

    let app: Router = Router::new()
        .route(
            ApiRoutes::AutoAddCompany.get_path(),
            post(auto_add_company_handler)
        )
        .route(
            ApiRoutes::AuthRestoreToken.get_path(),
            post(restore_by_token_handler)
        ).route(
            ApiRoutes::AuthRestorePassword.get_path(),
            post(restore_by_password_handler)
        ).route(
            ApiRoutes::AuthMakeTokenTelCall.get_path(),
            post(make_session_token_by_tell_call_handler)
        ).route(
            ApiRoutes::AuthRegister.get_path(), 
            post(register_user_by_crypto_handler)
        )

        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("msd");

    tracing::info!("Сервер запущен на http://0.0.0");

    axum::serve(listener, app).await.expect("msg");
}
