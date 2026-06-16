use shared_lib::Status;
use shared_lib::service::auth_service::client_state::SessionUser;
use shared_lib::service::mchd::service::{MchdStep, MchdInfo};


use crate::state::ClientState;

pub(crate) async fn check_user_mchd_tax(
    state: &ClientState
) -> Result<MchdStep, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN check_user_mchd_tax FAILED BY state.get_session(), local_err = {}", err
            );
            return Ok(MchdStep::TryLater { text: MchdInfo::ClientServiceError });
        }
    };

    let session_user: &SessionUser = &session.session_user;
    let user = &session_user.user;
    if user.mchd_tax_guid.is_some() {
        return Ok(MchdStep::TaxMchdFull { text: MchdInfo::TaxMchdFull });
    }

    let person = &session_user.person;

    Ok(MchdStep::TaxMchdMiss { pers: person.clone(), text: MchdInfo::TaxMchdMiss })

}


pub(crate) async fn check_user_mchd_home(
    state: &ClientState
) -> Result<MchdStep, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN check_user_mchd_tax FAILED BY state.get_session(), local_err = {}", err
            );
            return Ok(MchdStep::TryLater { text: MchdInfo::ClientServiceError });
        }
    };

    let session_user: &SessionUser = &session.session_user;
    let user = &session_user.user;
    if user.mchd_home_guid.is_some() {
        return Ok(MchdStep::HomeMchdFull { text: MchdInfo::HomeMchdFull });
    }

    let person = &session_user.person;

    Ok(MchdStep::HomeMchdMiss { pers: person.clone(), text: MchdInfo::HomeMchdMiss })

}