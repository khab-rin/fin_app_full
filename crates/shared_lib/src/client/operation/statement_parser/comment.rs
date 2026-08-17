use crate::primitives::frozen::text::{Date, RubF, TextInfo, DocNum};
use crate::sql_models::operation::parser::BlockCommentData;
use crate::static_data::primitives_re::*;



pub fn parse_comment(
    comment: &TextInfo
) -> BlockCommentData {

    let low = comment.to_lowercase();

    let mut parse_data = BlockCommentData::default();
    
    
    if low.contains("возвр") && (
        low.contains("кредит") || 
        low.contains("заем") || 
        low.contains("займ") ||
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
        low.contains("займ") ||
        low.contains("ссуд")
    ) {
        parse_data.is_cred_loan = true;
    }

    parse_data.is_period = low.contains("за период");

    if !parse_data.is_period && low.contains(" с ") && low.contains(" по ") {parse_data.is_period = true}

    parse_data.is_contract = low.contains("договор") || low.contains("контракт");

    parse_data.is_salary = low.contains("заработн") || low.contains("зарплата");

    parse_data.is_invoice = (low.contains("счет") || low.contains("cчёт")) && !low.contains("обсл.сч");

    parse_data.is_penalty = low.contains("штраф") || 
        low.contains("взыскан") || 
        low.contains("неустой") || 
        low.contains("пени");
    
    parse_data.is_komis = low.contains("комисс");

    for cap in get_scan_dates_reg().captures_iter(comment) {
        if let Some(d) = cap.get(0) {
            let raw_date = d.as_str();
            
            let normalized = if raw_date.contains('.') {
                let parts: Vec<&str> = raw_date.split('.').collect();
                if parts.len() == 3 && parts[2].len() == 2 {
                    format!("{}.{}.20{}", parts[0], parts[1], parts[2])
                } else {
                    raw_date.to_string()
                }
            } else {
                raw_date.to_string()
            };

            match Date::new(&normalized) {
                Ok(dd) => parse_data.dates.push(dd),
                Err(err) => { parse_data.errors.insert(err); }
            }
        }
    }
    
    for cap in get_scan_doc_nums_reg().captures_iter(&low) {
        if let Some(num) = cap.get(1).or_else(|| cap.get(2)) {
            let cleaned_num = num.as_str()
                .trim()
                .trim_end_matches('.')
                .to_string();

            if cleaned_num == "от" || cleaned_num == "за" || cleaned_num == "период" {
                continue;
            }

            let is_only_cyrillic = cleaned_num.chars().all(|c| {
                matches!(c, 'а'..='я' | 'ё')
            });

            if is_only_cyrillic {
                continue;
            }

            parse_data.doc_num.insert(DocNum::unchecked(&cleaned_num));
        }
    }

    if low.contains("без ндс") || low.contains("не облаг") || low.contains("без налог") {
        parse_data.nds_rate = 0;
        parse_data.nds_amount = None;
    } else if let Some(caps) = get_scan_nds_rate_reg().captures(&low) {
        match caps.get(1).map(|v| v.as_str()) {
            Some("22") => {parse_data.nds_rate = 22}
            Some("20") => {parse_data.nds_rate = 20}
            Some("18") => {parse_data.nds_rate = 18}
            _ => {}
        }
        if let Some(cap) = get_scan_nds_amount_reg()
            .captures(&low) {
                let v = cap.get(1).unwrap().as_str();
                match RubF::new(v) {
                    Ok(rubf) => { parse_data.nds_amount = Some(rubf); }
                    Err(err) => { parse_data.errors.insert(err); }
                }
            }
    }
    
    parse_data
}

