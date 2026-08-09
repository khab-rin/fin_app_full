use regex::Regex;
use std::sync::OnceLock;

use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{CompInn, Snils, PersInn};
use shared_lib::service::crypto_service::implements::CryptoSignFields;
use shared_lib::service::auth_service::implements::RegInitData;

pub(crate) fn parse_crypto_fields_org(text: &str) -> Result<CryptoSignFields, Status> {
    
    static RE_COMP_INN: OnceLock<Regex> = OnceLock::new();
    let comp_inn_reg = RE_COMP_INN.get_or_init(|| {
        Regex::new(r"(?:ИНН ЮЛ=|,|(?:\s))(?P<inn>\d{10})(?:,|\s|$)")
            .map_err(|err| err.process_err(Status::SystemErr, "")).unwrap()
    });

    let comp_inn_str_option = comp_inn_reg
        .captures(text)
        .and_then(|cap| cap.get(1))
        .map(|x| x.as_str());

    let comp_inn_option = match comp_inn_str_option {
        Some(s) => Some({
            CompInn::new(s).map_err(|err| err.process_err(err, ""))?
        }),
        None => None
    };

    static RE_MAN_TITLE: OnceLock<Regex> = OnceLock::new();
    let man_title_reg = RE_MAN_TITLE.get_or_init(|| {
        Regex::new(r"(?i)(?:T=|,?\s*)(ЛИКВИДАТОР|ДИРЕКТОР|ГЕНЕРАЛЬНЫЙ ДИРЕКТОР)(?:,|\s|$)")
            .map_err(|err| err.process_err(Status::SystemErr, "")).unwrap()  
    });
    let man_title = man_title_reg
        .captures(text)
        .and_then(|cap| cap.get(1))
        .map(|x| x.as_str().to_string());

    static RE_PERS_INN: OnceLock<Regex> = OnceLock::new();
    let pers_inn_reg = RE_PERS_INN.get_or_init(|| {
        Regex::new(r"(?:ИНН=|,|(?:\s))(?P<inn>\d{12})(?:,|\s|$)")
            .map_err(|err| err.process_err(Status::SystemErr, "")).unwrap()
    });
    
    let pers_inn_str = pers_inn_reg
        .captures(text)
        .and_then(|cap| cap.get(1))
        .map(|x| x.as_str())
        .ok_or_else(|| Status::Tech.process_err(Status::UserWrongData, ""))?;

    let pers_inn = PersInn::new(pers_inn_str)
        .map_err(|err| err.process_err(err, ""))?;

    static RE_SNILS: OnceLock<Regex> = OnceLock::new();
    let snils_reg = RE_SNILS.get_or_init(|| {
        Regex::new(r"(?:СНИЛС=|,|(?:\s))(?P<snils>\d{11})(?:,|\s|$)")
            .map_err(|err| err.process_err(Status::SystemErr, "")).unwrap()
    });

    let snils_str = snils_reg
        .captures(text)
        .and_then(|cap| cap.get(1))
        .map(|x| x.as_str())
        // Если тега СНИЛС вообще нет в логе — логируем и выходим
        .ok_or_else(|| Status::Tech.process_err(Status::UserWrongData, ""))?;

    let snils = Snils::new(snils_str).map_err(|err| err.process_err(err, ""))?;

    let comp_inn = match comp_inn_option {
        Some(i) => i,
        None => CompInn::unchecked(pers_inn.as_ref())
    };


    Ok(CryptoSignFields {
        comp_inn,
        man_title,
        pers_inn,
        snils
    })

}


pub fn check_auth_person(
    init_data: &RegInitData,
    fields: &CryptoSignFields
) -> bool {

    init_data.comp_inn == fields.comp_inn

}

pub fn check_auth_manager(
    init_data: &RegInitData,
    fields: &CryptoSignFields
) -> bool {

    tracing::info!("check_manager runing");

    tracing::info!(init_data = ?init_data, fields = ?fields);

    if init_data.comp_inn.len() == 10 {
        fields.man_title.is_some() && init_data.pers_inn == fields.pers_inn
    } else {
        fields.pers_inn == init_data.pers_inn && fields.snils == init_data.snils
    }

}