use std::sync::OnceLock;
use regex::Regex;
use rust_decimal::Decimal;

pub(crate) static SCAN_DATES_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static SCAN_DOC_NUMS_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static SCAN_NDS_RATE_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static SCAN_NDS_AMOUNT_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static IS_INN_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static IS_KPP_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static IS_COR_ACC_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static IS_BIC_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static IS_OGRN_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static IS_OKVED_REG:OnceLock<Regex> = OnceLock::new();
pub(crate) static NDS_22_DEFAULT:OnceLock<Decimal> = OnceLock::new();




pub fn get_scan_dates_reg() -> & 'static Regex {
    SCAN_DATES_REG.get_or_init(|| {
        Regex::new(r"(\d{2}[./-]?\d{2}[./-]?\d{2,4})")
            .expect("SCAN_DATES_REGEX_ERROR!!!")
    })
}

pub fn get_scan_doc_nums_reg() -> & 'static Regex {
    SCAN_DOC_NUMS_REG.get_or_init(|| {
        Regex::new(r"(?i)(?:договор[а-я]{0,3}|дог\.?|счет|сч\.?|инвойс|№)\s*(?:№|от|н[./]р)?\s*([a-zа-я0-9\-/№]{2,18})")
            .expect("WRONG_SCAN_DOC_NUMS_REGEX!!!")
    })
}

pub fn get_scan_nds_rate_reg() -> & 'static Regex {
    SCAN_NDS_RATE_REG.get_or_init(|| {
        Regex::new(r"(?i)ндс\s*(?:\d{1,2}\s*%\s*[,.]?\s*)([\d\s]+[.,]\d{2})")
            .expect("WRONG_SCAN_NDS_RATE_REGEX!!!")  
    })
}

pub fn get_scan_nds_amount_reg() -> & 'static Regex {
    SCAN_NDS_AMOUNT_REG.get_or_init(|| {
        Regex::new(r"(?i)ндс(?:.*?\d{1,2}\s*%)?.*?([\d\s]+[.,]\d{2})")
            .expect("WRONG_SCAN_NDS_AMOUNT_REGEX!!!")  
    })
}


pub fn get_is_inn_reg() -> & 'static Regex {
    IS_INN_REG.get_or_init(|| {
        Regex::new(r"^(\d{10}|\d{12})$")
            .expect("WRONG_INN_REGEX!!!")
    })
}

pub fn get_is_kpp_reg() -> & 'static Regex {
    IS_KPP_REG.get_or_init(|| {
        Regex::new(r"^\d{4}[0-9A-Z]{2}\d{3}$")
            .expect("WRONG_KPP_REGEX!!!")
    })
}

pub fn get_is_corr_ras_acc_reg() -> & 'static Regex {
    IS_COR_ACC_REG.get_or_init(|| {
        Regex::new(r"^\d{20}$")
            .expect("WRONG_BANK_ACC_REGEX!!")
    })
}

pub fn get_is_bic_reg() -> & 'static Regex {
    IS_BIC_REG.get_or_init(|| {
        Regex::new(r"^0[14]\d{7}$")
            .expect("WRONG_BIC_REGEX!!!")  
    })
}

pub fn get_is_ogrn_reg() -> & 'static Regex {
    IS_OGRN_REG.get_or_init(|| {
        Regex::new(r"^(\d{13}|\d{15})$")
         .expect("WRONG_OGRN_REGEX!!!")
    })
}

pub fn get_is_okveg_reg() -> &'static Regex {
    IS_OKVED_REG.get_or_init(
        || Regex::new(r"^\d{2}(\.\d{1,2}(\.\d{1,2})?)?$")
        .expect("WRONG_IS_OKVED_REG")
    )
}

pub fn get_nds_22_default() -> &'static Decimal {
    NDS_22_DEFAULT.get_or_init(|| {
        Decimal::from(22) / Decimal::from(122)
    })
}


pub fn init_primitivrs_re() {
    get_scan_dates_reg();
    get_scan_doc_nums_reg();
    get_scan_nds_rate_reg();
    get_scan_nds_amount_reg();
    get_is_inn_reg();
    get_is_kpp_reg();
    get_is_corr_ras_acc_reg();
    get_is_bic_reg();
    get_is_ogrn_reg();
    get_is_okveg_reg();
    get_nds_22_default();
}