use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::env::temp_dir;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use crate::AppState;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    CryptoVerifySnilsRequest,
    CryptoVerifySnilsResponse
};



pub async fn verify_signature_handler (
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CryptoVerifySnilsRequest> 
) -> Result<Json<CryptoVerifySnilsResponse>, Status> {

    let run_id = Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir();

    let doc_path = temp_dir.join(format!("{}.dat", run_id));
    let sig_path = temp_dir.join(format!("{}.dat", run_id));
    
    let mut doc_file = tokio::fs::File::create(&doc_path)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::CryptoServerError,
                "FUN verify_signature_handler FAILED BY CREATING DOC FILE"
            );
        }).map_err(|_| Status::CryptoServerError)?;

    doc_file.write_all(&payload.document)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::CryptoServerError,
                "FUN verify_signature_handler FAILED BY WRITING DOC FILE"
            );
        }).map_err(|_| Status::CryptoServerError)?;

    let mut sig_file = tokio::fs::File::create(&sig_path)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::CryptoServerError,
                "FUN verify_signature_handler FAILED BY CREATING SIG FILE"
            );
        }).map_err(|_| Status::CryptoServerError)?;

    sig_file.write_all(&payload.signature)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::CryptoServerError,
                "FUN verify_signature_handler FAILED BY WRITING SIG FILE"
            );
        }).map_err(|_| Status::CryptoServerError)?;

    let output = tokio::process::Command::new(&state.cryptcp_path)
        .arg("-vfy")
        .arg("-detached")
        .arg("-dir")
        .arg(&temp_dir)
        .arg(sig_path.to_str().unwrap_or("")) // Файл подписи
        .arg(doc_path.to_str().unwrap_or(""))
        .output()
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::CryptoServerError,
                "FUN verify_signature_handler FAILED: COULD NOT EXECUTE CRYPTCP"
            );
        }).map_err(|_| Status::CryptoServerError)?;
    
    let _ = tokio::fs::remove_file(&doc_path).await;
    let _ = tokio::fs::remove_file(&sig_path).await;
    
    if !output.status.success() {
        let wrong_data = String::from_utf8_lossy(&output.stderr).to_string();
    
        tracing::warn!(
            wrong_data = %wrong_data,
            "Signature verification failed via cryptcp (invalid signature)"
        );
        return Ok(Json(CryptoVerifySnilsResponse {
            is_signed: false,
            snils: None
        }));
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    match parse_snils_from_stdout(&stdout_str) {
        Ok(snils) => {
            // Переменную из Ok назвали snils, теперь имя совпадает с полем структуры!
            Ok(Json(CryptoVerifySnilsResponse {
                is_signed: true,
                snils: Some(snils), // Rust поймет, что это snils: Some(snils)
            }))
        }
        Err(_) => {
            tracing::error!(
                "Cryptcp verification succeeded, but parser could not extract SNILS. Stdout: {}", 
                stdout_str
            );
            
            Ok(Json(CryptoVerifySnilsResponse {
                is_signed: true,
                snils: None, 
            }))
        }
    }

}