


use crate::primitives::frozen::text::{BoxUuid, Date};
use crate::{Status, ClientState, ProcessError};
use crate::sql_models::contracts::implements::{Contract, NewContrData};

use crate::client::sql_queries::contracts::get::contract_by_ids::get_contract_by_contr_id;

pub async fn make_new_contract(
    state: &ClientState,
    data: NewContrData
) -> Result<Contract, Status> {
    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?;


    let id_str = format!("{}_{}_{:?}",
        data.contract_num,
        data.contract_date,
        data.contract_currency,
    );

    let contract_id = BoxUuid::unchecked(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, id_str.as_bytes()));

    let exist_contract_option = get_contract_by_contr_id(state, &contract_id).await
        .map_err(|err| err.process_err(err, ""))?;

    if let Some(contract) = exist_contract_option {
        return Ok(contract);
    }

    let user_id = session.session_user.user.user_id.clone();
    let comp_id = session.session_user.company.comp_id.clone();

    let entr_date = Date::unchecked(chrono::Local::now().date_naive());



    let contract = Contract {
        contract_id,
        user_id,
        comp_id,
        ctrpty_id: data.ctrpty_id,

        contract_num: data.contract_num,

        contract_date: data.contract_date,
        title: data.contract_title,

        start_date: data.contract_st_date,
        end_date: data.contract_end_date,

        currency: data.contract_currency,

        total_amount: data.contract_tot_amnt,

        payment_deferral_days: data.contract_def_days,

        is_active: 1,

        description: data.contract_descr,

        entr_date,

        is_del: 0
    };

    

    Ok(contract)

}