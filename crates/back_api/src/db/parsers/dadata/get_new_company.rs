use shared_lib::sql_models::id_rules::IdRules;
use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::company::implements::Company;
use shared_lib::primitives::frozen::text::{CompInn, Kpp, CompType, Date, BoxUuid, DateTime};

use crate::config::BackApiState;
use crate::db::parsers::dadata::get_metadata::get_dadata_metadata;

pub(crate) async fn get_dadata_company(
    state: &BackApiState,
    comp_inn: &CompInn,
    kpp: &Kpp
) -> Result<Option<Company>, Status> {

    let metadata_option = match get_dadata_metadata(
            state, 
            comp_inn, 
            kpp)
        .await {
            Ok(m) => m,
            Err(err) => {
                err.process_err(err, "");
                return Ok(None);
            }
        };
    
    let mut metadata = match metadata_option {
        Some(m) => m,
        None => return Ok(None)
    };

    let ext_info = format!("inn = {:?}, kpp = {:?}", comp_inn, kpp);
    
    let kpp = if let Some(kpp_m) = metadata.kpp.as_ref() {
        kpp_m.clone()
    } else {
        kpp.clone()
    };

    let okved = match &metadata.okved {
        Some(o) => o,
        None => {
            Status::Tech.process_err(Status::MappingError, &ext_info);
            return Ok(None);
        }
    };

    let opf_data = match &metadata.opf {
        Some(o_d) => o_d,
        None => {
            Status::Tech.process_err(Status::MappingError, &ext_info);
            return Ok(None);
        }
    };

    let opf_code = match &opf_data.opf_code {
        Some(code) => code,
        None => {
            Status::Tech.process_err(Status::MappingError, &ext_info);
            return Ok(None);
        }
    };

    let comp_type = if okved.starts_with("64.1") || okved.starts_with("64.92") {
        CompType::Bank
    } else if opf_code.starts_with('7') || opf_code.starts_with('6') {
        CompType::Gov
    } else if opf_code.starts_with('5') {
        CompType::Ip
    } else {
        CompType::ComEnt
    };

    let is_active_data = match &metadata.is_active {
        Some(d) => d,
        None => {
            Status::Tech.process_err(Status::MappingError, &ext_info);
            return Ok(None);
        }
    };

    let comp_state = match &is_active_data.status {
        Some(s) => s,
        None => {
            Status::Tech.process_err(Status::MappingError, &ext_info);
            return Ok(None);
        }
    };

    if let Some(ms) = metadata.ogrn_date_dadata && let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
		let date_str = dt.naive_utc().date().to_string();
		metadata.ogrn_date_date = match Date::new(date_str.as_str()) {
			Ok(d) => Some(d),
			Err(err) => {
				err.process_err(Status::MappingError, &ext_info);
				return Ok(None);
			}
		}

    }

	let rule = IdRules::Company { comp_inn: comp_inn.clone(), kpp: kpp.clone() };
	let comp_id = rule.make_stat_id();

    Ok(Some(Company {
        comp_id,
        comp_inn: comp_inn.clone(),
        kpp,
        comp_type,  
        comp_status:comp_state.clone(),
        metadata,
        last_update: DateTime::unchecked(chrono::Utc::now())
    }))

}