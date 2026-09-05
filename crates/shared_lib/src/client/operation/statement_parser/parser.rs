use std::collections::{HashMap, HashSet};
use encoding_rs::WINDOWS_1251;
use futures::stream::{self, StreamExt};

use crate::primitives::frozen::text::Kpp;
use crate::{ClientState, ProcessError, Status};
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
use crate::sql_models::company::implements::{InnKppMapAcc, CompCrateData};
use crate::client::operation::helper::make_statement_block_map;
use crate::client::operation::statement_parser::comment::parse_comment;
use crate::client::operation::statement_parser::make_operations::make_statement_operation_raw;
use crate::client::sql_queries::companys::add::new_companys::add_companys_by_inn_cpp_acc;
use crate::client::sql_queries::operations::get::exist_ids_by_operations::get_exist_ids_by_ids;


pub async fn parse_statement(
    state: &ClientState,
    ras_bic_acc: &RasBicAcc,
    path: &str
) -> Result<OperationStep, Status> {

    let failed_result = Ok(OperationStep::TryLater {
        text: OperationInfo::ClientApiSystemError,
    });

    let mut new_companys: InnKppMapAcc = HashMap::new();

    let mut parsed_blocks: Vec<ParsedBlock> = vec!();

    let bytes = match tokio::fs::read(path).await {
        Ok(b) => b,
        Err(err) => {
            err.process_err(Status::FileReadError, "");
            return failed_result;
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
            Status::Tech.process_err(Status::DataCorruptionErr, "");
            return Ok(OperationStep::Loading { text: OperationInfo::WrongFile });
        }
    };

    let mut full_head_iter = full_head.split("СекцияРасчСчет");

    match full_head_iter.next() {
        Some(_) => {},
        None =>  {
            Status::Tech.process_err(Status::DataCorruptionErr, "");
            return Ok(OperationStep::Loading { text: OperationInfo::WrongFile });
        }
    };

    let head_str  = match full_head_iter.next() {
        Some(a) => a.trim(),
        None =>  {
            Status::Tech.process_err(Status::DataCorruptionErr, "");
            return Ok(OperationStep::Loading { text: OperationInfo::WrongFile });
        }
    };

    let head_map = make_statement_block_map(head_str);

    let head = match StatementHead::from_map(&head_map) {
        Ok(h) => h,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    if head.head_acc != ras_bic_acc.ras_acc {
        return Ok(OperationStep::Loading {
            text: OperationInfo::WrongBankAcc,
        });
    }

    for block_str in data_iter {
        let block_map = make_statement_block_map(block_str);

        let mut block_fields = match BlockFields::from_map(&block_map) {
            Ok(f) => f,
            Err(err) => {
                err.process_err(err, block_str);
                return failed_result;
            }
        };

        if let Some(comp_inn) = block_fields.pay_inn.clone() {
            if comp_inn.len() == 12 {
                block_fields.pay_kpp = Kpp::unchecked("".to_string());
            }
        } else {
            block_fields.pay_kpp = Kpp::unchecked("".to_string());
        }
        
        if let Some(comp_inn) = block_fields.rec_inn.clone() {
            if comp_inn.len() == 12 {
                block_fields.rec_kpp = Kpp::unchecked("".to_string());
            }
        } else {
            block_fields.rec_kpp = Kpp::unchecked("".to_string());
        }

        let comment_data = parse_comment(&block_fields.doc_comment);



        if let Some(comp_inn) = block_fields.pay_inn.clone() {
            let pay_key = (comp_inn, block_fields.pay_kpp.clone());
            new_companys.insert(pay_key, HashSet::new());
    
        }

        if let Some(comp_inn) = block_fields.rec_inn.clone() {
            let rec_key = (comp_inn, block_fields.rec_kpp.clone());
            new_companys.insert(rec_key, HashSet::new());
        }
        
        parsed_blocks.push(ParsedBlock { block_fields, comment_data});

    }

    let mut data: Vec<CompCrateData> = vec!();

    for ((comp_inn, kpp), bank_acc) in new_companys {
        data.push(CompCrateData{comp_inn, kpp, bank_acc});
    }

    match add_companys_by_inn_cpp_acc(state, &data).await {
        Ok(_) => {},
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
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
                err.process_err(err, "");
                return failed_result;
            } 
        }
    }
    
    let exist_ids = match get_exist_ids_by_ids(state, &operations).await {
        Ok(ids) => ids,
        Err(err) => {
            err.process_err(err, "");
            return failed_result;
        }
    };

    for operation in operations.iter_mut() {
        if exist_ids.contains(&operation.oper_id) {
            operation.is_duplicate  = true;
   
        }
    }
    
    let success_result = OperationStep::StatementSuccess {
        text: OperationInfo::SuccessRaw,
        operations
    };

    Ok(success_result)
    
}