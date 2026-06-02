use std::sync::Arc;

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, CompInn, Kpp, CompType, Date, DateTime};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};

use crate::config::BackApiState;
use crate::db::parsers::dadata::parser::dadata_reqwest_func;

pub(crate) fn dto_to_company_vec(
    dtos: Vec<CompanyDto>
) -> Vec<Company> {
    let mut res:Vec<Company> = vec!();
    for dto in dtos {
        match dto.clone().try_into() {
            Ok(company) => res.push(company),
            Err(err) => {
                tracing::error!(
                    err = ?err,
                    elemet = ?dto
                )
            }
        }
    }
    res
}

pub(crate) async fn make_new_company(
    state: &Arc<BackApiState>,
    comp_inn: &CompInn,
    kpp: &Kpp
) -> Result<Company, Status> {

    let mut meta_d = match dadata_reqwest_func(state, comp_inn, kpp).await {
        Ok(m_d) => m_d,
        Err(err) => {
            tracing::error!(
                err = ?err,
                failed_data = ?(comp_inn, kpp),
                "FUN make_new_company FAILED BY FUN dadata_reqwest_func"
            );
            return Err(err);
        }
    };

    let okved = match &meta_d.okved {
        Some(o) => o,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN make_new_company FAILED CtrprtyMetadata MISS okved"
            );
            return Err(Status::MappingError);
        }
    };

    let opf_data = match &meta_d.opf {
        Some(o_d) => o_d,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN make_new_company FAILED CtrprtyMetadata MISS opf_data"
            );
            return Err(Status::MappingError);
        }
    };

    let opf_code = match &opf_data.opf_code {
        Some(code) => code,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN make_new_company FAILED CtrprtyMetadata MISS opf_code"
            );
            return Err(Status::MappingError);
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

    let is_active_data = match &meta_d.is_active {
        Some(d) => d,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN make_new_company FAILED CtrprtyMetadata MISS is_active_data"
            );
            return Err(Status::MappingError);
        }
    };

    let comp_state = match &is_active_data.status {
        Some(s) => s,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN make_new_company FAILED CtrprtyMetadata MISS comp_state"
            );
            return Err(Status::MappingError);
        }
    };

    if let Some(ms) = meta_d.ogrn_date_dadata {
        if let Some(dt) = chrono::DateTime::from_timestamp_millis(ms) {
            let date_str = dt.naive_utc().date().to_string();
            meta_d.ogrn_date_date = Some(Date::new(date_str.as_str())?);
        }
    }

    let comp_id = match BoxUuid::new(uuid::Uuid::new_v4().to_string().as_str()) {
        Ok(u) => u,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::ValidBoxUuid,
                "FUN make_company FAILED BY PARSING BoxUuid"
            );
            return Err(Status::ValidBoxUuid);
        }
    };

    Ok(Company {
        comp_id,
        comp_inn: comp_inn.clone(),
        kpp: kpp.clone(),
        comp_type,  
        comp_status:comp_state.clone(),
        metadata: meta_d,
        last_update: DateTime::unchecked(chrono::Utc::now())
    })

}