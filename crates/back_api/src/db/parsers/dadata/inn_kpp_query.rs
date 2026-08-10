use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::company::implements::Company;
use shared_lib::primitives::frozen::text::{CompInn, Kpp, CompType, Date, BoxUuid, DateTime};

use crate::config::BackApiState;
use crate::db::parsers::dadata::inn_query::dadata_reqwest_func;

pub(crate) async fn parse_company_by_inn_kpp(
    state: &BackApiState,
    comp_inn: &CompInn,
    kpp: &Kpp
) -> Result<Company, Status> {

    let mut metadata = dadata_reqwest_func(
            state, 
            comp_inn, 
            kpp)
        .await
        .map_err(|err| err.process_err(err, ""))?;

    let ext_info = format!("inn = {:?}, kpp = {:?}", comp_inn, kpp);
    
    let kpp = if let Some(kpp_m) = metadata.kpp.as_ref() {
        kpp_m.clone()
    } else {
        kpp.clone()
    };

    let okved = match &metadata.okved {
        Some(o) => o,
        None => {
            return Err(Status::MappingError.process_err(Status::MappingError, &ext_info));
        }
    };

    let opf_data = match &metadata.opf {
        Some(o_d) => o_d,
        None => {
            return Err(Status::MappingError.process_err(Status::MappingError, &ext_info));
        }
    };

    let opf_code = match &opf_data.opf_code {
        Some(code) => code,
        None => {
            return Err(Status::MappingError.process_err(Status::MappingError, &ext_info));
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
            return Err(Status::MappingError.process_err(Status::MappingError, ""));
        }
    };

    let comp_state = match &is_active_data.status {
        Some(s) => s,
        None => {
            return Err(Status::MappingError.process_err(Status::MappingError, ""));
        }
    };

    if let Some(ms) = metadata.ogrn_date_dadata {
        if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
            let date_str = dt.naive_utc().date().to_string();
            metadata.ogrn_date_date = Some(Date::new(date_str.as_str())?);
        }
    }

    let comp_id = BoxUuid::unchecked(uuid::Uuid::new_v4());


    Ok(Company {
        comp_id,
        comp_inn: comp_inn.clone(),
        kpp,
        comp_type,  
        comp_status:comp_state.clone(),
        metadata,
        last_update: DateTime::unchecked(chrono::Utc::now())
    })

}