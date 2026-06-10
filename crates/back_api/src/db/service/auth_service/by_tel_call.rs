use std::sync::Arc;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{ 
    ExternalDeviceData, 
    SessionUserToken, 
    AuthStep,
    TextInfo
};

use crate::config::BackApiState;
use crate::db::sql_queries::call_cf::get::by_extern_device::get_user_time_by_device_external;
use crate::db::service::auth_service::smsru_cf_query::smsru_get_cf;
use crate::db::sql_queries::sessions::set::new_session::new_session;
use crate::db::sql_queries::users::get::by_user_id::get_user_by_user_id;


pub(crate) async fn make_session_by_tel_call(
    state: &Arc<BackApiState>,
    data: &ExternalDeviceData
) -> Result<AuthStep, Status> {

    let ExternalDeviceData {external_id, device_id} = data;

    let expire_option = match get_user_time_by_device_external(state, data).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN make_session_by_tel_call FAILED BY FUN get_user_time_by_device_external"
            );
            return Ok(AuthStep::TryLater { text: TextInfo::BackApiError });
        }
    };

    let (user_id, expires_t) = match expire_option {
        Some((a, b)) => (a, b),
        None => return Ok(AuthStep::NeedRegistration { text: TextInfo::MissUserNeedRegistration })
    };


    let phone_cf = match smsru_get_cf(state, &expires_t, external_id).await {
        Ok(cf) => cf,
        Err(err) => {
            tracing::error!(
                user_id = %user_id,
                err = ?err,
                "FUN restore_user_by_tel_call FAILED IN CALL FUN smsru_get_cf"
            );
            return Ok(AuthStep::TryLater {text: TextInfo::BackApiError});
        }
    };

    if !phone_cf {
        return Ok(AuthStep::NeedPassword {text: TextInfo::SmsRuCallMiss});
    }

    let token = match new_session(state, &user_id, device_id).await {
        Ok(t) => t,
        Err(err) => {
            tracing::error!(
                user_id = %user_id,
                err = ?err,
                "FUN restore_user_by_tel_call FAILED BY FUN new_session"
            );
            return Ok(AuthStep::TryLater {text: TextInfo::BackApiError});
        }
    };

    let session_user = match get_user_by_user_id(state, &user_id).await {
        Ok(u) => u,
        Err(err) => {
            tracing::error!(
                user_id = %user_id,
                err = ?err,
                "FUN restore_user_by_tel_call FAILED BY FUN get_user_by_user_id"
            );
            return Ok(AuthStep::TryLater {text: TextInfo::BackApiError});
        }
    };

    let session_user_token = SessionUserToken {user: session_user, token};

    Ok(AuthStep::SuccessFull { session_user_token: Box::new(session_user_token)})

}