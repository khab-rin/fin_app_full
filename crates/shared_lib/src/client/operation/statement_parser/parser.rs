use std::collections::HashMap;
use encoding_rs::WINDOWS_1251;
use futures::stream::{self, StreamExt};

use crate::{Status, ClientState};
use crate::sql_models::operation::parser::{
    BlockFields,
    ParsedBlock, 
    StatementHead
};
use crate::sql_models::operation::service::{
    OperationInfo,
    OperationStep
};
use crate::sql_models::operation::implements::OperationRaw;
use crate::primitives::composite::implements::RasBicAcc;
use crate::service::mchd::home_mchd_power::HomeMchdPower;
use crate::sql_models::company::implements::{InnKppMapAcc, CompCrateData};

use crate::client::operation::helper::make_statement_block_map;
use crate::client::operation::statement_parser::comment::parse_comment;
use crate::client::operation::statement_parser::make_operations::make_statement_operation_raw;
use crate::client::sql_queries::companys::add::new_companys::add_companys_by_inn_cpp_acc;
use crate::client::mchd::show_powers::check_access;
use crate::client::sql_queries::operations::get::ext_ids_by_operations::get_ext_ids_by_ext_id;


pub async fn parse_statement(
    state: &ClientState,
    ras_bic_acc: &RasBicAcc,
    path: &str
) -> Result<OperationStep, Status> {

    let failed_result = OperationStep::TryLater {
        text: OperationInfo::ClientApiSystemError,
    };

    match check_access(state, &HomeMchdPower::H210).await {
        Ok(true) => {},
        Ok(false) => {
            return Ok(OperationStep::AccessDenied { text: OperationInfo::AccessDenied })
        },
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN parse_comment FAILED BY FUN check_access", err
            );
            return Ok(failed_result);
        }
    }

    let mut new_companys: InnKppMapAcc = HashMap::new();

    let mut parsed_blocks: Vec<ParsedBlock> = vec!();

    let bytes = match tokio::fs::read(path).await {
        Ok(b) => b,
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN parse_statement FAILED BY std::fs::read(path)",
                err, Status::FileReadError
            );
            return Ok(failed_result);
        }
    };

    let buffer = match String::from_utf8(bytes) {
        Ok(good_utf8) => good_utf8,
        Err(err) => {
            let win1251_bytes = err.into_bytes();
            let (cow, _, _) = WINDOWS_1251.decode(&win1251_bytes);
            cow.into_owned()
        }
    };

    let mut data_iter = buffer.split("СекцияД");

    let full_head = match data_iter.next() {
        Some(a) => a.to_string(),
        None =>  {
            log::error!(
                "local_err = {:?}, FUN parse_statement FAILED BY data_iter.next()",
                Status::DataCorruptionErr
            );
            return Ok(OperationStep::Loading { text: OperationInfo::WrongFile });
        }
    };

    let mut full_head_iter = full_head.split("СекцияРасчСчет");

    match full_head_iter.next() {
        Some(_) => {},
        None =>  {
            log::error!(
                "local_err = {:?}, FUN parse_statement FAILED BY full_head_iter.next()",
                Status::DataCorruptionErr
            );
            return Ok(OperationStep::Loading { text: OperationInfo::WrongFile });
        }
    };

    let head_str  = match full_head_iter.next() {
        Some(a) => a.trim(),
        None =>  {
            log::error!(
                "local_err = {:?}, FUN parse_statement FAILED BY full_head_iter.next()",
                Status::DataCorruptionErr
            );
            return Ok(OperationStep::Loading { text: OperationInfo::WrongFile });
        }
    };

    let head_map = make_statement_block_map(head_str);

    let head = match StatementHead::from_map(&head_map) {
        Ok(h) => h,
        Err(err) => {
            log::error!(
                "local_err = {}, FUN bank_parser FAILED BY MAPPING StatementHead",
                err
            );
            return Ok(failed_result);
        }
    };

    if head.head_acc != ras_bic_acc.ras_acc {
        return Ok(OperationStep::Loading {
            text: OperationInfo::WrongBankAcc,
        });
    }

    for block_str in data_iter {
        let block_map = make_statement_block_map(block_str);

        let block_fields = match BlockFields::from_map(&block_map) {
            Ok(f) => f,
            Err(err) => {
                log::error!(
                    "local_err = {:?}, FUN parse_statement FAILED BY MAPPING BLOCK", err
                );
                return Ok(failed_result);
            }
        };

        let comment_data = parse_comment(&block_fields.doc_comment);

        let pay_rass_bic_acc = match RasBicAcc::new(
                &block_fields.pay_bic,
                &block_fields.pay_acc) {
            Ok(a) => a,
            Err(err) => {
                log::error!(
                    "local_err = {:?}, FUN parse_statement FAILED BY RasBicAcc::new", err
                );
                return Ok(failed_result);
            }
        };

        let rec_rass_bic_acc = match RasBicAcc::new(
                &block_fields.rec_bic,
                &block_fields.rec_acc) {
            Ok(a) => a,
            Err(err) => {
                log::error!(
                    "local_err = {:?}, FUN parse_statement FAILED BY RasBicAcc::new", err
                );
                return Ok(failed_result);
            }
        };

        let pay_key = (block_fields.pay_inn.clone(), block_fields.pay_kpp.clone());
        let rec_key = (block_fields.rec_inn.clone(), block_fields.rec_kpp.clone());

        new_companys.entry(pay_key).or_default().insert(pay_rass_bic_acc);

        new_companys.entry(rec_key).or_default().insert(rec_rass_bic_acc);
        
        parsed_blocks.push(ParsedBlock { block_fields, comment_data});

    }

    let mut data: Vec<CompCrateData> = vec!();

    for ((comp_inn, kpp), bank_acc) in new_companys {
        data.push(CompCrateData{comp_inn, kpp, bank_acc});
    }


    match add_companys_by_inn_cpp_acc(state, &data).await {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN parse_statement FAILED BY FUN add_companys_by_inn_cpp_acc", err
            );
            return Ok(failed_result);
        }
    }

    let mut operations: Vec<OperationRaw> = vec!();   

    let mut tasks_vec = vec!();


    for block in &parsed_blocks {
        tasks_vec.push(make_statement_operation_raw(state, block));
    }

    let mut task_stream = stream::iter(tasks_vec).buffer_unordered(2);

    while let Some(res) = task_stream.next().await {
        match res {
            Ok(o) => operations.push(o),
            Err(err) => {
                log::error!(
                    "local_err = {:?}, FUN parse_statement FAILED BY FUB make_statement_operation_raw", err
                );
                return Ok(failed_result);
            } 
        }
    }

    let exist_ext_ids = match get_ext_ids_by_ext_id(state, &operations).await {
        Ok(ids) => ids,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN parse_statement FAILED BY FUN get_ext_ids_by_ext_id",
                err
            );
            return Ok(failed_result);
        }
    };

    for operation in operations.iter_mut() {
        if exist_ext_ids.contains(&operation.external_id) {
            operation.is_duplicate  = true;
        }
    }

    let success_result = OperationStep::SuccessRaw  {
        text: OperationInfo::SuccessRaw,
        operations
    };

    Ok(success_result)
    
}