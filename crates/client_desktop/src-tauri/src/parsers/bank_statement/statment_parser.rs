use std::collections::{HashMap, HashSet};
use encoding_rs::WINDOWS_1251;

use shared_lib::Status;
use shared_lib::parsers::bank_statement::implements::{
    BlockFields,
    ParsedBlock, 
    StatementHead,
    StatementParseResult,
    InnKppMapAcc
};
use shared_lib::sql_models::operation::implements::Operation;
use shared_lib::primitives::composite::implements::RasBicAcc;

use crate::state::ClientState;
use crate::parsers::bank_statement::helper::make_statement_block_map;
use crate::parsers::bank_statement::comment_parser::parse_comment;
use crate::sql_queries::companys::add::new_companys::add_companys_by_inn_cpp_acc;


pub(crate) async fn parse_statement(
    state: &ClientState,
    ras_bic_acc: &RasBicAcc,
    path: &String
) -> Result<StatementParseResult, Status> {

    let failed_result = StatementParseResult {
        text: "Критическая ошибка, попробуйте позже".to_string(),
        operations: vec!()
    };

    let mut new_companys: InnKppMapAcc = HashMap::new();

    let mut parsed_blocks: Vec<ParsedBlock> = vec!();

    let bytes = match std::fs::read(path) {
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
            return Ok(failed_result);
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
            return Ok(failed_result);
        }
    };

    let head_str  = match full_head_iter.next() {
        Some(a) => a.trim(),
        None =>  {
            log::error!(
                "local_err = {:?}, FUN parse_statement FAILED BY full_head_iter.next()",
                Status::DataCorruptionErr
            );
            return Ok(failed_result);
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
        return Ok(StatementParseResult {
            text: "Загружена выписка с ошибочного расчетного счета".to_string(),
            operations: vec!()
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
                block_fields.pay_bic.clone(),
                block_fields.pay_acc.clone()) {
            Ok(a) => a,
            Err(err) => {
                log::error!(
                    "local_err = {:?}, FUN parse_statement FAILED BY RasBicAcc::new", err
                );
                return Ok(failed_result);
            }
        };

        let rec_rass_bic_acc = match RasBicAcc::new(
                block_fields.rec_bic.clone(),
                block_fields.rec_acc.clone()) {
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

        new_companys.entry(pay_key).or_insert(HashSet::new()).insert(pay_rass_bic_acc);

        new_companys.entry(rec_key).or_insert(HashSet::new()).insert(rec_rass_bic_acc);
        
        parsed_blocks.push(ParsedBlock { block_fields, comment_data});

    }


    match add_companys_by_inn_cpp_acc(state, &new_companys).await {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN parse_statement FAILED BY FUN add_companys_by_inn_cpp_acc", err
            );
            return Ok(failed_result);
        }
    }

    



    Err(Status::Unknown)
    
}