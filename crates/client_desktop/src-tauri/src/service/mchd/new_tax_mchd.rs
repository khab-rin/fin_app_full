use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, NewMchdData};
use shared_lib::parsers::mchd::implements::*;
use shared_lib::parsers::mchd::poa::PoaMchd;


use crate::state::ClientState;

pub(crate) fn make_new_tax_mchd(
    state: &ClientState,
    data: &NewMchdData
) -> Result<MchdStep, Status> {

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