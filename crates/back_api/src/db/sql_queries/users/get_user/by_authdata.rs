use std::sync::Arc;

use shared_lib::service::auth_service::implements::{
    AuthData, 
    RestoreByAuthDataRequest,
    AuthCheckPassword
};
use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, Phone};

use crate::config::BackApiState;


pub(crate) async fn get_auth_password_check(
    state: &Arc<BackApiState>,
    data: &RestoreByAuthDataRequest
) -> Result<Vec<AuthCheckPassword>, Status> {
    
    let RestoreByAuthDataRequest { auth_data, device_id } = data;
    let AuthData { pers_inn, comp_inn, kpp, .. } = auth_data;

    sqlx::
        query_file_as!(
            AuthCheckPassword,
            "src/db/sql_queries/users/get_user/by_authdata.sql",
            pers_inn.as_ref(),
            comp_inn.as_ref(),
            kpp.as_ref(),
            device_id.as_ref()
        ).fetch_all(&state.pool)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::BackAuthGetCheckPasswQueryLogic
            )
        }).map_err(|_| Status::BackAuthGetCheckPasswQueryLogic)
}   