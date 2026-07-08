use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, Date};
use shared_lib::primitives::frozen::implements_base::Digits7_7;
use shared_lib::service::auth_service::client_state::ActiveSession;
use shared_lib::service::mchd::service::NewMchdData;
use shared_lib::static_data::mchd_powers::document_propertys::{MCHD_KND};
use shared_lib::service::mchd::implements::{
    DelegatePowers, MchdPower, PoaRootKind, PoaWrap, PowerCommonType, PowerType, RootPoa
};


use crate::service::mchd::make_poametadata::make_poametadata;
use crate::service::mchd::make_principalinfo::make_principal_info;
use crate::service::mchd::make_delegate_wrap::make_delegate_wrap;


pub(crate) fn make_poa_wrap(
    session: &ActiveSession,
    data: &NewMchdData,
    mchd_num: &BoxUuid,
    today: &Date
) -> Result<PoaWrap, Status> {

    let poa_wrap = PoaWrap {
        code_knd: Some(Digits7_7::unchecked(MCHD_KND)),
        poa_doc: PoaRootKind::RootPoa(Box::new(make_root_poa(session, data, mchd_num, today)?))
    };

    Ok(poa_wrap)
}

pub(crate) fn make_root_poa(
    session: &ActiveSession,
    data: &NewMchdData,
    mchd_num: &BoxUuid,
    today: &Date
) -> Result<RootPoa, Status> {

    let poa_metadata = make_poametadata(data, mchd_num, today);

    let principal_info = match make_principal_info(session, data) {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN make_root_poa FAILED BY FUN make_principal_info, err = {}", err
            );
            return Err(err);
        }
    };

    let delegate_wrap = make_delegate_wrap(data);

    let mut powers: Vec<MchdPower> = vec!();

    for field in data.powers.iter().cloned() {
        let power = field.make_mchd_power();
        powers.push(power);
    }

    let delegate_powers = DelegatePowers {
        power_type: PowerType::MachineReadable,
        power_common_type: PowerCommonType::Individual,
        redelegate_power_loss: None,
        power_text: None,
        mchd_powers: powers
    };

    let root_poa = RootPoa {
        poa_metadata,
        principal: vec!(principal_info),
        delegate: vec!(delegate_wrap),
        delegate_powers,
        notarial_certification: None
    };

    Ok(root_poa)
}