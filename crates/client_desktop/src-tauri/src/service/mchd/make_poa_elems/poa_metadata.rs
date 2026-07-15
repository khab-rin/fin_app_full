use shared_lib::service::auth_service::client_state::ActiveSession;

use shared_lib::service::mchd::implements::{
    PoaMetadata,
    PoaTypeRevocable,
    PoaTypeRedelegatable

};


use shared_lib::service::mchd::service::{NewMchdData, MchdType};
use shared_lib::primitives::frozen::implements::{
    BoxUuid, Date
};
use shared_lib::static_data::mchd_powers::document_propertys::MCHD_SYSTEM_INFO;
use shared_lib::primitives::frozen::implements_base::{
    Digits4_4, String1_1000
};


pub(crate) fn make_poametadata(
    session: &ActiveSession,
    data: &NewMchdData,
    mchd_num: &BoxUuid,
    today: &Date
) -> PoaMetadata {

    let tax_org_ident = match data.mchd_type {
        MchdType::FnsMchd => Some(data.tax_org_ident.clone()),
        _ => None
        
    };

    let mchd_system_info_str = format!("{}{}", MCHD_SYSTEM_INFO, mchd_num);

    let mchd_system_info: String1_1000 = String1_1000::unchecked(mchd_system_info_str);


    let metadata:PoaMetadata = PoaMetadata { 
        revocable_type: PoaTypeRevocable::Revocable, 
        redelegate_type: PoaTypeRedelegatable::Single, 
        doc_num: data.poa_number.clone(), 
        mchd_num: mchd_num.clone(), 
        notar_number: None, 
        extra_num: None, 
        registration_date: None, 
        issue_date: today.clone(), 
        life_date: data.poa_end_date.clone(), 
        tax_org_ident, 
        tax_org_idents: vec!(), 
        mchd_system_info, 
        irrevocable_poa: None,  
    };

    metadata
}