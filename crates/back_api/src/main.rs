mod db;
mod handlers;
mod config;
mod models;

use std::sync::Arc;
use axum::{routing::post, Router};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use shared_lib::service::api_routes::implements::ApiRoutes;
use shared_lib::static_data::init_re;

use crate::handlers::service::process::bank_statement::handler::auto_add_company_handler;

use crate::handlers::service::auth_service::handler::{
    register_step1_handler, make_session_by_tell_call_handler, register_user_by_crypto_handler, restore_by_password_handler, restore_by_token_handler
};

use crate::handlers::service::mchd::handler::register_mchd_hadler;

use crate::handlers::sql::handlers::get_person_by_inn_handler;


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
        .pretty()
        .with_ansi(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(true);

    // 3. Делаем черно белый форматтер для записи в файл (райтер, чб, имя файла, строки, модуля)
    let file_formatter = fmt::layer()
        .with_writer(non_blocking)
        .pretty()
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_target(true);

    // 4. Задаем фильтры для консоли и записи в файл с помощью строки
    let log_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "back_api=debug,tower_http=warn,axum=warn,sqlx=warn".into());

    // 5. Регистрируем все вместе
    tracing_subscriber::registry()
        .with(log_filter)
        .with(console_formatter)
        .with(file_formatter)
        .init();

    init_re();

    let base_url = &config::Config::global().data_base.database_url;

    let pool_fast = config::create_pool(
        base_url, 
        &config::Config::global().postgresql_options.fast_max_conn, 
        &config::Config::global().postgresql_options.fast_timeout
    ).await;

    let pool_long = config::create_pool(
        base_url, 
        &config::Config::global().postgresql_options.long_max_conn, 
        &config::Config::global().postgresql_options.long_timeout
    ).await;


    let state = Arc::new(config::BackApiState {
        pool_fast,
        pool_long,
        config: config::Config::global()
    });

    let app: Router = Router::new()
        .route(
            ApiRoutes::AutoAddCompany.get_path(),
            post(auto_add_company_handler)
        
        ).route(
            ApiRoutes::AuthRegister.get_path(), 
            post(register_user_by_crypto_handler)
        ).route(
            ApiRoutes::AuthRegisterStep1.get_path(),
            post(register_step1_handler)
        ).route(
            ApiRoutes::AuthRestorePassword.get_path(),
            post(restore_by_password_handler)
        ).route(
            ApiRoutes::AuthRestoreTellCall.get_path(),
            post(make_session_by_tell_call_handler)
        )
        .route(
            ApiRoutes::AuthRestoreToken.get_path(),
            post(restore_by_token_handler)
        ).route(
            ApiRoutes::MchdLend.get_path(),
            post(register_mchd_hadler)
        ).route(
            ApiRoutes::SqlPersonGetByInn.get_path(), 
            post(get_person_by_inn_handler)
        )

        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("msd");

    if let Ok(addr) = listener.local_addr() {
        tracing::info!("Сервер запущен на http://127.0.0.1:{}", addr.port());
    }

    axum::serve(listener, app).await.expect("msg");
}
