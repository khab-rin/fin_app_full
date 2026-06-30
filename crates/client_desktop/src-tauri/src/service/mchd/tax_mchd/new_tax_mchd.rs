use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, NewMchdData, MchdInfo};

use crate::state::ClientState;
use crate::service::mchd::helper::check_update_user;
use crate::sql_queries::persons::insert::person_no_sync::insert_person_no_sync;
use crate::service::mchd::tax_mchd::make_tax_poa::make_tax_poa;
use crate::service::mchd::tax_mchd::make_tax_success::make_mchd_step_tax_success;


pub(crate) async fn make_new_tax_mchd(
    state: &ClientState,
    data: &NewMchdData
) -> Result<MchdStep, Status> {

    let failed_result = MchdStep::TryLater { text:MchdInfo::ClientServiceError };

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "FUN make_new_tax_mchd FAILED BY MISS Session, err = {}", err
            );
            return Ok(failed_result);
        }
    };

    let mut person = session.session_user.person.clone();

    if !check_update_user(&mut person, data) {
        
        return Ok(MchdStep::WrongData { text: MchdInfo::WrongPerson })
    }

    match state.update_person(person.clone()).await {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "FUN make_new_tax_mchd FAILED BY state.update_person, err = {}", err
            );
            return Ok(failed_result);
        }
    }

    match insert_person_no_sync(state, &person).await {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "FUN make_new_tax_mchd FAILED BY insert_person_no_sync, err = {}", err
            );
            return Ok(failed_result);
        }
    }

    let poa_mchd = match make_tax_poa(&session, data) {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN make_new_tax_mchd FAILED BY FUN make_tax_poa, err = {}", err
            );
            return Ok(failed_result);
        }
    };

    make_mchd_step_tax_success(&session, &poa_mchd, data)
    
}