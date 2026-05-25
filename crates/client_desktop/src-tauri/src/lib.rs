#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod sql_queries;
pub mod config;
pub mod service;
pub mod parsers;
pub mod commands;

use std::sync::OnceLock;
use sqlx::{SqlitePool};
use tauri_plugin_log::{Target, TargetKind};

pub static POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn run_lib() {

    let base_url = &config::Config::global().database;

    #[cfg(debug_assertions)]
    {
        if let Some(path) = base_url.strip_prefix("sqlite:").and_then(|s| s.split('?').next()) {
            let _ = std::fs::remove_file(path); 
            println!("Database cleared");
        }
    }

    let pool = sqlx::SqlitePool::connect(base_url)
        .await
        .expect("FILE_SYSTEM_ACCESS_ERROR!!!");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("MIGRATION_ERROR!!");

    POOL.set(pool).expect("STATIC_MEMORY_WRONG!");

    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .format(|out, message, record| {
                out.finish(format_args!(
                    "[{file}:{line}] {level}: {message}",
                    file = record.file().unwrap_or("unknown"),
                    line = record.line().unwrap_or(0),
                    level = record.level(),
                    message = message
                ))
            })
            .targets([
                Target::new(TargetKind::Stdout),
                Target::new(TargetKind::LogDir { file_name: Some("app_logs".to_string()) }),
                Target::new(TargetKind::Webview),
            ])
            .level(log::LevelFilter::Info)
            .build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            commands::cmd_process_bank_statement
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
