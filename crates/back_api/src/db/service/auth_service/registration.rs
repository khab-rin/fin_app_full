use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{RegisterResponse, VerifyData,VerifyMethod, RegistrationRequestDto};
use shared_lib::sql_models::person_models::implements::Person;

use crate::config::ApiState;
use crate::db::parsers::cryptocp::parser::parse_cryptcp_output;

pub(crate) async fn register_new_user(
    state: &Arc<ApiState>,
    payload: RegistrationRequestDto
) -> Result<RegisterResponse, Status> {

    let failed_res = RegisterResponse::Verify(
        VerifyData { 
            device_id: payload.device_id.clone(),
            method: VerifyMethod::TryLater {}, 
        });
    
    let failed_data = (
        payload.person.inn.clone(),
        payload.device_id.clone(),
        payload.comp_inn.clone(),
        payload.kpp.clone()
    );

    let run_id = Uuid::new_v4().to_string();
    let tmp_dir = std::env::temp_dir();

    let doc_path = tmp_dir.join(format!("{}.dat", run_id));
    let sig_path = tmp_dir.join(format!("{}.sig", run_id));
    
    let mut doc_file = match tokio::fs::File::create(&doc_path).await {
        Ok(d_file) => d_file,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                failed_data = ?failed_data,
                "FUN register_new_user FAILED BY tokio::fs::File::create"
            );
            return Ok(failed_res);
        }
    };

    match doc_file.write_all(&payload.document).await {
        Ok(_) => {}
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                failed_data = ?failed_data,
                "FUN register_new_user FAILED BY doc_file.write_all"
            );
            return Ok(failed_res);
        }
    }

    let mut sig_file = match tokio::fs::File::create(&sig_path).await {
        Ok(s_file) => s_file,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                failed_data = ?failed_data,
                "FUN register_new_user FAILED BY tokio::fs::File::create"
            );
            return Ok(failed_res);
        }
    };

    match sig_file.write_all(&payload.document).await {
        Ok(_) => {}
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                failed_data = ?failed_data,
                "FUN register_new_user FAILED BY sig_file.write_all"
            );
            return Ok(failed_res);
        }
    }

    let output = match tokio::process::Command::new("/opt/cprocsp/bin/amd64/cryptcp")
        .arg("-vfy")
        .arg("-detached")
        .arg("-dir")
        .arg(&tmp_dir)
        .arg(sig_path.to_str().unwrap_or(""))
        .output()
        .await {
            Ok(o) => o,
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    failed_data = ?failed_data,
                    "FUN register_new_user FAILED BY CRYPTO PRO CLI CHECKING"
                );
                return Ok(failed_res);
            }
        };

    let _ = tokio::fs::remove_file(&doc_path).await;
    let _ = tokio::fs::remove_file(&sig_path).await;
    
    if !output.status.success() {
        return Err(Status::Unknown); // Математика подписи неверна или отозвана цепочка Минцифры
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);
    let cert_info = parse_cryptcp_output(&stdout_str)
        .map_err(|_| Status::Unknown)?;

    // Сверяем СНИЛС присланного Person со СНИЛС из КЭП
    if payload.person.metadata.snils.doc_ser_num.to_string() != cert_info {
        return Err(Status::Unknown); // Попытка подмены личности физлица
    }



    Err(Status::Unknown)
}