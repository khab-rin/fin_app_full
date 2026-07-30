use shared_lib::Status;
use shared_lib::primitives::frozen::text::{Kpp, CompInn, BoxUuid, CompStatus, CompType, DateTime};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};

use crate::state::ClientState;

pub(crate) async fn get_company_by_inn_kpp(
    state: &ClientState,
    comp_inn: &CompInn,
    kpp: &Kpp
) -> Result<Option<Company>, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN get_company_by_inn_kpp FAILED BY EMPTY SESSION", err
            );
            return Err(err);
        }
    };

    let var1= comp_inn.as_ref();
    let var2 = kpp.as_ref();

    let company_dto_option = match sqlx::query_file_as!(
        CompanyDto,
        "src/sql_queries/companys/get/company_by_inn_kpp.sql",
        var1,
        var2
    ).fetch_optional(&session.local_db).await {
        Ok(o) => o,
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN get_company_by_inn_kpp BY SQL QUERY",
                err, Status::SqlQueryWrongLogic
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    let company_dto = match company_dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    let company: Company = match company_dto.try_into() {
        Ok(c) => c,
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN get_company_by_inn_kpp FAILED BY MAPPING Company",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
        }
    };


    Ok(Some(company))
}