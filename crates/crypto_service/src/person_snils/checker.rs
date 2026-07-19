use axum::{extract::State, Json};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;

use crate::AppState;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    RegFilesData,
    PersonSignCheckResult
};

pub async fn verify_signature_handler (
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegFilesData> 
) -> Result<Json<PersonSignCheckResult>, Status> {

    let run_id = uuid::Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir();

    let json_path = temp_dir.join(format!("{}.dat", run_id));
    let sig_path = temp_dir.join(format!("{}.dat.sig", run_id));
    
    let mut json_file = tokio::fs::File::create(&json_path)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::FileCreateError,
                "FUN verify_signature_handler FAILED BY CREATING json FILE"
            );
        }).map_err(|_| Status::FileCreateError)?;

    json_file.write_all(&payload.json_file)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::FileWriteError,
                "FUN verify_signature_handler FAILED BY WRITING json FILE"
            );
        }).map_err(|_| Status::FileWriteError)?;

    let mut sig_file = tokio::fs::File::create(&sig_path)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::FileCreateError,
                "FUN verify_signature_handler FAILED BY CREATING SIG FILE"
            );
        }).map_err(|_| Status::FileCreateError)?;

    sig_file.write_all(&payload.sign_file)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::FileWriteError,
                "FUN verify_signature_handler FAILED BY WRITING SIG FILE"
            );
        }).map_err(|_| Status::FileWriteError)?;

    let output = tokio::process::Command::new(&state.cryptcp_path)
        .arg("-vsignf")
        .arg("-detached")
        .arg("-dir")
        .arg(&temp_dir)                       
        .arg("-fext")
        .arg(".sig") // Сюда прилетит ".sig"                  
        .arg("-nochain")                      
        .arg("-f")
        .arg(sig_path.to_str().unwrap_or("")) 
        .arg(json_path.to_str().unwrap_or("")) 
        .output()
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::CryptoServerError,
                "FUN verify_signature_handler FAILED: COULD NOT EXECUTE CRYPTCP"
            );
        }).map_err(|_| Status::CryptoServerError)?;
    
    let _ = tokio::fs::remove_file(&json_path).await;
    let _ = tokio::fs::remove_file(&sig_path).await;
    
    if !output.status.success() {
        let stdout_err = String::from_utf8_lossy(&output.stdout);
        let stderr_err = String::from_utf8_lossy(&output.stderr);
        
        let detailed_error = format!(
            "Cryptcp failed with exit code: {:?}.\nStdout: {}\nStderr: {}", 
            output.status.code(), 
            stdout_err, 
            stderr_err
        );

        tracing::warn!(
            wrong_data = %detailed_error,
            "Signature verification failed via cryptcp"
        );

        return Ok(Json(PersonSignCheckResult {
            is_signed: false,
            text: detailed_error, 
        }));
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout).into_owned();

    Ok(Json(PersonSignCheckResult { is_signed: true, text: stdout_str}))

}