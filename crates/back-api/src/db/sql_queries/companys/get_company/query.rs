use shared_lib::Status;
use shared_lib::sql_models::company_models::implements::{Company, CompanyDto};
use shared_lib::primitives::frozen::implements::{BoxUuid, Inn, Kpp, CompStatus, CompType, DateTime};

use crate::db::sql_queries::companys::helper::dto_to_company_vec;


pub(crate) async fn get_company_by_id(
    pool: &sqlx::PgPool,
    id_data: &[uuid::Uuid]
) -> Result<Vec<Company>, Status> {

    let companys_dto = sqlx::
        query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/get_company/get_by_id.sql",
            id_data
        ).fetch_all(pool)
        .await
        .inspect(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::BackGetCompanyQuery
            )
        }).map_err(|_| Status::BackGetCompanyQuery)?;

    Ok(dto_to_company_vec(companys_dto))
}