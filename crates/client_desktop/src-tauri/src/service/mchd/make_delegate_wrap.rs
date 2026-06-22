use shared_lib::primitives::composite::implements::Fio;
use shared_lib::primitives::frozen::implements_base::String1_25;
use shared_lib::service::mchd::service::NewMchdData;
use shared_lib::service::mchd::implements::{
    RussDocumCode,
    Delegate,
    DelegateType,
    DelegateWrap,
    PersonDocum,
    PersonMchd,
    WrapPerson
};

pub(crate) fn make_delegate_wrap(
    data: &NewMchdData
) -> DelegateWrap {

    DelegateWrap {
        delegate_type: DelegateType::PhysicalPerson,
        delegate: make_delegate(data)
    }
}

pub(crate) fn make_delegate(
    data: &NewMchdData
) -> Delegate {

     Delegate {
        organization: None,
        ip: None,
        person: Some(make_delegate_wrap_person(data)),
        filial: None,
        foreign_organization: None
    }
}

pub(crate) fn make_delegate_wrap_person(
    data: &NewMchdData
) -> WrapPerson {

    WrapPerson {
        principal_notarial_status: None,
        inn: Some(data.user_inn.clone()),
        snils: Some(data.user_snils.clone()),
        position: None,
        direct_authority_doc: None,
        person: make_delegate_person_mchd(data)
    }
}


pub(crate) fn make_delegate_person_mchd(
    data: &NewMchdData
) -> PersonMchd {

    PersonMchd {
        gender: Some(data.user_gender),
        is_citizen: Some(data.user_is_citizen),
        ern_num: None,
        birth_day: Some(data.user_birth_day.clone()),
        birth_place: None,
        country_code: None,
        tel_number: None,
        email: None,
        fio: Fio {
            sur_name: data.user_sur_name.clone(),
            first_name: data.user_first_name.clone(),
            mid_name: Some(data.user_mid_name.clone()),
        },
        address: None,
        person_docums: Some(make_delegate_passport(data))
    }
}


pub(crate) fn make_delegate_passport(
    data: &NewMchdData
) -> PersonDocum {

    PersonDocum {
        doc_code: RussDocumCode::PasspRf,
        doc_ser_num: String1_25::unchecked(data.user_passport_number.to_string()),
        doc_date: data.manager_birth_day.clone(),
        issued_by: Some(data.user_passport_issueer.clone()),
        issued_code: Some(data.user_passport_ussuer_code.clone()),
        exp_date: None

    }


}
