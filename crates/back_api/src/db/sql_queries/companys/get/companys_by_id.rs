use std::sync::Arc;

use shared_lib::Status;
use shared_lib::sql_models::company::implements::{Company, CompanyDto};
use shared_lib::primitives::frozen::implements::{BoxUuid, CompInn, Kpp, CompStatus, CompType, DateTime};

use crate::db::sql_queries::companys::helper::dto_to_company_vec;
use crate::config::BackApiState;

pub(crate) async fn get_companys_by_id(
    state: &Arc<BackApiState>,
    id_data: &[uuid::Uuid]
) -> Result<Vec<Company>, Status> {

    let companys_dto = sqlx::
        query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/get/companys_by_id.sql",
            id_data
        ).fetch_all(&state.pool_fast)
        .await
        .inspect(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::SqlQueryWrongLogic
            )
        }).map_err(|_| Status::SqlQueryWrongLogic)?;

    Ok(dto_to_company_vec(companys_dto))
}