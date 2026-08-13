#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub mod commands;
pub mod service;

use tauri_plugin_log::{Target, TargetKind};
use tauri::Manager;

use shared_lib::service::auth_service::client_state::*;


pub fn run_lib() {

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
            .level_for("shared_lib::client::operation::statement_parser::make_operations", log::LevelFilter::Off)
            .build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            commands::auth::cmd_get_nick_names,
            commands::auth::cmd_get_device_id,
            commands::auth::cmd_is_state_active_fast,
            commands::auth::cmd_is_state_active_init,
            commands::auth::cmd_logout,
            commands::auth::cmd_register_step1,
            commands::auth::cmd_register_step2,
            commands::auth::cmd_session_by_nick,
            commands::auth::cmd_session_by_password,
            commands::auth::cmd_session_by_tel_call,
            commands::mchd::cmd_get_all_btb_powers,
            commands::mchd::cmd_get_all_fns_powers,
            commands::mchd::cmd_get_all_home_powers,
            commands::mchd::cmd_get_power_info,
            commands::mchd::cmd_lend_mchd,
            commands::mchd::cmd_make_xml_doc_files,
            commands::mchd::cmd_show_powers,
            commands::operation::cmd_load_bank_statement,
            commands::other::cmd_validate_field,
            commands::sql_queries::cmd_add_comp_bank_acc,
            commands::sql_queries::cmd_get_comp_bank_accs,
            
        ]).build(tauri::generate_context!())
        .expect("error while building tauri application");
        
        let state = ClientState {
            config: Config::global(),
            app_handle: app.handle().clone(),
            session: tokio::sync::Mutex::new(None),
        };
        app.manage(state);

        app.run(|_app_handle, _event| {});
        
}


use sqlx::Connection;
// Подставьте ваши правильные пути к структурам, если они отличаются
use shared_lib::primitives::frozen::text::BoxUuid; 

// Структура для получения результатов запроса в рантайме
#[derive(sqlx::FromRow, Debug)]
struct TestRow {
    type_name: String,
    id_type: String,
    id: Vec<u8>, 
}

pub async fn run_uuid_experiment() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Буфер, куда мы записываем каждый шаг лога
    let mut report = String::new();
    report.push_str("\n\n==========================================================================\n");
    report.push_str("=== ЗАПУСК ЭКСПЕРИМЕНТА ЗАПИСИ И ЧТЕНИЯ UUID ===\n");
    report.push_str("==========================================================================\n");

    // 1. Путь к вашей базе данных Tauri
    let db_url = "sqlite:///home/khabipovrinat/dev/fin_app_full/crates/lite_mobile/src-tauri/data_base.db";
    let mut conn = sqlx::SqliteConnection::connect(db_url).await?;
    report.push_str(" Подключение к базе данных выполнено успешно.\n");

    // 2. Создаем временную тестовую таблицу и очищаем её
    sqlx::query("CREATE TABLE IF NOT EXISTS test_uuid_table (id TEXT, type_name TEXT);")
        .execute(&mut conn)
        .await?;
    sqlx::query("DELETE FROM test_uuid_table;")
        .execute(&mut conn)
        .await?;
    report.push_str(" Тестовая таблица 'test_uuid_table' создана и очищена.\n");

    // 3. Инициализируем переменные из вашего примера
    let a = BoxUuid::new("")?; // Сгенерирует случайный UUID v4
    let uu = a.as_ref();       // Получаем &uuid::Uuid
    let uu_t = a.to_string();  // Получаем String

    report.push_str(&format!(" Исходные данные в Rust:\n"));
    report.push_str(&format!("   - a (BoxUuid) токенизирован как строка: {}\n", uu_t));
    report.push_str(&format!("   - uu (&uuid::Uuid) отладочный вид: {:?}\n\n", uu));

    // Выполняем вставки и логируем процесс
    report.push_str(" Выполнение запросов INSERT:\n");

    // Тест №1: Записываем ваш кастомный BoxUuid напрямую (Проверка вашего Encode)
    sqlx::query("INSERT INTO test_uuid_table (id, type_name) VALUES ($1, '1. BoxUuid Direct')")
        .bind(&a) 
        .execute(&mut conn)
        .await?;
    report.push_str("   [ОК] Выполнен тест №1: Записан &BoxUuid\n");

    // Тест №2: Записываем результат .as_ref() (&uuid::Uuid)
    sqlx::query("INSERT INTO test_uuid_table (id, type_name) VALUES ($1, '2. uu (.as_ref())')")
        .bind(uu) 
        .execute(&mut conn)
        .await?;
    report.push_str("   [ОК] Выполнен тест №2: Записан &uuid::Uuid через .as_ref()\n");

    // Тест №3: Записываем чистую String
    sqlx::query("INSERT INTO test_uuid_table (id, type_name) VALUES ($1, '3. uu_t (.to_string())')")
        .bind(&uu_t) 
        .execute(&mut conn)
        .await?;
    report.push_str("   [ОК] Выполнен тест №3: Записана чистая String через .to_string()\n\n");

    report.push_str(" Чтение сохраненных данных из SQLite через typeof()...\n");

    // 4. Выгружаем данные с динамической проверкой typeof() на стороне SQLite
    let rows: Vec<TestRow> = sqlx::query_as::<_, TestRow>(
        "SELECT type_name, typeof(id) as id_type, id FROM test_uuid_table"
    )
    .fetch_all(&mut conn)
    .await?;

    // 5. Формируем финальную красивую матрицу результатов
    report.push_str("==========================================================================\n");
    report.push_str(" ФИНАЛЬНЫЙ ОТЧЕТ О ТИПАХ ДАННЫХ В СУБД:\n");
    report.push_str("==========================================================================\n");
    
    for row in rows {
        let display_val = String::from_utf8(row.id.clone())
            .unwrap_or_else(|_| {
                let hex_string: String = row.id.iter()
                    .map(|b| format!("{:02x}", b))
                    .collect();
                format!("[Бинарный BLOB hex]: {}", hex_string)
            });

        report.push_str(&format!(
            "Способ: {:<25} | Тип в SQLite: {:<5} | Значение: {}\n", 
            row.type_name, 
            row.id_type.to_uppercase(), 
            display_val
        ));
    }
    report.push_str("==========================================================================\n");

    // Паникуем, чтобы Cargo принудительно вывел весь накопленный лог в консоль терминала
    panic!("{}", report);
}

// Блок тестов для `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_my_uuid_experiment() {
        if let Err(e) = run_uuid_experiment().await {
            panic!("Тест упал на раннем этапе с ошибкой: {:?}", e);
        }
    }
}
