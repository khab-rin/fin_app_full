use shared_lib::primitives::frozen::text::BoxUuid;
use shared_lib::service::mchd::home_mchd_power::HomeMchdPower;
use shared_lib::service::mchd::implements::{RootPoa, PoaRootKind};
use shared_lib::{Status, ProcessError};
use shared_lib::service::reports::service::VerifyPowersResult;

use crate::config::BackApiState;

use crate::db::sql_queries::users::get::guides_by_id::get_guids_by_user_id;
use crate::db::service::mchd::mchd_storage::get_mchd_storage;



pub async fn verify_user_power_exist(
	state: &BackApiState,
	user_id: BoxUuid,
	verifying_power: HomeMchdPower
) -> Result<VerifyPowersResult, Status> {

	let storage = get_mchd_storage()
		.await
		.map_err(|err| err.process_err(err, ""))?;

	if storage.managers.contains(&user_id) {
		return Ok(VerifyPowersResult {
			is_manager: true,
			mchd_uuid: None
		})
	}

	let guides = get_guids_by_user_id(state, &user_id)
		.await
		.map_err(|err| err.process_err(err, ""))?;

	for g in guides {
		let Some(mchd) = storage.storage.get(&g) else {continue;};
		match &mchd.poa.poa_doc {
			PoaRootKind::RootPoa(poa) => {
				let mchd_powers = &poa.delegate_powers.mchd_powers;
				for power in mchd_powers {
					if power.powers_code.as_str() == verifying_power.get_power_info().code {
						return Ok(VerifyPowersResult { is_manager: false, mchd_uuid: Some(g.clone()) });
					}
				}
			},
			_ => continue
		}
	}



	Ok(VerifyPowersResult{is_manager: false, mchd_uuid: None})
	
}