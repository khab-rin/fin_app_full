use std::{str::FromStr};
use chrono::{NaiveDate, Datelike};
use rust_decimal::Decimal;

use crate::static_data::primitives_re::*;
use crate::static_data::const_var::DATE_FORMATS;
use crate::Status;

fn validate_mod11_rosstat(s: &str) -> bool {
    let digits: Vec<u32> = s.chars().filter_map(|c| c.to_digit(10)).collect();
    if digits.len() < 2 { return false; }

    let n = digits.len();
    let check_digit = digits[n - 1];
    let payload = &digits[0..n - 1];

    let mut sum = 0;
    for (i, &digit) in payload.iter().enumerate() {
        sum += digit * ((i as u32 % 10) + 1);
    }
    let mut remainder = sum % 11;

    if remainder == 10 {
        sum = 0;
        for (i, &digit) in payload.iter().enumerate() {
            sum += digit * ((i as u32 + 2) % 10 + 1);
        }
        remainder = sum % 11;
        if remainder == 10 { remainder = 0; }
    }
    remainder == check_digit
}

pub fn init_pers_inn_from_str(inn: &str) -> Result<Box<str>, Status> {
    let inn = inn.trim();
    
    if inn.len() != 12 || !inn.chars().all(|c| c.is_ascii_digit()) {
        return Err(Status::PersInnValid);
    }
    
    let digits: Vec<u32> = inn.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let d11_weights = [7, 2, 4, 10, 3, 5, 9, 4, 6, 8, 0];
    let d12_weights = [3, 7, 2, 4, 10, 3, 5, 9, 4, 6, 8, 0];

    let mut sum11 = 0;
    for i in 0..11 {
        sum11 += digits[i] * d11_weights[i];
    }
    let check_digit11 = (sum11 % 11) % 10;

    let mut sum12 = 0;
    for i in 0..12 {
        sum12 += digits[i] * d12_weights[i];
    }
    let check_digit12 = (sum12 % 11) % 10;

    if digits[10] == check_digit11 && digits[11] == check_digit12 {
        Ok(inn.to_string().into_boxed_str())
    } else {
        Err(Status::PersInnValid)
    }
}

pub fn init_comp_inn_from_str(inn: &str) -> Result<Box<str>, Status> {
    let inn = inn.trim();

    if inn.len() != 10 || !inn.chars().all(|c| c.is_ascii_digit()) {
        return Err(Status::CompInnValid);
    }

    let digits: Vec<u32> = inn.chars().map(|c| c.to_digit(10).unwrap()).collect();

    let d10_weights = [2, 4, 10, 3, 5, 9, 4, 6, 8, 0];

    let mut sum10 = 0;
    for i in 0..9 {
        sum10 += digits[i] * d10_weights[i];
    }
    let check_digit10 = (sum10 % 11) % 10;

    if digits[9] == check_digit10 {
        Ok(inn.to_string().into_boxed_str())
    } else {
        Err(Status::CompInnValid)
    }
}

pub(crate) fn init_kpp_from_str(kpp: &str) -> Result<Box<str>, Status> {
    let kpp = kpp.trim().to_uppercase();
    if kpp.is_empty() || kpp == "0" || kpp == "000000000" {
        return Ok("0".into());
    }
    get_is_kpp_reg().is_match(&kpp)
        .then(|| kpp.into_boxed_str())
        .ok_or(Status::ValidKpp)
}

pub(crate) fn init_cor_ras_acc_from_str(bank_acc: &str) -> Result<Box<str>, Status> {
    let bank_acc = bank_acc.trim();
    get_is_corr_ras_acc_reg().is_match(bank_acc)
        .then(|| bank_acc.into())
        .ok_or(Status::ValidBankAcc)
}

pub(crate) fn init_bic_from_str(bic: &str) -> Result<Box<str>, Status> {
    let bic = bic.trim();
    get_is_bic_reg().is_match(bic)
    .then(|| bic.into())
    .ok_or(Status::ValidBic)
}

pub(crate) fn init_ogrn_from_str(ogrn: &str) -> Result<Box<str>, Status> {
    let ogrn = ogrn.trim();
    if !get_is_ogrn_reg().is_match(ogrn) {
        return Err(Status::ValidOgrn);
    }
    let n = ogrn.len();
    let head: u64 = ogrn[..n-1].parse().map_err(|_| Status::ValidOgrn)?;
    let last: u8 = ogrn.as_bytes()[n-1] - b'0';
    let divider = if n == 13 { 11 } else { 13 };
    let expected = ((head % divider) % 10) as u8;

    if expected == last { Ok(ogrn.into()) } 
    else { Err(Status::ValidOgrn) }
}

pub(crate) fn str_to_decimal(value: &str) -> Result<Decimal, Status> {
    let clean = value.trim()
        .replace([' ', '\u{00A0}'], "")
        .replace(',', ".");
    match Decimal::from_str(&clean) {
        Ok(res) => Ok(res),
        Err(_) => Err(Status::ValidDecimalParser)
    } 
}

pub(crate) fn init_rubf_from_str(amount: &str) -> Result<Decimal, Status> {
    let v = str_to_decimal(amount.trim())?;
    if v < Decimal::ZERO { return Err(Status::ValidRubFParser) }
    if v.normalize().scale() > 2 { return Err(Status::ValidRubFParser ) }
    Ok(v)
}

pub(crate) fn init_date_time_from_str(val: &str) -> Result<chrono::DateTime<chrono::Utc>, Status> {
    let s = val.trim();
    
    tracing::info!("=== ВАЛИДАТОР ДАТЫ ВЫЗВАН С ТЕКСТОМ: '{}' ===", s);

    if s.is_empty() {
        return Ok(chrono::Utc::now());
    }

    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&chrono::Utc));
    }
    let clean_string = if s.contains('.') && s.ends_with(" UTC") {
        if let Some((left, _right)) = s.split_once('.') {
            format!("{} UTC", left)
        } else {
            s.to_string()
        }
    } else {
        s.to_string()
    };

    let formats = [
        "%Y-%m-%d %H:%M:%S UTC",
        "%Y-%m-%d %H:%M:%S",
        "%Y-%m-%dT%H:%M:%S",
    ];

    for fmt in &formats {
        if let Ok(naive) = chrono::NaiveDateTime::parse_from_str(&clean_string, fmt) {
            return Ok(chrono::DateTime::from_naive_utc_and_offset(naive, chrono::Utc));
        }
    }

    tracing::error!("=== КРИТИЧЕСКАЯ ОШИБКА: Ни один формат не подошел для даты '{}' (очищенная: '{}') ===", s, clean_string);
    
    Err(Status::ValidDateTime)
}

pub(crate) fn str_to_date(value: &str) -> Result<NaiveDate, Status> {
    let clean = value.trim()
        .replace(['/', '-', ' ', ',','_', '\u{00A0}'], ".")
        .replace("..", ".");
    for &fmt in DATE_FORMATS  {
        if let Ok(date) = NaiveDate::parse_from_str(&clean, fmt) {
            if (1900..2100).contains(&date.year()) {
                return Ok(date);           
            } else {
                return Err(Status::ValidDate);
            }
        }
    }
    Err(Status::ValidDate)
}

pub(crate) fn init_doc_num_from_str(val: &str) -> Result<Box<str>, Status> {
    Ok(val.into())
}

pub(crate) fn init_text_info_from_str(val: &str) -> Result<Box<str>, Status> {
    Ok(val.into())
}

pub(crate) fn init_branch_type_from_str(val: &str) -> Result<Box<str>, Status> {
    let val = val.trim().to_uppercase();
    match val.as_str() {
        "MAIN" | "BRANCH" => Ok(val.into_boxed_str()),
        _ => Err(Status::ValidBranchType)
    }
}

pub(crate) fn init_okpo_from_str(val: &str) -> Result<Box<str>, Status> {
    let s = val.trim();
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) || (s.len() != 8 && s.len() != 10) {
        return Err(Status::ValidOkpo);
    }
    match validate_mod11_rosstat(s) {
        true => { Ok(s.into())}
        false => { Err(Status::ValidOkpo)}
    }
}

pub(crate) fn init_oktmo_from_str(oktmo: &str) -> Result<Box<str>, Status> {
    let s = oktmo.trim();
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) || (s.len() != 8 && s.len() != 11) {
        return Err(Status::ValidOktmo);
    }
    Ok(s.into())
}

pub(crate) fn init_okogu_from_str(okogu: &str) -> Result<Box<str>, Status> {
    let s = okogu.trim();
    if s.is_empty() || !s.chars().all(|c| c.is_ascii_digit()) || s.len() != 7 {
        return Err(Status::ValidOkogu);
    }
    Ok(s.into())
}

pub(crate) fn init_opf_code_from_str(val: &str) -> Result<Box<str>, Status> {
    let s = val.trim();
    if s.len() == 5 
        && s.chars().all(|c| c.is_ascii_digit()) 
        && s.starts_with(|c: char| ('1'..='7').contains(&c)) 
    {
        Ok(s.into())
    } else {
        Err(Status::ValidOpf)
    }
}

pub(crate) fn init_okfs_from_str(val: &str) -> Result<Box<str>, Status> {
    let s = val.trim();
    if s.len() == 2 && s.chars().all(|c| c.is_ascii_digit()) {
        Ok(s.into())
    } else {
        Err(Status::ValidOkfs)
    }
}

pub(crate) fn init_okved_from_str(val: &str) -> Result<Box<str>, Status> {
    let val = val.trim();
    get_is_okveg_reg().is_match(val)
        .then(|| val.into())
        .ok_or(Status::ValidOkved)
}

pub(crate) fn init_phone_from_str(val: &str) -> Result<Box<str>, Status> {
    let n = val.chars().count();
    if !(1..=50).contains(&n) {
        return Err(Status::ValidPhone);
    }
    let mut digits: String = val
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();

    if digits.is_empty() {
        digits.push('8');
    }
    Ok(digits.into_boxed_str())
}

pub(crate) fn init_uuid_from_str(val: &str) -> Result<uuid::Uuid, Status> {
    let trimmed = val.trim();
    if trimmed.is_empty() {
        Ok(uuid::Uuid::new_v4())
    } else {
        uuid::Uuid::parse_str(trimmed)
            .map_err(|_| Status::ValidUuid)
    }
}

pub(crate) fn init_fio(val: &str) -> Result<Box<str>, Status> {
    let mut s: Vec<char> = val.trim().to_lowercase().chars().collect();
    if s.is_empty() || s.len() > 200 { return Err(Status::ValidFio) }
    if !s.iter().all(|&c| c.is_alphabetic() || c == '-') {
        return Err(Status::ValidFio);
    }
    s[0] = s[0].to_uppercase().next().ok_or(Status::ValidFio)?;
    for i in 1..s.len() {
        if s[i - 1] == '-' {
            s[i] = s[i].to_uppercase().next().ok_or(Status::ValidFio)?;
        }
    }
    Ok(s.iter().collect())  
}

pub(crate) fn init_region(val: &str) -> Result<Box<str>, Status> {
    let s = val.trim();
    if s.len() == 2 && s.chars().all(|c| c.is_ascii_digit()) {
        Ok(s.into())
    } else {
        Err(Status::ValidRegionNumber) 
    }
}

pub(crate) fn init_snils_from_str(val: &str) -> Result<Box<str>, Status> {
    let s = val.trim();
    if !s.chars().all(|c| {
        c.is_ascii_digit() || 
        matches!(c, '-' | '.' | ',' | '\\' | '|' | '/' | '\'' | '`' | ' ' | '\u{A0}')
    }) {
        return Err(Status::ValidSnils);
    }
    let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() != 11 {
        return Err(Status::ValidSnils);
    }
    let snils_num: u64 = digits.parse().unwrap_or(0);
    if snils_num > 1001998 {
        let d: Vec<u32> = digits
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        let mut sum = 0;
        for (digit, weight) in d.iter().take(9).zip((1..=9).rev()) {
            sum += digit * weight;
        }
        let calculated_checksum = match sum % 101 {
            100 | 101 => 0,
            s => s,
        };
        let actual_checksum = d[9] * 10 + d[10];
        if calculated_checksum != actual_checksum {
            return Err(Status::ValidSnils);
        }
    }
    Ok(digits.into_boxed_str())
}


pub(crate) fn init_part_status(val: &str) -> Result<Box<str>, Status> {
    let s = val.trim();
    match s {
        "101" | "102" | "299" | "301" | "303" | "399" => Ok(Box::from(s)),
        _ => Err(Status::ValidMchdPartStatus)
    }
}

pub(crate) fn init_flag_str(val: &str) -> Result<Box<str>, Status> {
    let s = val.trim();
    let chars:Vec<char> = s.chars().collect();
    if chars.len() != 8 { return Err(Status::Unknown);}
    for ch in chars {
        if ch != '1' && ch != '0' { return Err(Status::Unknown)}
    }
    Ok(s.into())
}

pub(crate) fn init_email_from_str(val: &str) -> Result<Box<str>, Status> {
    let n = val.chars().count();
    if !(3..=129).contains(&n) {
        return Err(Status::ValidEmail);
    }

    let parts: Vec<&str> = val.split('@').collect();
    
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(Status::ValidEmail);
    }

    let domain_parts: Vec<&str> = parts[1].split('.').collect();
    
    if domain_parts.len() < 2 || domain_parts.iter().any(|p| p.is_empty()) {
        return Err(Status::ValidEmail);
    }

    Ok(val.to_lowercase().into_boxed_str())
}

pub(crate) fn init_password_from_str(password: &str) -> Result<Box<str>, Status> {
    let password = password.trim();
    let len = password.chars().count();

    // Проверяем диапазон от 8 до 20 символов включительно
    (8..=20).contains(&len)
        .then(|| password.into())
        .ok_or(Status::ValidPassword)
}






