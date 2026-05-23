use std::sync::Arc;

use shared_lib::Status;
use shared_lib::sql_models::company::implements::{Company, CompanyDto};
use shared_lib::primitives::frozen::implements::{BoxUuid, Inn, Kpp, CompType, CompStatus};

use crate::config::BackApiState;
use crate::db::sql_queries::companys::helper::dto_to_company_vec;

pub(crate) async fn get_companys_by_inn_kpp(
    state: &Arc<BackApiState>,
    data: &(Vec<String>, Vec<String>)

) -> Result<Vec<Company>, Status> {
    
    let (inn_data, kpp_data) = data;

    let companys_dto = match sqlx::
        query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/get/companys_by_inn_kpp.sql",
            &inn_data[..],
            &kpp_data[..]
        ).fetch_all(&state.pool).await {
            Ok(d) => d,
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::SqlCompanysQueryLogicErr,
                );
                return Err(Status::SqlCompanysQueryLogicErr);
            }
        };

    Ok(dto_to_company_vec(companys_dto))

}