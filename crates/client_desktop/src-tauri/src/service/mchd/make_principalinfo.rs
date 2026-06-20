use shared_lib::Status;
use shared_lib::primitives::frozen::implements::Region;
use shared_lib::service::mchd::service::NewMchdData;
use shared_lib::parsers::mchd::implements::{
    Flag, 
    IpPrincipal, 
    PostalAddress, 
    Principal, 
    PrincipalIdentity, 
    PrincipalInfo, 
    RussOrgPrincipal, 
    RussOrganization,
    AddressChoice
};
use shared_lib::service::auth_service::client_state::ActiveSession;


pub(crate) fn make_principal_info(
    session: &ActiveSession,
    data: &NewMchdData
) -> PrincipalInfo {
    let principal_identity = match session.session_user.company.comp_inn.len() {
        10 => PrincipalIdentity::RussianLegalEntity,
        _ => PrincipalIdentity::IndividualEntrepreneur
    };

    PrincipalInfo { 
        principal_identity, 
        principal: make_principal(session, data)
    }
}



pub(crate) fn make_principal(
    session: &ActiveSession,
    data: &NewMchdData
) -> Principal {
    Principal {
        russian_org: make_russ_org_principal(session, data),
        foreign_org: None,
        ip: make_ip_principal(session, data),
        person: None
    }
}

pub(crate) fn make_russ_org_principal(
    session: &ActiveSession,
    data: &NewMchdData
) -> Option<RussOrgPrincipal> {
    
    None
}

pub(crate) fn make_ip_principal(
    session: &ActiveSession,
    data: &NewMchdData
) -> Option<IpPrincipal> {
    
    None
}


pub(crate) fn make_russionorgenization(
    session: &ActiveSession,
    data: &NewMchdData
) -> Result<RussOrganization, Status> {

    let comp_name_data = match session.session_user.company.metadata.comp_name.clone() {
        Some(d) => d,
        None => {
            log::error!(
                "FUN make_russionorgenization FAILED BY MISS session.session_user.company.metadata.comp_name, err = {}",
                Status::DadataMissFields
            );
            return Err(Status::DadataMissFields);
        }
    };

    let comp_name = match comp_name_data.full_egrul_name {
        Some(n) => n,
        None => {
            log::error!(
                "FUN make_russionorgenization FAILED BY MISS session.session_user.company.metadata.comp_name.full_egrul_name, err = {}",
                Status::DadataMissFields
            );
            return Err(Status::DadataMissFields);
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
        phone: session.session_user.company.metadata.phone.clone(), 
        email: session.session_user.company.metadata.e_mail.clone(), 
        direct_authority_doc: None, 
        address: make_address(session, data)
    };

    return Ok(a);
}

pub(crate) fn make_address(
    session: &ActiveSession,
    data: &NewMchdData
) -> Option<PostalAddress> {

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


    Some(PostalAddress {
        region,
        fias_id,
        address
    })
}