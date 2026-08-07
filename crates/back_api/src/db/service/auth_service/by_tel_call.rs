use std::sync::Arc;

use shared_lib::{ProcessError, Status};
use shared_lib::service::auth_service::implements::{ 
    AuthStep, ExternalDeviceData, SessionUserToken, SmsRuResponseTextCode, AuthInfo
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

    let failed_result = Ok(AuthStep::TryLater { text: AuthInfo::BackApiError });
    let ExternalDeviceData {external_id, device_id} = data;

    let expire_option = match get_user_time_by_device_external(state, data).await {
        Ok(o) => o,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };


    let (user_id, _) = match expire_option {
        Some((a, b)) => (a, b),
        None => return Ok(AuthStep::RegisterStep1 { text: AuthInfo::MissUserNeedRegistration })
    };


    let phone_cf = match smsru_get_cf(state, external_id).await {
        Ok(cf) => cf,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    match phone_cf {
        SmsRuResponseTextCode::Polling => {
            return Ok(AuthStep::CallInWaiting { text: AuthInfo::CallInWaiting });
        },
        SmsRuResponseTextCode::SuccessConfirmed => {},
        SmsRuResponseTextCode::TimeOut => {
            let res = AuthStep::Password { text: AuthInfo::CallInnTimeOut };
            return Ok(res)
        },
        SmsRuResponseTextCode::UnknownCode => {
            return failed_result
        }
    }

    let token = match new_session(state, &user_id, device_id).await {
        Ok(t) => t,
        Err(err) => {
            err.process_err(err, "");
            return Ok(AuthStep::TryLater {text: AuthInfo::BackApiError});
        }
    };

    let session_user = match get_user_by_user_id(state, &user_id).await {
        Ok(u) => u,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    let session_user_token = SessionUserToken {user: session_user, token};

    Ok(AuthStep::SuccessFull { session_user_token: Box::new(session_user_token)})

}