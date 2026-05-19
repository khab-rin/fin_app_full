use axum::Json;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{Inn, Snils, MidName, FirstName, SurName};
use shared_lib::primitives::composite::implements::Fio;
use shared_lib::service::auth_service::implements::CryptoVerifyPersonResponse;
use shared_lib::static_data::crypto_re::*;

pub(crate) fn parse_snils_from_stdout(
    value: &str
) -> Result<Json<CryptoVerifyPersonResponse>, Status> {

    let mut result = CryptoVerifyPersonResponse {
        is_signed: true,
        snils: None,
        inn: None,
        fio: None
    };

    let subj_line = value
        .lines()
        .find(|line| line.contains("Subject:"))
        .unwrap_or(value);
    
    let snils_str = match get_scan_crypto_snils_reg().captures(subj_line) {
        Some(c) => c[1].to_owned(),
        None => {
            tracing::warn!(
                "FUN parse_snils_from_stdout SNILS IS MISSED"
            );
            return Ok(Json(result));
        }
    };

    result.snils = match Snils::new(&snils_str) {
        Ok(s) => Some(s),
        Err(err) => {
            tracing::warn!(
                err = ?err,
                "FUN parse_snils_from_stdout SNILS IS MISSED"
            );
            return Ok(Json(result));
        }
    };

    let inn_str = match get_scan_crypto_person_inn_reg().captures(subj_line) {
        Some(c) => c[1].to_owned(),
        None => {
            tracing::warn!(
                "FUN parse_snils_from_stdout INN IS MISSED"
            );
            return Ok(Json(result));
        }
    };

    result.inn = match Inn::new(&inn_str) {
        Ok(s) => Some(s),
        Err(err) => {
            tracing::warn!(
                err = ?err,
                "FUN parse_snils_from_stdout INN IS MISSED"
            );
            return Ok(Json(result));
        }
    };

    let first_name_str = match get_scan_crypto_name_reg().captures(subj_line) {
        Some(c) => c[1].to_owned(),
        None => {
            tracing::warn!(
                "FUN parse_snils_from_stdout FIRSTNAME IS MISSED"
            );
            return Ok(Json(result));
        }
    };

    let first_name = match FirstName::new(&first_name_str) {
        Ok(s) => s,
        Err(err) => {
            tracing::warn!(
                err = ?err,
                "FUN parse_snils_from_stdout FIRSTNAME IS MISSED"
            );
            return Ok(Json(result));
        }
    };

    let sur_name_str = match get_scan_crypto_surname_reg().captures(subj_line) {
        Some(c) => c[1].to_owned(),
        None => {
            tracing::warn!(
                "FUN parse_snils_from_stdout SURNAME IS MISSED"
            );
            return Ok(Json(result));
        }
    };

    let sur_name = match SurName::new(&sur_name_str) {
        Ok(s) => s,
        Err(err) => {
            tracing::warn!(
                err = ?err,
                "FUN parse_snils_from_stdout SURNAME IS MISSED"
            );
            return Ok(Json(result));
        }
    };

    let fio: Fio = Fio { 
        sur_name, 
        first_name, 
        mid_name: None 
    };

    result.fio = Some(fio);

    let mid_name_str = match get_scan_crypto_mid_name_reg().captures(subj_line) {
        Some(c) => c[1].to_owned(),
        None => {
            return Ok(Json(result));
        }
    };

    match MidName::new(&mid_name_str) {
        Ok(s) => {
            if let Some(fio) = result.fio.as_mut() {
                fio.mid_name = Some(s);
            }
        },
        Err(err) => {
            tracing::warn!(
                err = ?err,
                "FUN parse_snils_from_stdout MIDNAME IS MISSED"
            );
        }
    };

    Ok(Json(result))
}