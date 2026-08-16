use crate::{Status, ClientState, ProcessError};
use crate::primitives::frozen::text::{Kpp, CompInn, BoxUuid, CompStatus, CompType, DateTime};
use crate::sql_models::company::implements::{Company, CompanyDto};
use crate::client::sql_queries::companys::add::new_company::add_company_by_inn_cpp_acc;

pub async fn get_company_by_inn_kpp(
    state: &ClientState,
    comp_inn: &CompInn,
    kpp: &Kpp
) -> Result<Option<Company>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

    let var1= comp_inn.as_ref();
    let var2 = kpp.as_ref();

    let company_dto_option = sqlx::query_file_as!(
            CompanyDto,
            "src/client/sql_queries/companys/get/company_by_inn_kpp.sql",
            var1,
            var2
        ).fetch_optional(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    if let Some(dto) = company_dto_option {
        let company: Company = dto.try_into().map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;
        return Ok(Some(company));
    }

    let comp_inn_option = Some(comp_inn.clone());
    let kpp_option = Some(kpp.clone());
        
    add_company_by_inn_cpp_acc(
        state, &comp_inn_option, &kpp_option, &None
    ).await


}