
use std::collections::HashSet;

use shared_lib::Status;

use shared_lib::service::mchd::home_mchd_power::HomeMchdPower;
use shared_lib::service::mchd::implements::PoaRootKind;
use shared_lib::service::mchd::service::{MchdStep, MchdInfo};
use shared_lib::primitives::frozen::implements::{BoxUuid, Date};


use crate::config::BackApiState;
use crate::db::sql_queries::users::get::guides_by_id::get_guids_by_user_id;
use crate::db::sql_queries::users::set::del_guids::del_guids_by_user_id;
use crate::db::service::mchd::mchd_storage::{
    get_mchd_storage,
    write_mchd_storage_to_file
};

pub(crate) async fn show_powers(
    state: &BackApiState,
    user_id: &BoxUuid
) -> Result<MchdStep, Status> {

    let failed_result = MchdStep::TryLater { text: MchdInfo::BackApiError };

    let mut fns:HashSet<HomeMchdPower> =  HashSet::new();
    let mut btb:HashSet<HomeMchdPower> =  HashSet::new();
    let mut home_powers:HashSet<HomeMchdPower> =  HashSet::new();
    
    let guids_option = match get_guids_by_user_id(state, user_id).await {
        Ok(o) => o,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN show_powers FAILED BY FUN get_guids_by_user_id"
            );
            return Ok(failed_result);
        }
    };

    let guids = match guids_option {
        Some(g) => g,
        None => return Ok(MchdStep::ShowPowers { 
            fns: HashSet::new(), 
            btb: HashSet::new(), 
            home: HashSet::new(), 
            text: MchdInfo::ShowPowers 
        })
    };

    let mut storage = match get_mchd_storage() {
        Ok(s) => s,
        Err(err) => {
            tracing::error!(
                local_err = ?err,
                "FUN show_powers FAILED BY FUN get_mchd_storage"
            );
            return Ok(failed_result);
        }
    };

    let mut del_guids: Vec<BoxUuid> = vec!();

    let today = Date::unchecked(chrono::Local::now().date_naive());

    for guid in guids{

        let poa_option = storage.storage.get(&guid);

        let poa = match poa_option {
            Some(p) => p,
            None => {
                del_guids.push(guid);
                continue;
            }
        };

        let root_poa = match &poa.poa.poa_doc {
            PoaRootKind::RootPoa(r) => r.as_ref(),
            _ => {
                del_guids.push(guid.clone());
                continue;
            }
        }; 

        
        let poa_end_data = root_poa.poa_metadata.life_date.clone();

        if today > poa_end_data {
            del_guids.push(guid);
            continue;
        }

        let text_powers = &root_poa.delegate_powers.power_text;

        if let Some(text) = text_powers {
            for power in text.as_ref().split(" -*- ") {
                let home_power = HomeMchdPower::parse_from_str(power.trim());
                home_powers.insert(home_power);
            }
        }

        for power in &root_poa.delegate_powers.mchd_powers {
            let home_power = HomeMchdPower::parse_from_str(power.powers_code.as_ref());
            if poa.tax_file_identificator.is_some() {
                fns.insert(home_power);
            } else {
                btb.insert(home_power);
            }
        };

    }

    if !del_guids.is_empty() {

        for g in del_guids.iter() {
            let _ = storage.storage.remove(g);
        }

        match write_mchd_storage_to_file(storage) {
            Ok(_) => {}
            Err(err) => {
                tracing::error!(
                    local_err = ?err,
                    "FUN show_powers FAILED BY FUN write_mchd_storage_to_file"
                );
                return Ok(failed_result);
            }
        }

        match del_guids_by_user_id(state, user_id, &del_guids).await {
            Ok(_) => {},
            Err(err) => {
                tracing::error!(
                    local_err = ?err,
                    "FUN show_powers FAILED BY FUN del_guids_by_user_id"
                );
                return Ok(failed_result);
            }
        }
    }
    
    Ok(MchdStep::ShowPowers { 
            fns, 
            btb, 
            home: home_powers, 
            text: MchdInfo::ShowPowers 
        })

}


