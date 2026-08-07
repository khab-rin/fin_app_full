use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid,Email,Phone};


use crate::config::BackApiState;

pub(crate) async fn get_passw_rel_mail_by_pers_comp_id(
    state: &BackApiState, 
    comp_id: &BoxUuid,
    pers_id: &BoxUuid
) -> Result<Option<(String, Phone, Email)>, Status> {

    let record_option = sqlx::query_file!(
            "src/db/sql_queries/users/get/passw_tel_mail_by_pers_comp_id.sql",
            comp_id.as_ref(),
            pers_id.as_ref()
        ).fetch_optional(&state.pool_fast)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 
    
    match record_option {
        Some(r) => Ok(Some((r.password_hash, r.phone, r.email))),
        None => Ok(None)
    }


}