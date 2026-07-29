use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use crate::state::ClientState;

use shared_lib::Status;
use shared_lib::primitives::frozen::text::{BoxUuid, Date};
use shared_lib::parsers::bank_statement::implements::{ParsedBlock};
use shared_lib::service::auth_service::client_state::ActiveSession;
use shared_lib::sql_models::operation::implements::{
    Operation, 
    OperationRaw
};
use shared_lib::sql_models::operation::account::Account;

use crate::sql_queries::companys::get::by_inn_kpp::get_company_by_inn_kpp;


pub(crate) async fn make_statement_operation(
    state: &ClientState,
    parsed_block: &ParsedBlock
) -> Result<Operation, Status> {




    Err(Status::Unknown)

}


pub(crate) async fn make_statement_pay_operation_raw(
    state: &ClientState,
    parsed_block: ParsedBlock
) -> Result<OperationRaw, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN make_statement_pay_operation_raw FAILED BY MISS SESSION",
                err
            );
            return Err(err);
        }
    };

    let ParsedBlock { block_fields, comment_data } = parsed_block;

    let oper_id = BoxUuid::unchecked(uuid::Uuid::new_v4());

    let user_id = session.session_user.user.user_id.clone();

    let comp_id = session.session_user.company.comp_id.clone();




    let inn_kpp = (block_fields.rec_inn, block_fields.rec_kpp);

    let ctrpty_option = match get_company_by_inn_kpp(state, &inn_kpp).await {
        Ok(o) => o,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN make_statement_pay_operation_raw FAILED BY FUN get_companys_by_inn_kpp", err
            );
            return Err(err);
        }
    };

    let ctrpty = match ctrpty_option {
        Some(c) => c,
        None => {
            log::error!(
                "local_err = {:?}, FUN make_statement_pay_operation_raw FAILED BY WRONG SYSTEM LOGIC",
                Status::SystemLogicErr
            );
            return Err(Status::SystemLogicErr);

        }
    };

    let ctrpty_id = ctrpty.comp_id;

    let debet = if comment_data.is_tax {
        Account::Taxes
    } else if comment_data.is_salary {
        Account::Payroll
    } else if comment_data.is_komis {
        Account::OtherIncome
    } else {
        Account::Vendors
    };
     
    let credit = Account::BankAcc;
    
    let amount = block_fields.statement_amount;

    let oper_date = block_fields.pay_date;

    let doc_type = block_fields.doc_type;

    let doc_num = block_fields.doc_num;

    let doc_date = block_fields.doc_date;

    let is_storno = false;

    let is_del = false;

    let entr_date = Date::unchecked(chrono::Utc::now().naive_utc());

    // let external_id = {

    //     let mut hasher = DefaultHasher::new();
        
    //     block_fields.doc_num.hash(&mut hasher);
    //     block_fields.doc_date.hash(&mut hasher);
    //     block_fields.statement_amount.to_string().hash(&mut hasher);
    //     ctrpty_id.hash(&mut hasher);

    //     hasher.finish() as i64
    // };

    let is_sync = Some(false);


    // let res = OperationRaw {
    //     oper_id,
    //     user_id,

    //     comp_id,
    //     ctrpty,
    //     contract: ,

    //     debet,
    //     credit,
    //     amount,
    //     oper_date,

    //     doc_type,
    //     doc_num,
    //     doc_date,

    //     is_storno,
    //     is_del,

    //     entr_date,

    //     external_id,

    //     is_sync,
    // };

    Err(Status::Unknown)
}