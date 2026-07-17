use shared_lib::Status;
use shared_lib::sql_models::person::implements::Person;
use shared_lib::service::auth_service::client_state::{NickData, UserLogInfo};
use shared_lib::service::auth_service::implements::{RegInitData, InitFiles};

use crate::service::auth_service::nick_data::add_nick_data;
use crate::service::auth_service::key_ring::{write_keyring_data, get_keyring_data};
use crate::state::ClientState;

pub(crate) async  fn make_init_files(
    state: &ClientState,
    data: &RegInitData
) -> Result<InitFiles, Status> {

    let RegInitData { 
        sur_name, 
        first_name, 
        mid_name, 
        pers_inn, 
        snils, 
        comp_inn, 
        kpp, 
        phone, 
        email ,
        password
    } = data;

    

    Err(Status::Unknown)
}