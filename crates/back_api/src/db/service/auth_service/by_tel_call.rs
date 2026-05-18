use std::sync::Arc;

use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    RegisterResponse, RestoreByTelCallRequest, SessionUserToken, SmsruCallResponse, VerifyData, VerifyMethod};

use crate::config::ApiState;
use crate::db::sql_queries::call_cf::get::by_extern_device::get_user_time_by_device_external;
use crate::db::service::auth_service::smsru_cf_query::smsru_get_cf;
use crate::db::sql_queries::sessions::set::new_session::new_session;
use crate::db::sql_queries::users::get_user::by_user_id::get_user_by_user_id;


pub(crate) async fn restore_user_by_tel_call(
    state: &Arc<ApiState>,
    data: &RestoreByTelCallRequest
) -> Result<RegisterResponse, Status> {
    let failed_result = Ok(RegisterResponse::
        Verify(VerifyData { 
            device_id: data.device_id.clone(), 
            method: VerifyMethod::TryLater {} 
        }));

    let RestoreByTelCallRequest {external_id, device_id} = data;

    let (user_id, expires_t) = match get_user_time_by_device_external(state, data).await {
        Ok(wrap) => wrap,
        Err(err) => {
            tracing::error!(
                data = ?data,
                tech_err = ?err,
                "FUN restore_user_by_tel_call FAILED BY FUN get_user_time_by_device_external"
            );
            return failed_result;
        }
    };

    let phone_cf = match smsru_get_cf(state, &expires_t, external_id).await {
        Ok(cf) => cf,
        Err(err) => {
            tracing::error!(
                user_id = %user_id,
                err = ?err,
                "FUN restore_user_by_tel_call FAILED IN CALL FUN smsru_get_cf"
            );
            return Ok(RegisterResponse::Verify(VerifyData { 
                device_id: device_id.clone(), 
                method: VerifyMethod::TryLater {} 
            }));
        }
    };

    if !phone_cf {
        return Ok(RegisterResponse::Verify(VerifyData { 
                device_id: device_id.clone(), 
                method: VerifyMethod::NeedPassword {} 
            }));
    }

    let token = match new_session(state, &user_id, device_id).await {
        Ok(t) => t,
        Err(err) => {
            tracing::error!(
                user_id = %user_id,
                err = ?err,
                "FUN restore_user_by_tel_call FAILED BY FUN new_session"
            );
            return failed_result;
        }
    };

    let user = match get_user_by_user_id(state, &user_id).await {
        Ok(u) => u,
        Err(err) => {
            tracing::error!(
                user_id = %user_id,
                err = ?err,
                "FUN restore_user_by_tel_call FAILED BY FUN get_user_by_user_id"
            );
            return failed_result;
        }
    };

    let res = SessionUserToken {user,token};

    Ok(RegisterResponse::Success(Box::new(res)))

}