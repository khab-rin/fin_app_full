use shared_lib::Status;
use shared_lib::primitives::frozen::implements::BoxUuid;
use shared_lib::service::auth_service::implements::SessionUserToken;
use shared_lib::service::auth_service::client_state::NickData;

use crate::state::ClientState;
use crate::service::auth_service::nick_data::add_nick_data;
use crate::service::auth_service::key_ring::write_keyring_token;


pub(crate) fn get_device_id() -> Result<BoxUuid, Status> {

    let id_string = match machine_uid::get() {
        Ok(i) => i,
        Err(err) => {
            log::error!(
                "FUN get_device_id FAILED BY machine_uid::get(), tech_err = {:?}, local_err = {:?}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    let id_uuid_str = uuid::Uuid::new_v5(
        &uuid::Uuid::NAMESPACE_DNS,
        id_string.as_bytes()
    ).to_string();

    BoxUuid::new(&id_uuid_str)
}

pub(crate) fn write_new_user_info_to_device(
    state: &ClientState,
    session: &SessionUserToken
) -> Result<(), Status> {

    let pers_inn = &session.user.person.pers_inn;
    let comp_inn = &session.user.company.comp_inn;
    let kpp = &session.user.company.kpp;

    let sur_name = &session.user.person.metadata.fio.sur_name;
    let first_name = &session.user.person.metadata.fio.first_name;
    let comp_name = session.user.company.metadata.comp_name.as_ref()
        .and_then(|c| c.short_egrul_name.as_ref())
        .map(|s| s.to_string())
        .unwrap_or("Неизвестная компания".to_string());

    
    let token = session.token.clone();

    let nick = format!("{}_{}_{}", sur_name, first_name, comp_name);
    let key_ = format!("{}{}{}", pers_inn, comp_inn, kpp);

    let nick_data = NickData {
        nick,
        pers_inn: pers_inn.clone(),
        comp_inn: comp_inn.clone(),
        kpp: kpp.clone()
    };

    match add_nick_data(state, &nick_data) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN register_step2 FAILED BY FUN add_nick_data, err = {}", err);
            return Err(err);
        }
    }

    match write_keyring_token(state, &key_, &token) {
        Ok(_) => {},
        Err(err) => {
            log::error!("FUN register_step2 FAILED BY FUN write_keyring_data, err = {}", err);
            return Err(err);
        }
    };

    Ok(())
}

