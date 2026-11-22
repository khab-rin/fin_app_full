use crate::sql_models::company::implements::CtrPtyTypes;
use crate::sql_models::id_rules::IdRules;
use crate::{ClientState, ProcessError, Status};

use crate::primitives::frozen::text::Date;
use crate::sql_models::operation::parser::ParsedBlock;
use crate::sql_models::operation::implements::{
    ContractOption, OperationRaw
};
use crate::sql_models::operation::account::Account;

use crate::client::sql_queries::contracts::get::contract_by_ids::get_contracts_by_comp_ctrpty_ids;
use crate::client::sql_queries::companys::get::fun_try_inn_kpp_then_inn::try_get_by_inn_kpp_then_inn_function;


pub async fn make_statement_operation_raw(
    state: &ClientState,
    parsed_block: &ParsedBlock,
	ctrpty_types: &CtrPtyTypes
) -> Result<OperationRaw, Status> {

    let session = state
        .get_session()
        .await
        .map_err(|err| err.process_err(err, ""))?;

	let own_inn = &session.session_user.company.comp_inn;
	let comp_inn_pay = &parsed_block.block_fields.pay_inn;
	let comp_inn_rec = &parsed_block.block_fields.rec_inn;

	if own_inn == comp_inn_pay && own_inn == comp_inn_rec {
		return make_statement_home_operation_raw(state, parsed_block).await
	} else if own_inn == comp_inn_pay {
		return make_statement_pay_operation_raw(state, parsed_block, ctrpty_types).await
	} else {
		return make_statement_rec_operation_raw(state, parsed_block, ctrpty_types).await
	}
}


pub async fn make_statement_pay_operation_raw(
    state: &ClientState,
    parsed_block: &ParsedBlock,
	ctrpty_types: &CtrPtyTypes
) -> Result<OperationRaw, Status> {


    let session = state
        .get_session()
        .await
        .map_err(|err| err.process_err(err, ""))?;

    let ParsedBlock { block_fields, comment_data } = parsed_block;

    let user_id = session.session_user.user.user_id.clone();

    let comp_id = session.session_user.company.comp_id.clone();

	let ctrpty_option = try_get_by_inn_kpp_then_inn_function(
		state, 
		&block_fields.rec_inn, 
		&block_fields.rec_kpp
	).await
	.map_err(|err| err.process_err(err, ""))?;

    let contracts = if let Some(ctrpty) = ctrpty_option.as_ref() {
        get_contracts_by_comp_ctrpty_ids (
            state,
            &ctrpty.comp_id
        ).await
        .map_err(|err| err.process_err(err, ""))?
    } else {
        vec![]
    };


    let mut contrac_option = ContractOption {
        current: None,
        contracts
    };

    'outer: for num in comment_data.doc_num.iter() {
        for contract in contrac_option.contracts.iter() {
            if num == contract.contract_num {
                contrac_option.current = Some(contract.clone());
                break 'outer;
            }
        }
    }

	let debet: Account = if let Some(ctrpty) = &ctrpty_option {
		if ctrpty_types.banks.contains(&ctrpty.comp_id) {
			if comment_data.is_komis || comment_data.is_depos_persent {
				Account::OtherIncome
			} else if comment_data.is_person || comment_data.is_founder {
				Account::OtherPayables
			} else if comment_data.is_salary {
				Account::Payroll
			} else {
				Account::SpecBankAcc
			}
		} else if ctrpty_types.gov.contains(&ctrpty.comp_id) {
			if comment_data.is_tax {
				Account::Taxes
			} else {
				Account::OtherIncome
			}
		} else {
			if comment_data.is_depos_persent {
				Account::OtherIncome
			} else if comment_data.is_cred_return {
				Account::ShortLoans
			} else {
				Account::Vendors
			}	
		}
	} else {
		if comment_data.is_founder || comment_data.is_person {
			Account::OtherPayables
		} else if comment_data.is_salary {
			Account::Payroll
		} else {
			Account::Vendors
		}
	};

     
    let credit = Account::BankAcc;
    
    let amount = block_fields.statement_amount.clone();

    let oper_date = block_fields.pay_date.clone();

    let doc_type = block_fields.doc_type.clone();

    let doc_num = block_fields.doc_num.clone();

    let doc_date = block_fields.doc_date.clone();

    let is_storno = false;

    let is_del = false;

    let entr_date = Date::unchecked(chrono::Utc::now().naive_utc());

    let ctrpty_id_option = ctrpty_option.as_ref().map(|c| c.comp_id.clone());

	let rule = IdRules::Operation {
		comp_id: Some(comp_id.clone()),
		ctrpty_id: ctrpty_id_option,
		debet: Some(debet.clone()),
		credit: Some(credit.clone()),
		amount: Some(amount.clone()),
		oper_date: oper_date.clone(),
		doc_num: Some(doc_num.clone()), 
	};
	let oper_id = rule.make_stat_id();

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

        is_sync,

        comment: block_fields.doc_comment.clone(),

        is_duplicate: false
    };

    Ok(res)
}



pub async fn make_statement_rec_operation_raw(
    state: &ClientState,
    parsed_block: &ParsedBlock,
	ctrpty_types: &CtrPtyTypes
) -> Result<OperationRaw, Status> {

    let session = state
        .get_session()
        .await
        .map_err(|err| err.process_err(err, ""))?; 

    let ParsedBlock { block_fields, comment_data } = parsed_block;

    let user_id = session.session_user.user.user_id.clone();

    let comp_id = session.session_user.company.comp_id.clone();

	let ctrpty_option = try_get_by_inn_kpp_then_inn_function(
		state, 
		&block_fields.pay_inn, 
		&block_fields.pay_kpp
	).await
	.map_err(|err| err.process_err(err, ""))?;

    let contracts = if let Some(ctrpty) = ctrpty_option.as_ref() {
        get_contracts_by_comp_ctrpty_ids (
            state,
            &ctrpty.comp_id
        ).await
        .map_err(|err| err.process_err(err, ""))?
    } else {
        vec![]
    };

    let mut contrac_option = ContractOption {
        current: None,
        contracts
    };

    'outer: for num in comment_data.doc_num.iter() {
        for contract in contrac_option.contracts.iter() {
            if num == contract.contract_num {
                contrac_option.current = Some(contract.clone());
                break 'outer;
            }
        }
    }
    
    let debet = Account::BankAcc;

	let credit: Account = if let Some(ctrpty) = &ctrpty_option {
		if ctrpty_types.banks.contains(&ctrpty.comp_id) {
			if comment_data.is_cred_return {
				Account::ShortLoans
			} else {
				Account::SpecBankAcc
			}
		} else if ctrpty_types.gov.contains(&ctrpty.comp_id) {
			if comment_data.is_tax {
				Account::Taxes
			} else {
				Account::OtherIncome
			}
		} else {
			if comment_data.is_depos_persent {
				Account::OtherIncome
			} else if comment_data.is_cred_loan {
				Account::ShortLoans
			} else {
				Account::Customers
			}	
		}
	} else {
		if comment_data.is_founder {
			Account::OtherPayables
		} else {
			Account::Customers
		}
	};
    
    let amount = block_fields.statement_amount.clone();

    let oper_date = block_fields.rec_date.clone();

    let doc_type = block_fields.doc_type.clone();

    let doc_num = block_fields.doc_num.clone();

    let doc_date = block_fields.doc_date.clone();

    let is_storno = false;

    let is_del = false;

    let entr_date = Date::unchecked(chrono::Utc::now().naive_utc());

    let ctrpty_id_option = ctrpty_option.as_ref().map(|c| c.comp_id.clone());

	let rule = IdRules::Operation {
		comp_id: Some(comp_id.clone()),
		ctrpty_id: ctrpty_id_option,
		debet: Some(debet.clone()),
		credit: Some(credit.clone()),
		amount: Some(amount.clone()),
		oper_date: oper_date.clone(),
		doc_num: Some(doc_num.clone()), 
	};

	let oper_id = rule.make_stat_id();

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

    let user_id = session.session_user.user.user_id.clone();

    let comp_id = session.session_user.company.comp_id.clone();

    let ctrpty_option = Some(session.session_user.company.clone());

    let contract = ContractOption {
        current: None,
        contracts: vec![]
    };

    let (debet, credit) = if parsed_block.comment_data.is_cred_loan {
        (Account::SpecBankAcc, Account::BankAcc)
    } else {
        (Account::BankAcc, Account::SpecBankAcc)
    };

    let amount = parsed_block.block_fields.statement_amount.clone();

    let oper_date = parsed_block.block_fields.pay_date.clone();

    let doc_type = parsed_block.block_fields.doc_type.clone();

    let doc_num = parsed_block.block_fields.doc_num.clone();

    let doc_date = parsed_block.block_fields.doc_date.clone();

    let is_storno = false;

    let is_del = false;

    let entr_date = Date::unchecked(chrono::Utc::now().naive_utc());

    let ctrpty_id_option = ctrpty_option.as_ref().map(|c| c.comp_id.clone());

	let rule = IdRules::Operation {
		comp_id: Some(comp_id.clone()),
		ctrpty_id: ctrpty_id_option,
		debet: Some(debet.clone()),
		credit: Some(credit.clone()),
		amount: Some(amount.clone()),
		oper_date: oper_date.clone(),
		doc_num: Some(doc_num.clone()), 
	};

	let oper_id = rule.make_stat_id();

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

        is_sync,

        comment: parsed_block.block_fields.doc_comment.clone(),

        is_duplicate : false
    };

    Ok(res)
}