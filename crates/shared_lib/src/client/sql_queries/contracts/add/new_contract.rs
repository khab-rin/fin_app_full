
use crate::primitives::frozen::text::{Date};
use crate::sql_models::id_rules::IdRules;
use crate::{Status, ClientState, ProcessError, LL};
use crate::sql_models::contracts::implements::{Contract, NewContrData};


pub async fn make_new_contract(
    state: &ClientState,
    data: NewContrData
) -> Result<Contract, Status> {
    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?;

	let comp_id = session.session_user.company.comp_id.clone();

	let rule = IdRules::Contract { comp_id: comp_id.clone(), ctrpty_id: data.ctrpty_id.clone(), contract_num: data.contract_num.clone(), contrac_date: data.contract_date.clone() };
	let contract_id = rule.make_stat_id();

    let user_id = session.session_user.user.user_id.clone();
    
    let entr_date = Date::unchecked(chrono::Local::now().date_naive());

    let contract = Contract {
        contract_id,
        user_id,
        comp_id,
        ctrpty_id: data.ctrpty_id,
        contract_num: data.contract_num,
        contract_date: data.contract_date,
        title: data.contract_title,
        st_date: data.contract_st_date,
        end_date: data.contract_end_date,
        currency: data.contract_currency,
        total_amount: data.contract_tot_amnt,
        payment_deferral_days: data.contract_def_days,
        is_active: true,
        descrip: data.contract_descr,
        entr_date,
        is_del: false,
    };

    Ok(contract)

}