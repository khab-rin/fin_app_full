use shared_lib::service::mchd::implements::{
    PoaMetadata,
    PoaTypeRevocable,
    PoaTypeRedelegatable

};
use shared_lib::service::mchd::service::{NewMchdData, MchdType};
use shared_lib::primitives::frozen::implements::{
    BoxUuid, Date
};
use shared_lib::static_data::mchd_powers::document_propertys::{MCHD_SYSTEM_INFO, MCHD_TAX_ORG_IDENT};
use shared_lib::primitives::frozen::implements_base::{
    Digits4_4, String1_1000
};


pub(crate) fn make_poametadata(
    data: &NewMchdData,
    mchd_num: &BoxUuid,
    today: &Date
) -> PoaMetadata {

    
    let tax_org_ident = match data.mchd_type {
        MchdType::FnsMchd => Some(Digits4_4::unchecked(MCHD_TAX_ORG_IDENT)),
        _ => None
        
    };

    let tax_org_idents = match &tax_org_ident {
        Some(i) => vec!(i.clone()),
        None => vec!()
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
        tax_org_idents, 
        mchd_system_info, 
        irrevocable_poa: None,  
    };

    metadata
}