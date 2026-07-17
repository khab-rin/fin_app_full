use shared_lib::Status;
use shared_lib::sql_models::company::implements::Company;
use shared_lib::primitives::frozen::implements::{CompInn, Kpp, CompType, Date, BoxUuid, DateTime};
use shared_lib::parsers::dadata::implements::*;

use crate::config::BackApiState;

pub(crate) async fn parse_company_by_inn_kpp(
    state: &BackApiState,
    comp_inn: &CompInn,
    kpp: &Kpp
) -> Result<Company, Status> {
    let client = state.config.get_inst_client();

    let header = state.config.get_dadata_header();
    let url = &state.config.dadata.dadata_comp_url;

    let response = client
        .post(url)
        .headers(header.clone())
        .json(&serde_json::json!({"query": comp_inn, "kpp": kpp}))
        .send()
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_error = ?err,
                status_err = ?Status::QueryPostRequestErr,
                comp_inn = %comp_inn, 
                kpp = %kpp);
        })
        .map_err(|_| Status::QueryPostRequestErr)?;

    let status = response.status();

    if !status.is_success() {
        let error_body = response.text().await.unwrap_or_else(|_| "Не удалось прочитать тело ответа".to_string());
        
        tracing::error!(
            status_code = %status,
            body = %error_body,
            "DaData вернула ошибку вместо данных компании!"
        );
        return Err(Status::QueryPostRequestErr);
    }

    std::println!("{:?}", response);

    let resp_wrap:DadaRespWrap = response
        .json()
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_error = ?err,
                status_err = ?Status::MappingError,
                inn = %comp_inn, 
                kpp = %kpp);
        })
        .map_err(|_| Status::MappingError)?;

    let metadata_option = match resp_wrap.suggestions.iter().next() {
        Some(m) => m.data.clone(),
        None => {
            tracing::error!(
                local_err = ?Status::DadataResponseError,
                "FUN parse_company_by_inn_kpp FAILED BY DADATA RESPONSE"
            );
            return Err(Status::DadataResponseError);
        }
    };

    let mut metadata = match metadata_option {
        Some(m) => m,
        None => {
            tracing::error!(
                local_err = ?Status::DadataResponseError,
                "FUN parse_company_by_inn_kpp FAILED BY DADATA RESPONSE"
            );
            return Err(Status::DadataResponseError);
        }
    };

    let okved = match &metadata.okved {
        Some(o) => o,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN parse_company_by_inn_kpp FAILED CtrprtyMetadata MISS okved"
            );
            return Err(Status::MappingError);
        }
    };

    let opf_data = match &metadata.opf {
        Some(o_d) => o_d,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN parse_company_by_inn_kpp FAILED CtrprtyMetadata MISS opf_data"
            );
            return Err(Status::MappingError);
        }
    };

    let opf_code = match &opf_data.opf_code {
        Some(code) => code,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN parse_company_by_inn_kpp FAILED CtrprtyMetadata MISS opf_code"
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

    let is_active_data = match &metadata.is_active {
        Some(d) => d,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN make_company FAILED CtrprtyMetadata MISS is_active_data"
            );
            return Err(Status::MappingError);
        }
    };

    let comp_state = match &is_active_data.status {
        Some(s) => s,
        None => {
            tracing::error!(
                err = ?Status::MappingError,
                "FUN make_company FAILED CtrprtyMetadata MISS comp_state"
            );
            return Err(Status::MappingError);
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
        kpp: kpp.clone(),
        comp_type,  
        comp_status:comp_state.clone(),
        metadata,
        last_update: DateTime::unchecked(chrono::Utc::now())
    })

}