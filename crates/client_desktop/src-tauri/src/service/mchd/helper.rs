use shared_lib::sql_models::person::implements::Person;
use shared_lib::service::mchd::service::NewMchdData;
use shared_lib::sql_models::person::implements::{PassportRf, PasspRfCode};

pub(crate) fn check_update_user(
    person: &mut Person,
    data: &NewMchdData
) -> bool {
    if 
        person.pers_inn != data.user_inn ||
        person.metadata.fio.sur_name != data.user_sur_name ||
        person.metadata.fio.first_name != data.user_first_name ||
        person.metadata.fio.mid_name.clone().unwrap() != data.user_mid_name ||
        person.metadata.snils != data.user_snils 
    {
        return false;
    }
    
    let passport = PassportRf {
        doc_code: PasspRfCode::PasspRf,
        doc_ser_num: data.user_passport_number.clone(),
        doc_date: None,
        issued_by: Some(data.user_passport_issueer.clone()),
        issued_code: Some(data.user_passport_ussuer_code.clone()),
        exp_date: None
    };

    person.metadata.passport = Some(passport);
    person.metadata.birth_day = Some(data.user_birth_day.clone());
    person.metadata.gender = Some(data.user_gender.clone());
    
    true
}

