use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid,Email,Phone};


use crate::config::BackApiState;

pub(crate) async fn get_user_phone_mail_by_id(
    state: &BackApiState, 
    user_id: &BoxUuid
) -> Result<Option<(Phone, Email)>, Status> {

    let record_option = sqlx::query_file!(
            "src/db/sql_queries/users/get/tel_mail_by_id.sql",
            user_id.as_ref()
        ).fetch_optional(&state.pool_fast)
        .await 
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;
  
        
    match record_option {
        Some(r) => Ok(Some((r.phone, r.email))),
        None => Ok(None)
    }


}