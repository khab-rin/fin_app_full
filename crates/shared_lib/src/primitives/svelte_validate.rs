use serde::{Serialize, Deserialize};
use crate::Status;
// Импортируем все ваши примитивы из соответствующего модуля
use crate::primitives::frozen::implements::*;
use crate::primitives::frozen::implements_base::*;

#[derive(Serialize, Deserialize, ts_rs::TS, Debug)]
pub enum SvelteValidator {
    Inn(String),
    Kpp(String),
    CorAcc(String),
    RasAcc(String),
    Bic(String),
    Ogrn(String),
    Date(String),
    RubF(String),
    DocNum(String),
    TextInfo(String),
    BranchType(String),
    Okpo(String),
    Oktmo(String),
    Okogu(String),
    Okfs(String),
    Okved(String),
    Phone(String),
    OpfCode(String),
    SurName(String),
    FirstName(String),
    MidName(String),
    Region(String),
    Snils(String),
    BoxUuid(String),
    DateTime(String),
    Email(String),
    ParticipantStatus(String),
    PoaReqElemsFlag(String),
    CompanyName(String),
    IdentStatus(String),
    PayType(String),
    String7_7(String),
    String3_8(String),
    String11_11(String),
    String3_13(String),
    String1_25(String),
    String1_28(String),
    String1_50(String),
    String1_80(String),
    String1_120(String),
    String3_129(String),
    String1_250(String),
    String1_255(String),
    String6_255(String),
    String1_1000(String),
    String1_2500(String),
    String1_4000(String),
    String1_5000(String),
    String1_10000(String),
    String1_16000(String),
    Digits2_2(String),
    Digits3_3(String),
    Digits4_4(String),
    Digits12_12(String),
    PasspRfNumber(String),
}

impl SvelteValidator {
    pub fn validate_svelte_field(self) -> Result<bool, Status> {
        match self {
            SvelteValidator::Inn(value) => match Inn::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Kpp(value) => match Kpp::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::CorAcc(value) => match CorAcc::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::RasAcc(value) => match RasAcc::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Bic(value) => match Bic::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Ogrn(value) => match Ogrn::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Date(value) => match Date::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::RubF(value) => match RubF::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::DocNum(value) => match DocNum::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::TextInfo(value) => match TextInfo::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::BranchType(value) => match BranchType::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Okpo(value) => match Okpo::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Oktmo(value) => match Oktmo::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Okogu(value) => match Okogu::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Okfs(value) => match Okfs::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Okved(value) => match Okved::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Phone(value) => match Phone::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::OpfCode(value) => match OpfCode::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::SurName(value) => match SurName::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::FirstName(value) => match FirstName::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::MidName(value) => match MidName::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Region(value) => match Region::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Snils(value) => match Snils::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::BoxUuid(value) => match BoxUuid::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::DateTime(value) => match DateTime::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Email(value) => match Email::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::ParticipantStatus(value) => match ParticipantStatus::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::PoaReqElemsFlag(value) => match PoaReqElemsFlag::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::CompanyName(value) => match CompanyName::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::IdentStatus(value) => match IdentStatus::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::PayType(value) => match PayType::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String7_7(value) => match String7_7::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String3_8(value) => match String3_8::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String11_11(value) => match String11_11::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String3_13(value) => match String3_13::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_25(value) => match String1_25::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_28(value) => match String1_28::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_50(value) => match String1_50::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_80(value) => match String1_80::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_120(value) => match String1_120::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String3_129(value) => match String3_129::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_250(value) => match String1_250::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_255(value) => match String1_255::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String6_255(value) => match String6_255::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_1000(value) => match String1_1000::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_2500(value) => match String1_2500::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_4000(value) => match String1_4000::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_5000(value) => match String1_5000::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_10000(value) => match String1_10000::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_16000(value) => match String1_16000::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Digits2_2(value) => match Digits2_2::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Digits3_3(value) => match Digits3_3::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Digits4_4(value) => match Digits4_4::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Digits12_12(value) => match Digits12_12::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::PasspRfNumber(value) => match PasspRfNumber::new(&value) {
                Ok(_) => Ok(true), Err(_) => Ok(false)
            }
        }
    }
}
