use std::sync::Arc;

use shared_lib::service::auth_service::implements::{
    PasswordDataClient,
    PasswordDataBackApi
};
use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, Phone};

use crate::config::BackApiState;


pub(crate) async fn get_restore_password_data(
    state: &Arc<BackApiState>,
    data: &PasswordDataClient
) -> Result<Option<PasswordDataBackApi>, Status> {

    let PasswordDataClient {  
        device_id, 
        pers_inn, 
        comp_inn, 
        kpp, .. } = data;

    match sqlx::
        query_file_as!(
            PasswordDataBackApi,
            "src/db/sql_queries/users/get/auth_check_passw_by_authdata.sql",
            pers_inn.as_ref(),
            comp_inn.as_ref(),
            kpp.as_ref(),
            device_id.as_ref()
        ).fetch_optional(&state.pool_fast).await {
            Ok(o) => Ok(o),
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_err = ?Status::SqlQueryWrongLogic,
                    "FUN get_restore_password_data FAILED BY SQL QUERY"
                );
                Err(Status::SqlQueryWrongLogic)
            }
        }
}   