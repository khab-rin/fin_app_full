use shared_lib::service::mchd::home_mchd_power::HomeMchdPower;
use shared_lib::service::mchd::service::{NewMchdData, MchdType};
use shared_lib::service::mchd::implements::{
    DelegatePowers,
    PowerType,
    PowerCommonType,
    MchdPower
};


pub(crate) fn make_delegate_powers(
    data: &NewMchdData
) -> DelegatePowers {

    let power_type = match data.mchd_type {
        MchdType::HomeMchd => PowerType::Textual,
        _ => PowerType::MachineReadable
    };

    let power_text = match data.mchd_type {
        MchdType::HomeMchd => Some(HomeMchdPower::make_text_power(&data.powers)),
        _ => None
    };

    let mchd_powers: Vec<MchdPower> = match data.mchd_type {
        MchdType::HomeMchd => vec!(),
        _ => data.powers.iter().map(|x| x.clone().make_mchd_power()).collect()
    };

    DelegatePowers {
        power_type,
        power_common_type: PowerCommonType::Individual,
        redelegate_power_loss: None,
        power_text,
        mchd_powers
    }
}