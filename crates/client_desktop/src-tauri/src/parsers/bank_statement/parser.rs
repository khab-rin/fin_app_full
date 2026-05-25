use std::collections::HashMap;
use std::path::Path;
use std::fs;
use shared_lib::parsers::bank_statement::implements::{*};
use shared_lib::err_models::implements::Status;
use shared_lib::primitives::frozen::implements::{Date, DocNum, RubF, RasAcc};
use shared_lib::static_data::primitives_re::*;
use encoding_rs::WINDOWS_1251;

use crate::config::Config;


pub(crate) fn parse_comment(operation: &OperationReadFields) -> OperationParseData {
    let own_inn = &Config::global().own_company.own_inn;
    
    let comment = &*operation.doc_comment;
    let low = comment.to_lowercase();
    let mut parse_data = OperationParseData::default();
    
    if operation.pay_inn == own_inn && operation.rec_inn == own_inn { parse_data.is_own_operation = true;}
    if low.contains("личные средства") { parse_data.is_own_operation = true; }

    if operation.doc_maker_status.is_some() { parse_data.is_tax = true }

    parse_data.is_period = low.contains("за период");
    if !parse_data.is_period && low.contains(" с ") && low.contains(" по ") {parse_data.is_period = true}

    parse_data.is_contract = low.contains("договор");


    parse_data.is_salary = low.contains("заработн") || low.contains("зарплата");

    parse_data.is_invoice = low.contains("счет") || low.contains("cчёт");

    parse_data.is_penalty = low.contains("штраф") || low.contains("взыскан") || 
        low.contains("неустой") || low.contains("пени");
    
    parse_data.is_komis = low.contains("комисси");

    parse_data.is_credit = low.contains("кредит") || low.contains("депоз");
    if !parse_data.is_credit && !parse_data.is_penalty && low.contains("процент") {parse_data.is_credit = true}


    for cap in get_scan_dates_reg()
        .captures_iter(comment) {
            if let Some(d) = cap.get(0) {
                match Date::new(d.as_str()) {
                    Ok(dd) => parse_data.dates.push(dd),
                    Err(err) => {parse_data.errors.insert(err);}
                }
                
            }
        }
    
    for cap in get_scan_doc_nums_reg()
        .captures_iter(comment) {
            if let Some(num) = cap.get(1) {
                match DocNum::new(num.as_str()) {
                    Ok(dd) => parse_data.doc_nums.push(dd),
                    Err(err) => {parse_data.errors.insert(err); }
                }
            }
        }
    
    if low.contains("без ндс") || low.contains("не облаг") || low.contains("без налог") {
        parse_data.nds_rate = 0;
    } else if let Some(caps) = get_scan_nds_rate_reg().captures(&low) {
        match caps.get(1).map(|v| v.as_str()) {
            Some("22") => {parse_data.nds_rate = 22}
            Some("20") => {parse_data.nds_rate = 20}
            Some("18") => {parse_data.nds_rate = 18}
            _ => {}
        }
    }


  
    if let Some(cap) = get_scan_nds_amount_reg()
        .captures(&low) {
            let v = cap.get(1).unwrap().as_str();
            match RubF::new(v) {
                Ok(rubf) => { parse_data.nds_amount = Some(rubf); }
                Err(err) => { parse_data.errors.insert(err); }
            }
        }
    
    parse_data
}



pub(crate) fn bank_parser<P: AsRef<Path>>(path: P) -> Result<ParseBankStatRes, Status> {
    let own_ras_acc = RasAcc::new("40802810629370001827").expect("123");

    let mut parse_result = ParseBankStatRes::default();

    let bytes = match fs::read(path).map_err(|_| Status::FileReadError) {
        Ok(b) => b,
        Err(err) => {
            log::error!(
                "tech_err = {}, local_err = {}
                FUN bank_parser FAILED BY READING FILE",
                err, Status::FileReadError
            );
            return Err(Status::FileReadError);
        }
    };

    let (buffer, wr_bytes) = match String::from_utf8(bytes) {
        Ok(good_utf8) => { (good_utf8, false) }
        Err(err) => {
            let win1251_bytes = err.into_bytes();
            let (cow, _, wr_bytes) = WINDOWS_1251.decode(&win1251_bytes);
            (cow.into_owned(), wr_bytes)
        }
    };


    if wr_bytes { parse_result.status.insert(Status::FileReadError); }

    let mut data_iter = buffer.split("СекцияДокумент=");

    let mut head_block_iter = data_iter
        .next()
        .ok_or(Status::MappingError)?
        .trim()
        .split("СекцияРасчСчет");

    head_block_iter.next().ok_or(Status::MappingError)?;
    let block_head = head_block_iter.next().ok_or(Status::MappingError)?.trim();
    let mut block_map:HashMap<&str, &str> = HashMap::new();

    for s in block_head.lines() {
        if let Some((key, value)) = s.split_once('=') {
            block_map.insert(key.trim(), value.trim());
        }
    }

    let head = StatementHead::from_map(&block_map)
        .map_err(|_| Status::MappingError)?;

    if head.head_acc != own_ras_acc { return Err(Status::FileInvalideData); }

    parse_result.st_head = Some(head);


    for block in data_iter {
        let block = block.trim();
        let mut block_map: HashMap<&str, &str> = HashMap::new();
        for line in block.lines() {
            if let Some((key, value)) = line.split_once('=') {
                block_map.insert(key.trim(), value.trim());
            }
        }

        match OperationReadFields::from_map(&block_map) {
            Ok(read_fields) => {
                let parse_data = parse_comment(&read_fields);
                parse_result.correct_lines.push(
                    ParsedOperation {
                        read_fields, parse_data
                    }
                );
            }
            Err(err) =>  {
                parse_result.status.insert(err);
                let wrong_line:HashMap<String, String> = block_map
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect();
                parse_result.wrong_lines.push(wrong_line);
            }
        }
    }

     Ok(parse_result)
    
}
