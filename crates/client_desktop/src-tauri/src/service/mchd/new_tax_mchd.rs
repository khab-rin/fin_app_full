use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, NewMchdData, MchdInfo};
use shared_lib::parsers::mchd::implements::*;
use shared_lib::parsers::mchd::poa::PoaMchd;

use crate::state::ClientState;
use crate::service::mchd::helper::check_update_user;
use crate::service::mchd::make_poametadata::make_poametadata;
use crate::sql_queries::persons::insert::person_no_sync::insert_person_no_sync;
use crate::service::mchd::make_poa_wrap::make_poa_wrap;


pub(crate) async  fn make_new_tax_mchd(
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



    let poa_wrap = match make_poa_wrap(&session, data) {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN make_new_tax_mchd FAILED BY FUN make_poa_wrap, err = {}", err
            );
            return Ok(failed_result);
        }
    };









    

    let NewMchdData { 
        poa_number, 
        poa_end_date, 
        manager_tittle, 
        manager_sur_name, 
        manager_first_name, 
        manager_mid_name, 
        manager_birth_day, 
        manager_snils, 
        manager_inn, 
        manager_is_citizen, 
        user_sur_name, 
        user_first_name, 
        user_mid_name, 
        user_birth_day, 
        user_gender, 
        user_snils, 
        user_inn, 
        user_passport_number, 
        user_passport_issueer, 
        user_passport_ussuer_code, 
        user_is_citizen,
        powers } = data;
    
    

    Err(Status::Unknown)
}