use chrono::format::parse;

use crate::sql_models::session;
use crate::{ClientState, ProcessError, Status};

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

    let session = state
        .get_session()
        .await
        .map_err(|err| err.process_err(err, ""))?;

    if let Some(ctrpty_inn) = parsed_block.block_fields.pay_inn.as_ref() {
        if ctrpty_inn != session.session_user.company.comp_inn ||
            parsed_block.block_fields.pay_kpp != session.session_user.company.kpp {
                return make_statement_rec_operation_raw(state, parsed_block)
                    .await
                    .map_err(|err| err.process_err(err, "ext_info"));
        }
    } else {
        return make_statement_rec_operation_raw(state, parsed_block)
            .await
            .map_err(|err| err.process_err(err, "ext_info"));
    }

    if let Some(ctrpty_inn) = parsed_block.block_fields.rec_inn.as_ref() {
        if ctrpty_inn != session.session_user.company.comp_inn ||
            parsed_block.block_fields.rec_kpp != session.session_user.company.kpp {
                return make_statement_pay_operation_raw(state, parsed_block)
                    .await
                    .map_err(|err| err.process_err(err, ""));
            }
    } else {
        return make_statement_pay_operation_raw(state, parsed_block)
            .await
            .map_err(|err| err.process_err(err, ""));
    }

    make_statement_home_operation_raw(state, parsed_block)
        .await
        .map_err(|err| err.process_err(err, &format!("ext_info = {:?}", parsed_block)))



}


pub async fn make_statement_pay_operation_raw(
    state: &ClientState,
    parsed_block: &ParsedBlock
) -> Result<OperationRaw, Status> {

    let session = state
        .get_session()
        .await
        .map_err(|err| err.process_err(err, ""))?;

    let ParsedBlock { block_fields, comment_data } = parsed_block;

    let oper_id = BoxUuid::unchecked(uuid::Uuid::new_v4());

    let user_id = session.session_user.user.user_id.clone();

    let comp_id = session.session_user.company.comp_id.clone();

    let ctrpty_option = get_company_by_inn_kpp(
            state,
            &block_fields.rec_inn,
            &block_fields.rec_kpp)
        .await
        .map_err(|err| err.process_err(err, ""))?;
    
    let contracts = if let Some(ctrpty) = ctrpty_option.as_ref() {
        get_contracts_by_comp_ctrpty_ids (
            state,
            &comp_id,
            &ctrpty.comp_id
        ).await
        .map_err(|err| err.process_err(err, ""))?
    } else {
        vec!()
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

    let external_id = if let Some(ctrpty) = ctrpty_option.as_ref() {
        hasher.update(doc_num.as_bytes());
        hasher.update(b"|");
        hasher.update(doc_date.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(amount.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(ctrpty.comp_id.as_ref().as_bytes());

        let hash = hasher.finalize();

        let bytes: [u8; 8] = hash.as_bytes()[..8].try_into().unwrap();
        
        let id: i64 = i64::from_le_bytes(bytes);
        Some(id)
    } else {
        None
    };


    let is_sync = Some(false);

    let res = OperationRaw {
        oper_id,
        user_id,

        comp_id,
        ctrpty: ctrpty_option,
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

    let session = state
        .get_session()
        .await
        .map_err(|err| err.process_err(err, ""))?; 

    let ParsedBlock { block_fields, comment_data } = parsed_block;

    let oper_id = BoxUuid::unchecked(uuid::Uuid::new_v4());

    let user_id = session.session_user.user.user_id.clone();

    let comp_id = session.session_user.company.comp_id.clone();

    let ctrpty_option = get_company_by_inn_kpp(
            state,
            &block_fields.pay_inn,
            &block_fields.pay_kpp)
        .await
        .map_err(|err| err.process_err(err, ""))?; 

    let contracts = if let Some(ctrpty) = ctrpty_option.as_ref() {
        get_contracts_by_comp_ctrpty_ids (
            state,
            &comp_id,
            &ctrpty.comp_id
        ).await
        .map_err(|err| err.process_err(err, ""))?
    } else {
        vec!()
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

    let external_id = if let Some(ctrpty) = ctrpty_option.as_ref() {
        hasher.update(doc_num.as_bytes());
        hasher.update(b"|");
        hasher.update(doc_date.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(amount.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(ctrpty.comp_id.as_ref().as_bytes());

        let hash = hasher.finalize();

        let bytes: [u8; 8] = hash.as_bytes()[..8].try_into().unwrap();
        
        let id: i64 = i64::from_le_bytes(bytes);
        Some(id)
    } else {
        None
    };

    let is_sync = Some(false);


    let res = OperationRaw {
        oper_id,
        user_id,

        comp_id,
        ctrpty: ctrpty_option,
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

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?;

    let oper_id= BoxUuid::unchecked(uuid::Uuid::new_v4());

    let user_id = session.session_user.user.user_id.clone();

    let comp_id = session.session_user.company.comp_id.clone();

    let ctrpty_option = Some(session.session_user.company.clone());

    let contract = ContractOption {
        current: None,
        contracts: vec!()
    };

    let (debet, credit) = if parsed_block.comment_data.is_cred_loan {
        (Account::SpecBankAcc, Account::BankAcc)
    } else if parsed_block.comment_data.is_cred_return {
        (Account::BankAcc, Account::SpecBankAcc)
    } else {
        (Account::BankAcc, Account::SpecBankAcc)
    };

    let amount = parsed_block.block_fields.statement_amount.clone();

    let oper_date = parsed_block.block_fields.pay_date.clone();

    let doc_type =parsed_block.block_fields.doc_type;

    let doc_num = parsed_block.block_fields.doc_num.clone();

    let doc_date = parsed_block.block_fields.doc_date.clone();

    let is_storno = false;

    let is_del = false;

    let entr_date = Date::unchecked(chrono::Utc::now().naive_utc());

    let mut hasher = blake3::Hasher::new();

    let external_id = if let Some(ctrpty) = ctrpty_option.as_ref() {
        hasher.update(doc_num.as_bytes());
        hasher.update(b"|");
        hasher.update(doc_date.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(amount.to_string().as_bytes());
        hasher.update(b"|");
        hasher.update(ctrpty.comp_id.as_ref().as_bytes());

        let hash = hasher.finalize();

        let bytes: [u8; 8] = hash.as_bytes()[..8].try_into().unwrap();
        
        let id: i64 = i64::from_le_bytes(bytes);
        Some(id)
    } else {
        None
    };

    let is_sync = Some(false);

    let res = OperationRaw {
        oper_id,
        user_id,

        comp_id,
        ctrpty: ctrpty_option,
        contract,

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

        comment: parsed_block.block_fields.doc_comment.clone(),

        is_duplicate : false
    };

    Ok(res)
}