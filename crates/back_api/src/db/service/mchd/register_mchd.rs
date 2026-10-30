use shared_lib::service::mchd::implements::PoaRootKind;
use shared_lib::{Status, ProcessError, LL};
use shared_lib::primitives::frozen::text::BoxUuid;
use shared_lib::service::cryp_serv::implements::{CheckSignDocData, PersonSignCheckResult};
use shared_lib::service::api_routes::implements::CryptoApiRoutes;
use shared_lib::service::mchd::service::{RegisterMchdData, MchdStep, MchdInfo};
use shared_lib::service::mchd::poa::PoaMchd;

use crate::config::BackApiState;
use crate::db::service::auth_service::pers_sign_parser::parse_crypto_fields_org;
use crate::db::sql_queries::users::set::guid_by_user_id::set_guid_by_user_id;
use crate::db::service::mchd::mchd_storage::add_new_poa;
use crate::db::sql_queries::users::get::by_user_id::get_user_by_user_id;

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

	let sign_fields = match parse_crypto_fields_org(&check_result.text) {
		Ok(r) => r,
		Err(err) => {
			err.process_err(err, "");
			return Ok(failed_result);
		}
	};

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

	let session_user_option = get_user_by_user_id(state, user_id)
		.await
		.map_err(|err| err.process_err(err, ""))?;

	let session_user = match session_user_option {
		Some(s) => s,
		None => {return Ok(failed_result)}
	};

	let docum = match &poa.poa.poa_doc {
		PoaRootKind::RootPoa(doc) => doc,
		_ => return Ok(MchdStep::WrongData { text: MchdInfo::WrongSignFile })
	};

	let principal = match docum.principal.first() {
		Some(p) => &p.principal,
		_ => return Ok(MchdStep::WrongData { text: MchdInfo::WrongSignFile })
	};

	let manager_inn = if let Some(ip) = &principal.ip {
		&ip.inn
	} else if let Some(company) = &principal.russian_org {
		let manager = match company.root_managers.first() {
			Some(m) => m,
			_ => {return Ok(MchdStep::WrongData { text: MchdInfo::WrongSignFile })}
		};

		let pers_inn_option = match &manager.prime_manager_person {
			Some(p) => &p.inn,
			_ => {return Ok(MchdStep::WrongData { text: MchdInfo::WrongSignFile })}
		};
		match pers_inn_option {
			Some(p_i) => p_i,
			_ => {return Ok(MchdStep::WrongData { text: MchdInfo::WrongSignFile })}
		}
	} else {
		return Ok(MchdStep::WrongData { text: MchdInfo::WrongSignFile });
	};

	let comp_inn = &session_user.company.comp_inn;

	if sign_fields.comp_inn != comp_inn || sign_fields.pers_inn != manager_inn {
		Status::log(LL::Debug, &format!("comp_inn = {:?}, manager_inn = {:?}, sign_fields = {:?}", comp_inn, manager_inn, sign_fields));
		return Ok(MchdStep::WrongData { text: MchdInfo::WrongSignFile });
	}

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