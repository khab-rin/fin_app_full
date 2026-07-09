use chrono::Datelike;

use shared_lib::Status;

use shared_lib::primitives::frozen::implements::{Date, BoxUuid};
use shared_lib::primitives::frozen::implements_base::String1_255;

use shared_lib::service::auth_service::client_state::ActiveSession;


use shared_lib::service::mchd::implements::{
    FormatVersion,
    PoaReqElemsFlag
};
use shared_lib::service::mchd::poa::PoaMchd;
use shared_lib::service::mchd::service::{NewMchdData, MchdType};
use shared_lib::static_data::mchd_powers::document_propertys::{MCHD_R_T, MCHD_TAX_R_T,MCHD_TAX_ORG_IDENT, MCHD_XMLNS, MCHD_XMLNS_XSD, MCHD_XMLNS_XSI};

use crate::service::mchd::make_poa_wrap::make_poa_wrap;


pub(crate) fn make_poa(
    session: &ActiveSession,
    data: &NewMchdData
) -> Result<PoaMchd, Status> {

    let mchd_num = BoxUuid::unchecked(uuid::Uuid::new_v4());
    let today = Date::unchecked(chrono::Local::now().date_naive());

    let years = today.as_ref().year();
    let month = today.as_ref().month();
    let days = today.as_ref().day();
    let comp_inn = session.session_user.company.comp_inn.as_ref();
    let kpp = session.session_user.company.kpp.as_ref();

    let flie_identificator = String1_255::unchecked(
        format!("{}_{}{:02}{:02}_{}", MCHD_R_T, years, month, days, mchd_num)
    );


    let tax_file_identificator = match data.mchd_type {
        MchdType::FnsMchd => Some(String1_255::unchecked(
            format!("{}_{}_{}_{}{}_{}{:02}{:02}_{}", 
            MCHD_TAX_R_T, 
            MCHD_TAX_ORG_IDENT, 
            MCHD_TAX_ORG_IDENT, 
            comp_inn, 
            kpp, 
            years,
            month,
            days,
            mchd_num))),
        _ => None
    };

    let required_elements = match data.mchd_type {
        MchdType::FnsMchd => PoaReqElemsFlag::BTBTax,
        _ => PoaReqElemsFlag::BTB
    };

    let poa = PoaMchd {
        xmlns_xsi: MCHD_XMLNS_XSI.to_string(),
        xmlns_xsd: MCHD_XMLNS_XSD.to_string(),
        xmlns: MCHD_XMLNS.to_string(),
        version_format: FormatVersion::Emchd1,
        required_elements,
        flie_identificator,
        tax_file_identificator,
        oid: None,
        text_info: None,
        poa: make_poa_wrap(session, data, &mchd_num, &today)?

    };

    Ok(poa)
}