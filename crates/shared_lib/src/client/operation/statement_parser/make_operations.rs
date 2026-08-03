use crate::{Status, ClientState};

use crate::primitives::frozen::text::{BoxUuid, Date};
use crate::sql_models::operation::parser::ParsedBlock;
use crate::sql_models::operation::implements::{
    ContractOption, OperationRaw
};
use crate::sql_models::operation::account::Account;

use crate::client::sql_queries::companys::get::by_inn_kpp::get_company_by_inn_kpp;
use crate::client::sql_queries::contracts::get::contracts_by_ids::get_contracts_by_comp_ctrpty_ids;

pub async fn make_statement_operation_raw(
    state: &ClientState,
    parsed_block: &ParsedBlock
) -> Result<OperationRaw, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN make_statement_operation_raw FAILED BY MISS SESSION",
                err
            );
            return Err(err);
        }
    };

    if parsed_block.block_fields.pay_inn == session.session_user.company.comp_inn &&
        parsed_block.block_fields.pay_kpp == session.session_user.company.kpp {
            make_statement_pay_operation_raw(state, parsed_block).await
        } else if parsed_block.block_fields.rec_inn == session.session_user.company.comp_inn &&
            parsed_block.block_fields.rec_kpp == session.session_user.company.kpp {
            make_statement_rec_operation_raw(state, parsed_block).await
        } else {
            make_statement_home_operation_raw(state, parsed_block).await
        }

}


pub async fn make_statement_pay_operation_raw(
    state: &ClientState,
    parsed_block: &ParsedBlock
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


    let ctrpty_option = match get_company_by_inn_kpp(
            state,
            &block_fields.rec_inn,
            &block_fields.rec_kpp).await {
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

    let contracts = match get_contracts_by_comp_ctrpty_ids(state, &comp_id, &ctrpty.comp_id).await {
        Ok(c) => c,
        Err(err) => {
            log::error!(
            "local_err = {:?}, FUN make_statement_pay_operation_raw FAILED BY FUN get_contracts_by_comp_ctrpty_ids", err
            );
            return Err(err);
        }
    };

    let mut contrac_option = ContractOption {
        current: None,
        contracts
    };

    'outer: for num in comment_data.doc_num.iter() {
        for contract in contrac_option.contracts.iter() {
            if num == &contract.contract_num {
                contrac_option.current = Some(contract.clone());
                break 'outer;
            }
        }
    }

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
    
    let amount = block_fields.statement_amount.clone();

    let oper_date = block_fields.pay_date.clone();

    let doc_type = block_fields.doc_type;

    let doc_num = block_fields.doc_num.clone();

    let doc_date = block_fields.doc_date.clone();

    let is_storno = false;

    let is_del = false;

    let entr_date = Date::unchecked(chrono::Utc::now().naive_utc());

    let mut hasher = blake3::Hasher::new();

    hasher.update(doc_num.as_bytes());
    hasher.update(b"|");
    hasher.update(doc_date.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(amount.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(ctrpty.comp_id.as_ref().as_bytes());

    let hash = hasher.finalize();

    let bytes: [u8; 8] = hash.as_bytes()[..8].try_into().unwrap();
    
    let external_id: i64 = i64::from_le_bytes(bytes);

    let is_sync = Some(false);


    let res = OperationRaw {
        oper_id,
        user_id,

        comp_id,
        ctrpty,
        contract: contrac_option,

        debet,
        credit,
        amount,
        oper_date,

        doc_type,
        doc_num,
        doc_date,

        is_storno,
        is_del,

        entr_date,

        external_id,

        is_sync,

        comment: block_fields.doc_comment.clone(),

        is_duplicate: false
    };

    Ok(res)
}



pub async fn make_statement_rec_operation_raw(
    state: &ClientState,
    parsed_block: &ParsedBlock
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


    let ctrpty_option = match get_company_by_inn_kpp(
            state,
            &block_fields.pay_inn,
            &block_fields.pay_kpp).await {
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

    let contracts = match get_contracts_by_comp_ctrpty_ids(state, &comp_id, &ctrpty.comp_id).await {
        Ok(c) => c,
        Err(err) => {
            log::error!(
            "local_err = {:?}, FUN make_statement_pay_operation_raw FAILED BY FUN get_contracts_by_comp_ctrpty_ids", err
            );
            return Err(err);
        }
    };

    let mut contrac_option = ContractOption {
        current: None,
        contracts
    };

    'outer: for num in comment_data.doc_num.iter() {
        for contract in contrac_option.contracts.iter() {
            if num == &contract.contract_num {
                contrac_option.current = Some(contract.clone());
                break 'outer;
            }
        }
    }
    
    let debet = Account::BankAcc;

    let credit = if comment_data.is_tax {
        Account::Taxes
    } else if comment_data.is_penalty {
        Account::OtherPayables
    } else if comment_data.is_cred_loan || comment_data.is_cred_return {
        Account::ShortLoans
    } else if comment_data.is_komis {
        Account::OtherPayables
    } else {
        Account::Customers
    };
    
    let amount = block_fields.statement_amount.clone();

    let oper_date = block_fields.pay_date.clone();

    let doc_type = block_fields.doc_type;

    let doc_num = block_fields.doc_num.clone();

    let doc_date = block_fields.doc_date.clone();

    let is_storno = false;

    let is_del = false;

    let entr_date = Date::unchecked(chrono::Utc::now().naive_utc());

    let mut hasher = blake3::Hasher::new();

    hasher.update(doc_num.as_bytes());
    hasher.update(b"|");
    hasher.update(doc_date.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(amount.to_string().as_bytes());
    hasher.update(b"|");
    hasher.update(ctrpty.comp_id.as_ref().as_bytes());

    let hash = hasher.finalize();

    let bytes: [u8; 8] = hash.as_bytes()[..8].try_into().unwrap();
    
    let external_id: i64 = i64::from_le_bytes(bytes);



    let is_sync = Some(false);


    let res = OperationRaw {
        oper_id,
        user_id,

        comp_id,
        ctrpty,
        contract: contrac_option,

        debet,
        credit,
        amount,
        oper_date,

        doc_type,
        doc_num,
        doc_date,

        is_storno,
        is_del,

        entr_date,

        external_id,

        is_sync,

        comment: block_fields.doc_comment.clone(),

        is_duplicate : false
    };

    Ok(res)
}

pub async fn make_statement_home_operation_raw(
    state: &ClientState,
    parsed_block: &ParsedBlock
) -> Result<OperationRaw, Status> {

    


    Err(Status::Unknown)
}