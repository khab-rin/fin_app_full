use crate::{ProcessError, Status};
use crate::primitives::frozen::text::BoxUuid;
use crate::service::auth_service::implements::SessionUserToken;
use crate::service::auth_service::client_state::NickData;

use crate::ClientState;
use crate::client::auth_service::nick_data::add_nick_data;
use crate::client::auth_service::key_ring::write_keyring_token;


pub fn get_device_id() -> Result<BoxUuid, Status> {

    let id_string = machine_uid::get()
        .map_err(|err| err.process_err(Status::SystemErr, ""))?; 

    let id_uuid_str = uuid::Uuid::new_v5(
        &uuid::Uuid::NAMESPACE_DNS,
        id_string.as_bytes()
    ).to_string();

    BoxUuid::new(&id_uuid_str)
}

pub fn write_new_user_info_to_device(
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

    add_nick_data(state, &nick_data)
        .map_err(|err| err.process_err(err, ""))?;

    write_keyring_token(state, &key_, &token)
        .map_err(|err| err.process_err(err, ""))?;


    Ok(())
}

