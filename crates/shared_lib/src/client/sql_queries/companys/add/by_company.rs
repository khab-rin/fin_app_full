use crate::{Status, ClientState, ProcessError};
use crate::sql_models::company::implements::{Company, CompanyDto};
use crate::primitives::frozen::text::{CompInn, Kpp, BoxUuid, CompStatus, CompType, DateTime};

use crate::client::sql_queries::companys::get::by_inn_kpp::get_company_by_inn_kpp;
pub async fn add_company(
    state: &ClientState,
    company: &Company
) -> Result<Company, Status> {

	log::debug!("add_company function started!");

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

	let prev_company_option = get_company_by_inn_kpp(
		state, &company.comp_inn, &company.kpp).await
		.map_err(|err| err. process_err(err, ""))?;

	let company = match prev_company_option {
		Some(mut c) => {
			c.comp_status = company.comp_status.clone();
            c.comp_type = company.comp_type.clone();
            c.metadata.bank_acc.extend(company.metadata.bank_acc.clone());
            c
		},
		None => company.clone()
	};

	let comp_id = &company.comp_id;
	let comp_inn = &company.comp_inn;
	let kpp = &company.kpp;
	let comp_type = &company.comp_type;
	let comp_status = &company.comp_status;
	let metadata = serde_json::to_string(&company.metadata).unwrap_or_else(|_| "{}".to_string());
	let last_update = &company.last_update;

	let company_dto = sqlx::query_file_as!(
		CompanyDto,
		"src/client/sql_queries/companys/add/by_company.sql",
		comp_id,
		comp_inn,
		kpp,
		comp_type,
		comp_status,
		metadata,
		last_update
	).fetch_one(&session.local_db).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	let company: Company = company_dto
		.try_into()
		.map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;
	
	log::debug!("add_company function finished!");

	Ok(company)
}