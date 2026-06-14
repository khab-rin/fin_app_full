#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod sql_queries;
pub mod service;
pub mod parsers;
pub mod state;
pub mod commands;

use tauri_plugin_log::{Target, TargetKind};
use tauri::Manager;

use shared_lib::service::auth_service::client_state::TempInfo;

use crate::state::ClientState;

pub async fn run_lib() {

    let app = tauri::Builder::default()
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
            .level_for("rustls_platform_verifier", log::LevelFilter::Warn)
            .build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::auth::cmd_get_nick_names,
            commands::auth::cmd_is_state_active_fast,
            commands::auth::cmd_is_state_active_init,
            commands::auth::cmd_logout,
            commands::auth::cmd_make_ingoing_doc,
            commands::auth::cmd_register_user,
            commands::auth::cmd_session_by_nick,
            commands::auth::cmd_session_by_password,
            commands::auth::cmd_session_by_tel_call,
            commands::other::cmd_process_bank_statement,
            commands::other::cmd_validate_field,
        ]).build(tauri::generate_context!())
        .expect("error while building tauri application");
        
        let state = ClientState {
            config: state::Config::global(),
            app_handle: app.handle().clone(),
            session: tokio::sync::Mutex::new(None),
            temp_info: tokio::sync::Mutex::new(TempInfo {
                file_hash: None,
                phone: None,
                nick: None
            })
        };
        app.manage(state);

        app.run(|_app_handle, _event| {});
        
}
