use axum::{extract::State, Json};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use crate::AppState;

use shared_lib::{Status, ProcessError};
use shared_lib::service::crypto_service::implements::{
    CheckSignDocData,
    PersonSignCheckResult
};

pub async fn verify_signature_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CheckSignDocData> 
) -> Result<Json<PersonSignCheckResult>, Status> {

    let run_id = uuid::Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir();

    // Формируем короткие имена файлов для рабочей директории
    let init_filename = format!("{}.dat", run_id);
    let sig_filename = format!("{}.dat.sig", run_id);

    let init_path = temp_dir.join(&init_filename);
    let sig_path = temp_dir.join(&sig_filename);
    
    // 1. Записываем файл данных
    let mut init_file = tokio::fs::File::create(&init_path)
        .await
        .map_err(|err| err.process_err(Status::FileCreateError, ""))?;

    init_file.write_all(&payload.init_file)
        .await
        .map_err(|err| err.process_err(Status::FileWriteError, ""))?;


    // 2. Записываем файл подписи
    let mut sig_file = tokio::fs::File::create(&sig_path)
        .await
        .map_err(|err| err.process_err(Status::FileCreateError, ""))?;


    sig_file.write_all(&payload.sign_file)
        .await
        .map_err(|err| err.process_err(Status::FileWriteError, ""))?;


    // 3. Шаг 1: Проверяем подпись через cryptcp (с фиксированной рабочей директорией)
    let cryptcp_output = tokio::process::Command::new(&state.cryptcp_path)
        .current_dir(&temp_dir) // Переходим в /tmp, чтобы избегать ошибки 0x20000065
        .arg("-vsignf")
        .arg("-detached")
        .arg("-nochain")
        .arg("-fext")
        .arg(".sig")
        .arg("-f")
        .arg(&sig_filename)   // Относительное имя
        .arg(&init_filename)  // Относительное имя
        .output()
        .await
        .map_err(|err| err.process_err(Status::CryptoServerError, ""))?;


    // 4. Проверяем статус работы cryptcp
    if !cryptcp_output.status.success() {
        let stdout_err = String::from_utf8_lossy(&cryptcp_output.stdout);
        let stderr_err = String::from_utf8_lossy(&cryptcp_output.stderr);
        
        let detailed_error = format!(
            "Cryptcp failed with exit code: {:?}.\nStdout: {}\nStderr: {}", 
            cryptcp_output.status.code(), 
            stdout_err, 
            stderr_err
        );

        tracing::warn!(
            wrong_data = %detailed_error,
            "Signature verification failed via cryptcp"
        );

        // Удаляем временные файлы перед возвратом
        let _ = tokio::fs::remove_file(&init_path).await;
        let _ = tokio::fs::remove_file(&sig_path).await;

        return Ok(Json(PersonSignCheckResult {
            is_signed: false,
            text: detailed_error, 
        }));
    }

    // 5. Шаг 2: Вызываем certmgr для извлечения понятных полей Subject (ИНН, СНИЛС, ОГРН и т.д.)
    let certmgr_path = state.cryptcp_path.replace("cryptcp", "certmgr"); // или возьми из state.certmgr_path
    
    let cert_out = tokio::process::Command::new(&certmgr_path)
        .current_dir(&temp_dir)
        .arg("-list")
        .arg("-file")
        .arg(&sig_filename)
        .output()
        .await
        .map_err(|err| err.process_err(Status::CryptoServerError, ""))?;

    // Удаляем временные файлы сразу после выполнения всех команд
    let _ = tokio::fs::remove_file(&init_path).await;
    let _ = tokio::fs::remove_file(&sig_path).await;

    // Формируем итоговый результат
    let mut result_text = String::from_utf8_lossy(&cryptcp_output.stdout).into_owned();

    if cert_out.status.success() {
        let cert_str = String::from_utf8_lossy(&cert_out.stdout);
        
        if let Some(subject_line) = cert_str.lines().find(|line| line.starts_with("Subject")) {
            result_text = format!("{}\n\n{}", result_text, subject_line);
        } else {
            result_text = format!("{}\n\n{}", result_text, cert_str);
        }
    }
    

    tracing::info!(info = %result_text);

    Ok(Json(PersonSignCheckResult { 
        is_signed: true, 
        text: result_text 
    }))
}