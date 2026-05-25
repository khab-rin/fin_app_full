use axum::{extract::State, Json};
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use encoding_rs;

use crate::AppState;
use crate::person_snils::helper::parse_snils_from_stdout;


use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    CryptoVerifyRequest,
    CryptoVerifyPersonResponse
};

pub async fn verify_signature_handler (
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CryptoVerifyRequest> 
) -> Result<Json<CryptoVerifyPersonResponse>, Status> {

    let run_id = uuid::Uuid::new_v4().to_string();
    let temp_dir = std::env::temp_dir();

    let doc_path = temp_dir.join(format!("{}_doc.dat", run_id));
    let sig_path = temp_dir.join(format!("{}_sig.dat", run_id));
    
    let mut doc_file = tokio::fs::File::create(&doc_path)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::FileCreateError,
                "FUN verify_signature_handler FAILED BY CREATING DOC FILE"
            );
        }).map_err(|_| Status::FileCreateError)?;

    doc_file.write_all(&payload.document)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::FileWriteError,
                "FUN verify_signature_handler FAILED BY WRITING DOC FILE"
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

    sig_file.write_all(&payload.signature)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::FileWriteError,
                "FUN verify_signature_handler FAILED BY WRITING SIG FILE"
            );
        }).map_err(|_| Status::FileWriteError)?;

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
        return Ok(Json(CryptoVerifyPersonResponse {
            is_signed: false,
            snils: None,
            inn: None,
            fio: None
        }));
    }

    let (decoded_str, _, error_1251) = encoding_rs::WINDOWS_1251.decode(&output.stdout);

    let stdout_str = if error_1251 {
        String::from_utf8_lossy(&output.stdout).into_owned()
    } else {
        decoded_str.into_owned()
    };

    parse_snils_from_stdout(&stdout_str) 

}