use crate::{Status, ProcessError};
use crate::primitives::frozen::text::{BoxUuid, Date};
use crate::primitives::frozen::text_base::Digits7_7;
use crate::service::auth_service::general::ActiveSession;
use crate::service::mchd::service::{NewMchdData, MchdType};
use crate::static_data::mchd_powers::document_propertys::{MCHD_KND};
use crate::service::mchd::implements::{
    PoaRootKind, PoaWrap, RootPoa
};


use crate::client::mchd::make_poa_elems::poa_metadata::make_poametadata;
use crate::client::mchd::make_poa_elems::principal_wrap::make_principal_wrap;
use crate::client::mchd::make_poa_elems::delegate_wrap::make_delegate_wrap;
use crate::client::mchd::make_poa_elems::powers::make_delegate_powers;


pub fn make_poa_wrap(
    session: &ActiveSession,
    data: &NewMchdData,
    mchd_num: &BoxUuid,
    today: &Date
) -> Result<PoaWrap, Status> {

    let code_knd = match data.mchd_type {
        MchdType::FnsMchd => Some(Digits7_7::unchecked(MCHD_KND)),
        _ => None
    };

    let poa_wrap = PoaWrap {
        code_knd,
        poa_doc: PoaRootKind::RootPoa(Box::new(make_root_poa(session, data, mchd_num, today)?))
    };

    Ok(poa_wrap)
}

pub fn make_root_poa(
    session: &ActiveSession,
    data: &NewMchdData,
    mchd_num: &BoxUuid,
    today: &Date
) -> Result<RootPoa, Status> {

    let poa_metadata = make_poametadata(session, data, mchd_num, today);

    let principal_wrap = make_principal_wrap(session, data)
        .map_err(|err| err.process_err(err, ""))?; 

    let delegate_powers = make_delegate_powers(data);

    let delegate_wrap = make_delegate_wrap(data);

    

    let root_poa = RootPoa {
        poa_metadata,
        principal: vec!(principal_wrap),
        delegate: vec!(delegate_wrap),
        delegate_powers,
        notarial_certification: None
    };

    Ok(root_poa)
}