use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::BoxUuid;
use shared_lib::service::crypto_service::implements::{CheckSignDocData, PersonSignCheckResult};
use shared_lib::service::api_routes::implements::CryptoApiRoutes;
use shared_lib::service::mchd::service::{RegisterMchdData, MchdStep, MchdInfo};
use shared_lib::service::mchd::poa::PoaMchd;

use crate::config::BackApiState;
use crate::db::sql_queries::users::set::guid_by_user_id::set_guid_by_user_id;
use crate::db::service::mchd::mchd_storage::add_new_poa;

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
        .await {
            Ok(r) => r,
            Err(err) => {
                err.process_err(Status::QueryGetRequestErr, "");
                return Ok(failed_result);
            }
        };

    if !response.status().is_success() {
        let status_code = response.status();
        let error_body = response.text().await.unwrap_or_else(|_| "Failed to read body".to_string());
        let ext_inf = format!("url = {:?}, http_status = {:?}, err_body = {:?}", crypto_url, status_code, error_body);
        Status::Tech.process_err(Status::QueryGetRequestErr, &ext_inf);
        return Ok(failed_result);
    }

    let check_result: PersonSignCheckResult = match response
            .json()
            .await {
        Ok(r) => r,
        Err(err) => {
            err.process_err(Status::MappingError, "");
            return Ok(failed_result);
        }
    };

    if !check_result.is_signed {
        return Ok(MchdStep::WrongData { text: MchdInfo::WrongSignFile})
    }

    let xml_content = match String::from_utf8(xml_file.clone()) {
        Ok(c) => c,
        Err(err) => {
            err.process_err(Status::FileReadError, "");
            return Ok(failed_result);
        }
    };

    let poa: PoaMchd = match quick_xml::de::from_str(&xml_content) {
        Ok(p) => p,
        Err(err) => {
            err.process_err(Status::MappingError, "");
            return Ok(failed_result);
        }
    };

    let identificator: Vec<char> = poa.flie_identificator.to_string().chars().collect();

    let guide_str: String = if identificator.len() > 36 {
        identificator[identificator.len() - 36..].iter().collect()
    } else {
        Status::Tech.process_err(Status::SystemLogicErr, "");
        return Ok(failed_result);
    };

    let guide_uuid = match uuid::Uuid::parse_str(&guide_str) {
        Ok(g) => g,
        Err(err) => {
            err.process_err(Status::SystemLogicErr, "");
            return Ok(failed_result);
        }
    };

    let guide = BoxUuid::unchecked(guide_uuid);

    match set_guid_by_user_id(state, user_id, &guide).await {
        Ok(_) => {},
        Err(err) => {
            err.process_err(err, "");
            return Ok(failed_result);
        }
    }

    if let Err(err) = add_new_poa(poa).await {
        err.process_err(err, "");
        return Ok(failed_result);
    }
   
    
    Ok(MchdStep::SuccessRegisterMchd { guide, text: MchdInfo::SuccessRegisterMchd })
}