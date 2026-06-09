use serde::{Serialize, Deserialize};
use crate::Status;
// Импортируем все ваши примитивы из соответствующего модуля
use crate::primitives::frozen::implements::*;
use crate::primitives::frozen::implements_base::*;

#[derive(Serialize, Deserialize, ts_rs::TS, Debug)]
pub enum SvelteValidator {
    PersInn,
    CompInn,
    Kpp,
    CorAcc,
    RasAcc,
    Bic,
    Ogrn,
    Date,
    RubF,
    DocNum,
    TextInfo,
    BranchType,
    Okpo,
    Oktmo,
    Okogu,
    Okfs,
    Okved,
    Phone,
    OpfCode,
    SurName,
    FirstName,
    MidName,
    Region,
    Snils,
    BoxUuid,
    DateTime,
    Email,
    ParticipantStatus,
    PoaReqElemsFlag,
    CompanyName,
    IdentStatus,
    PayType,
    String7_7,
    String3_8,
    String11_11,
    String3_13,
    String1_25,
    String1_28,
    String1_50,
    String1_80,
    String1_120,
    String3_129,
    String1_250,
    String1_255,
    String6_255,
    String1_1000,
    String1_2500,
    String1_4000,
    String1_5000,
    String1_10000,
    String1_16000,
    Digits2_2,
    Digits3_3,
    Digits4_4,
    Digits12_12,
    PasspRfNumber,
    Password
}

impl SvelteValidator {
    pub fn validate_svelte_field(self, value: &str) -> Result<bool, Status> {
        match self {
            SvelteValidator::PersInn => match PersInn::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::CompInn=> match CompInn::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Kpp=> match Kpp::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::CorAcc=> match CorAcc::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::RasAcc=> match RasAcc::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Bic=> match Bic::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Ogrn=> match Ogrn::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Date=> match Date::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::RubF=> match RubF::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::DocNum=> match DocNum::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::TextInfo=> match TextInfo::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::BranchType=> match BranchType::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Okpo=> match Okpo::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Oktmo=> match Oktmo::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Okogu=> match Okogu::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Okfs=> match Okfs::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Okved=> match Okved::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Phone=> match Phone::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::OpfCode=> match OpfCode::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::SurName=> match SurName::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::FirstName=> match FirstName::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::MidName=> match MidName::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Region=> match Region::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Snils=> match Snils::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::BoxUuid=> match BoxUuid::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::DateTime=> match DateTime::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Email=> match Email::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::ParticipantStatus=> match ParticipantStatus::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::PoaReqElemsFlag=> match PoaReqElemsFlag::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::CompanyName=> match CompanyName::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::IdentStatus=> match IdentStatus::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::PayType=> match PayType::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String7_7=> match String7_7::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String3_8=> match String3_8::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String11_11=> match String11_11::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String3_13=> match String3_13::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_25=> match String1_25::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_28=> match String1_28::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_50=> match String1_50::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_80=> match String1_80::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_120=> match String1_120::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String3_129=> match String3_129::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_250=> match String1_250::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_255=> match String1_255::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String6_255=> match String6_255::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_1000=> match String1_1000::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_2500=> match String1_2500::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_4000=> match String1_4000::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_5000=> match String1_5000::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_10000=> match String1_10000::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::String1_16000=> match String1_16000::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Digits2_2=> match Digits2_2::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Digits3_3=> match Digits3_3::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Digits4_4=> match Digits4_4::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::Digits12_12=> match Digits12_12::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false),
            },
            SvelteValidator::PasspRfNumber=> match PasspRfNumber::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false)
            },
            SvelteValidator::Password=> match Password::new(value) {
                Ok(_) => Ok(true), Err(_) => Ok(false)
            }
        }
    }
}
