use std::sync::OnceLock;
use regex::Regex;

pub static SCAN_CRYPTO_SNILS: OnceLock<Regex> = OnceLock::new();
pub static SCAN_PERSON_INN: OnceLock<Regex> = OnceLock::new();
pub static SCAN_CRYPTO_SURNAME: OnceLock<Regex> = OnceLock::new();
pub static SCAN_CRYPTO_NAME: OnceLock<Regex> = OnceLock::new();
pub static SCAN_CRYPTO_MIDNAME: OnceLock<Regex> = OnceLock::new();


pub fn get_scan_crypto_snils_reg() -> &'static Regex {
    SCAN_CRYPTO_SNILS.get_or_init(|| {
        Regex::new(r"(?i)(?:OID\.1\.2\.643\.100\.3|SNILS)\s*=\s*([0-9]{11})")
            .expect("WRONG_SCAN_CRYPTO_SNILS_REGEX!!!")
    })
}

pub fn get_scan_crypto_person_inn_reg() -> &'static Regex {
    SCAN_PERSON_INN.get_or_init(|| {
        Regex::new(r"(?i)(?:OID\.1\.2\.643\.3\.131\.1\.1|INN)\s*=\s*([0-9]{12})")
            .expect("WRONG_SCAN_PERSON_INN_REGEX!!!")
    })
}

pub fn get_scan_crypto_surname_reg() -> &'static Regex {
    SCAN_CRYPTO_SURNAME.get_or_init(|| {
        Regex::new(r"(?i)(?:SN|SURNAME)\s*=\s*([^,]+)")
            .expect("WRONG_SCAN_CRYPTO_SURNAME_REGEX!!!")
    })
}

pub fn get_scan_crypto_name_reg() -> &'static Regex {
    SCAN_CRYPTO_NAME.get_or_init(|| {
        Regex::new(r"(?i)(?:G|GIVENNAME)\s*=\s*([^,]+)")
            .expect("WRONG_SCAN_CRYPTO_NAME_REGEX!!!")
    })
}

pub fn get_scan_crypto_mid_name_reg() -> &'static Regex {
    SCAN_CRYPTO_MIDNAME.get_or_init(|| {
        Regex::new(r"(?i)CN\s*=\s*([^,]+)")
            .expect("WRONG_SCAN_CRYPTO_LASTNAME_REGEX!!!")
    })
}

pub fn init_crypto_re() {
    get_scan_crypto_snils_reg();
    get_scan_crypto_person_inn_reg();
    get_scan_crypto_surname_reg();
    get_scan_crypto_name_reg();
    get_scan_crypto_mid_name_reg();
}