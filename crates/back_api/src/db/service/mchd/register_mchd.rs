use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;
use shared_lib::service::crypto_service::implements::{CheckSignDocData, PersonSignCheckResult};
use shared_lib::service::api_routes::implements::CryptoApiRoutes;
use shared_lib::service::mchd::service::{RegisterMchdData, MchdStep, MchdInfo};
use shared_lib::service::mchd::poa::PoaMchd;

use crate::config::BackApiState;
use crate::db::sql_queries::users::set::guid_by_user_id::set_guid_by_user_id;
use crate::db::service::mchd::mchd_storage::{
    get_mchd_storage,
    add_new_mchd,
    write_mchd_storage_to_file
};

pub(crate) async fn register_mchd(
    state: &BackApiState,
    data: &RegisterMchdData
) -> Result<MchdStep, Status> {
    
    let failed_result = MchdStep::TryLater { text: MchdInfo::BackApiError };
    
    let RegisterMchdData { 
        xml_file, 
        sig_file, 
        user_id } = data;
    
    let check_data =  CheckSignDocData {
        init_file: xml_file.clone(),
        sign_file: sig_file.clone()
    };

    let crypto_url = format!(
        "{}/{}",
        state.config.crypto_servise.url.trim_end_matches('/'),
        CryptoApiRoutes::CryptoVerifyPerson.get_path().trim_start_matches('/')
    );

    let response = match state
        .config
        .get_inst_client()
        .post(&crypto_url)
        .json(&check_data)
        .send()
        .await
    {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(
                target: "back_api::crypto_client",
                url = %crypto_url,
                err = ?err,
                is_timeout = err.is_timeout(),
                is_connect = err.is_connect(),
                local_err = ?Status::QueryGetRequestErr,
                "FUN register_step2 FAILED BY NETWORK/CONNECT TO CRYPTO SERVICE"
            );
            return Ok(failed_result);
        }
    };

    if !response.status().is_success() {
        let status_code = response.status();
        
        let error_body = response.text().await.unwrap_or_else(|_| "Failed to read body".to_string());

        tracing::error!(
            target: "back_api::crypto_client",
            url = %crypto_url,
            http_status = %status_code,
            response_body = %error_body,
            local_err = ?Status::QueryGetRequestErr,
            "FUN register_step2 FAILED: CRYPTO SERVICE RETURNED ERROR STATUS"
        );
        return Ok(failed_result);
    }

    let check_result: PersonSignCheckResult = match response
            .json()
            .await {
        Ok(r) => r,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN register_step2 FAILED BY MAPPING CryptoVerifyPersonResponse"
            );
            return Ok(failed_result);
        }
    };

    if !check_result.is_signed {
        return Ok(MchdStep::WrongData { text: MchdInfo::WrongSignFile})
    }

    tracing::info!(text = %check_result.text);

    let xml_content = match String::from_utf8(xml_file.clone()) {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::FileReadError,
                "FUN register_mchd FAILED BY String::from_utf8(xml_file.clone())"
            );
            return Ok(failed_result);
        }
    };

    let poa: PoaMchd = match quick_xml::de::from_str(&xml_content) {
        Ok(p) => p,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::MappingError,
                "FUN register_mchd FAILED BY quick_xml::de::from_str(&xml_content)"
            );
            return Ok(failed_result);
        }
    };

    let identificator: Vec<char> = poa.flie_identificator.to_string().chars().collect();

    let guide_str: String = if identificator.len() > 36 {
        identificator[identificator.len() - 36..].iter().collect()
    } else {
        tracing::error!(
            local_err = ?Status::SystemLogicErr,
            "FUN register_mchd FAILED BY if identificator.len() > 36"
        );
        return Ok(failed_result);
    };

    let guide_uuid = match uuid::Uuid::parse_str(&guide_str) {
        Ok(g) => g,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::SystemLogicErr,
                "FUN register_mchd FAILED BY uuid::Uuid::parse_str(&guide_str)"
            );
            return Ok(failed_result);
        }
    };

    let guide = BoxUuid::unchecked(guide_uuid);

    match set_guid_by_user_id(state, user_id, &guide).await {
        Ok(_) => {},
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN register_mchd FAILED BY WRONG SQL QUERY"
            );
            return Ok(failed_result);
        }
    }
    
    let storage = match get_mchd_storage() {
        Ok(s) => s,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN register_mchd FAILED BY FUN get_mchd_storage"
            );
            return Ok(failed_result);
        }
    };

    let new_storage = match add_new_mchd(poa, storage) {
        Ok(s) => s,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN register_mchd FAILED BY FUN add_new_mchd"
            );
            return Ok(failed_result);
        }
    };

    match write_mchd_storage_to_file(new_storage) {
        Ok(_) => {},
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN register_mchd FAILED BY FUN write_mchd_storage_to_file"
            );
            return Ok(failed_result);
        }
    }

    
    Ok(MchdStep::SuccessRegisterMchd { guide, text: MchdInfo::SuccessRegisterMchd })
}