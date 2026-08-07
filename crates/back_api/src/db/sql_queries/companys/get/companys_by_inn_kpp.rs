use shared_lib::{Status, ProcessError};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};
use shared_lib::primitives::frozen::text::{BoxUuid, CompInn, Kpp, CompType, CompStatus, DateTime};

use crate::config::BackApiState;
use crate::db::sql_queries::companys::helper::dto_to_company_vec;

pub(crate) async fn get_companys_by_inn_kpp(
    state: &BackApiState,
    comp_inn_data: &[String],
    kpp_data: &[String]
) -> Result<Vec<Company>, Status> {

    let companys_dto = sqlx::query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/get/companys_by_inn_kpp.sql",
            &comp_inn_data[..],
            &kpp_data[..]
        ).fetch_all(&state.pool_long).await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;


    dto_to_company_vec(companys_dto)

}