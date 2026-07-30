use shared_lib::primitives::frozen::text::{Date, DocNum, RubF, TextInfo};
use shared_lib::sql_models::operation::parser::BlockCommentData;
use shared_lib::static_data::primitives_re::*;



pub(crate) fn parse_comment(
    comment: &TextInfo
) -> BlockCommentData {

    let low = comment.to_lowercase();

    let mut parse_data = BlockCommentData::default();
    
    if low.contains("возвр") && (
        low.contains("кредит") || 
        low.contains("заем") || 
        low.contains("ссуд")
    ) {
        parse_data.is_cred_return = true;
    }

    if (
        low.contains("выдач") || 
        low.contains("предоставлен") || 
        low.contains("перечислени")
    ) && (
        low.contains("кредит") || 
        low.contains("заем") || 
        low.contains("ссуд")
    ) {
        parse_data.is_cred_loan = true;
    }

    parse_data.is_period = low.contains("за период");

    if !parse_data.is_period && low.contains(" с ") && low.contains(" по ") {parse_data.is_period = true}

    parse_data.is_contract = low.contains("договор") || low.contains("контракт");

    parse_data.is_salary = low.contains("заработн") || low.contains("зарплата");

    parse_data.is_invoice = low.contains("счет") || low.contains("cчёт");

    parse_data.is_penalty = low.contains("штраф") || 
        low.contains("взыскан") || 
        low.contains("неустой") || 
        low.contains("пени");
    
    parse_data.is_komis = low.contains("комисс");

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
                parse_data.doc_num.insert(num.as_str().to_string());
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

