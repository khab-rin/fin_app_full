use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{FirstName, MidName, Ogrn, Region, Snils, SurName};
use shared_lib::primitives::frozen::implements_base::String1_255;
use shared_lib::service::mchd::service::{MchdType, NewMchdData};
use shared_lib::service::auth_service::client_state::ActiveSession;
use shared_lib::primitives::composite::implements::Fio;
use shared_lib::service::mchd::implements::{
    AddressChoice, 
    Flag, 
    IpPrincipal, 
    ManagementType, 
    PersonMchd, 
    PostalAddress, 
    Principal, 
    PrincipalIdentity, 
    PrincipalWrap, 
    RootManager, 
    RussOrgPrincipal,
    RussOrganization, 
    WrapPerson
};



pub(crate) fn make_principal_wrap(
    session: &ActiveSession,
    data: &NewMchdData
) -> Result<PrincipalWrap, Status> {

    let principal_identity = match session.session_user.company.comp_inn.len() {
        10 => PrincipalIdentity::RussianLegalEntity,
        _ => PrincipalIdentity::IndividualEntrepreneur
    };

    let principal = match make_principal(session, data) {
        Ok(p) => p,
        Err(err) => {
            log::error!(
                "FUN make_principal FAILED BY FUN make_principal, err = {:?}", err
            );
            return Err(err);
        }
    };

    let principal_info = PrincipalWrap { 
        principal_identity, 
        principal
    };

    Ok(principal_info)
}



pub(crate) fn make_principal(
    session: &ActiveSession,
    data: &NewMchdData
) -> Result<Principal, Status> {

    let russian_org = match make_russ_org_principal(session, data) {
        Ok(o) => o,
        Err(err) => {
            log::error!(
                "FUN make_principal FAILED BY FUN make_russ_org_principal, err = {:?}", err
            );
            return Err(err);
        }
    };

    let ip = match make_ip_principal(session, data) {
        Ok(i) => i,
        Err(err) => {
            log::error!(
                "FUN make_principal FAILED BY FUN make_ip_principal, err = {:?}", err
            );
            return Err(err);
        }
    };

    let principal = Principal {
        russian_org,
        foreign_org: None,
        ip,
        person: None
    };

    Ok(principal)
}

pub(crate) fn make_russ_org_principal(
    session: &ActiveSession,
    data: &NewMchdData
) -> Result<Option<RussOrgPrincipal>, Status> {

    if session.session_user.company.comp_inn.len() == 12 {
        return Ok(None)
    }
    
    let russ_organization = match make_russ_organization(session) {
        Ok(o) => o,
        Err(err) => {
            log::error!(
                "FUN make_principal FAILED BY FUN make_russ_org_principal, err = {:?}", err
            );
            return Err(err);
        }
    };
    
    let root_manager = make_rootmanager(data);

    let russ_organization_principal = RussOrgPrincipal {
        root_manager_yk: Flag::FalseFlag,
        root_manager_person: Flag::TrueFlag,
        root_manager_ip: Flag::FalseFlag,
        organization: russ_organization,
        root_managers: vec!(root_manager)

    };

    Ok(Some(russ_organization_principal))
}


pub(crate) fn make_russ_organization(
    session: &ActiveSession,
) -> Result<RussOrganization, Status> {

    let comp_name_data = match session.session_user.company.metadata.comp_name.clone() {
        Some(d) => d,
        None => {
            log::error!(
                "FUN make_russ_organization FAILED BY MISS session.session_user.company.metadata.comp_name, err = {}",
                Status::RequiredFieldsMiss
            );
            return Err(Status::RequiredFieldsMiss);
        }
    };

    let comp_name = match comp_name_data.full_egrul_name {
        Some(n) => n,
        None => {
            log::error!(
                "FUN make_russionorgenization FAILED BY MISS session.session_user.company.metadata.comp_name.full_egrul_name, err = {}",
                Status::RequiredFieldsMiss
            );
            return Err(Status::RequiredFieldsMiss);
        }
    };

    let a = RussOrganization { 
        principal_notarial_status: None, 
        name: comp_name, 
        comp_inn: Some(session.session_user.company.comp_inn.clone()), 
        kpp: session.session_user.company.kpp.clone(), 
        ogrn: session.session_user.company.metadata.ogrn.clone(), 
        reg_num: None, 
        founding_doc: None, 
        phone: None, 
        email: None, 
        direct_authority_doc: None, 
        address: Some(make_address(session))
    };

    Ok(a)
}


pub(crate) fn make_address(
    session: &ActiveSession,
) -> PostalAddress {

    let comp_inn_str = session.session_user.company.comp_inn.to_string();
    let region_base: String = comp_inn_str.chars().take(2).collect();
    let region = Region::unchecked(region_base);

    let fias_id = session.session_user.company.metadata.address.as_ref()
        .and_then(|addr_wrap| addr_wrap.address_data.as_ref())
        .and_then(|data| data.fias_id_full.clone());

    let address = session.session_user.company.metadata.address.as_ref()
        .and_then(|addr_wrap| addr_wrap.address_data.as_ref())
        .and_then(|address_data| address_data.address_egrul.clone())
        .map(AddressChoice::AdrRf);


    PostalAddress {
        region,
        fias_id: None,
        address
    }
}

pub(crate) fn make_rootmanager(
    data: &NewMchdData
) -> RootManager {
    RootManager {
        management_type: ManagementType::Sole,
        prime_manager_org: None,
        prime_manager_person: Some(make_root_manager_wrap_person(data)),
        prime_manager_ip: None
    }
}

pub(crate) fn make_root_manager_wrap_person(
    data: &NewMchdData
) -> WrapPerson {

    WrapPerson {
        principal_notarial_status: None,
        inn: Some(data.manager_inn.clone()),
        snils: Some(Snils::unchecked(data.manager_snils.beat_string())),
        position: Some(String1_255::unchecked(data.manager_tittle.beat_string())),
        direct_authority_doc: None,
        person: make_root_manager_person_mchd(data)
    }
}

pub(crate) fn make_root_manager_person_mchd(
    data: &NewMchdData
) -> PersonMchd {

    let is_citizen = match data.mchd_type {
        MchdType::FnsMchd => Some(data.manager_is_citizen),
        _ => None
    };

    PersonMchd {
        gender: None,
        is_citizen,
        ern_num: None,
        birth_day: Some(data.manager_birth_day.clone()),
        birth_place: None,
        country_code: None,
        tel_number: None,
        email: None,
        fio: Fio {
            first_name: FirstName::unchecked(data.manager_first_name.beat_string()),
            sur_name: SurName::unchecked(data.manager_sur_name.beat_string()),
            mid_name: Some(MidName::unchecked(data.manager_mid_name.beat_string()))
        },
        address: None,
        person_docums: None

    }
}



pub(crate) fn make_ip_principal(
    session: &ActiveSession,
    data: &NewMchdData
) -> Result<Option<IpPrincipal>, Status> {

    if session.session_user.company.comp_inn.len() == 10 {
        return Ok(None);
    }

    let ogrnip = match session.session_user.company.metadata.ogrn.clone() {
        Some(o) => Ogrn::unchecked(o.beat_string()),
        None => {
            log::error!(
                "FUN make_ip_principal FAILED BY MISS session.session_user.company.metadata.ogrn, err = {:?}", Status::RequiredFieldsMiss
            );
            return Err(Status::RequiredFieldsMiss);
        }
    };

    let fio = Fio {
        sur_name: SurName::unchecked(data.manager_sur_name.beat_string()),
        first_name: FirstName::unchecked(data.manager_first_name.beat_string()),
        mid_name: Some(MidName::unchecked(data.manager_mid_name.beat_string()))
    };

    let is_citizen = match data.mchd_type {
        MchdType::FnsMchd => Some(data.manager_is_citizen),
        _ => None
    };

    let person: PersonMchd = PersonMchd {
        gender: None,
        is_citizen,
        ern_num: None,
        birth_day: Some(data.manager_birth_day.clone()),
        birth_place: None,
        country_code: None,
        tel_number: None,
        email: None,
        fio,
        address: None,
        person_docums: None

    };

    let ip_principal = IpPrincipal {
        principal_notarial_status: None,
        name: None,
        ogrnip,
        inn: session.session_user.person.pers_inn.clone(),
        snils: Snils::unchecked(session.session_user.person.metadata.snils.beat_string()),
        direct_authority_doc: None,
        person
    };
    
    Ok(Some(ip_principal))
}