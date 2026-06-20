use shared_lib::parsers::mchd::implements::{
    PoaMetadata,
    PoaTypeRevocable,
    PoaTypeRedelegatable

};

use shared_lib::service::mchd::service::NewMchdData;
use shared_lib::primitives::frozen::implements::{
    BoxUuid, Date
};
use shared_lib::static_data::mchd_const::MCHD_SYSTEM_INFO;
use shared_lib::primitives::frozen::implements_base::{
    Digits4_4, String1_1000
};
use sqlx::types::chrono;

pub(crate) fn make_poametadata(
    data: &NewMchdData
) -> PoaMetadata {

    let mchd_num = BoxUuid::unchecked(uuid::Uuid::new_v4());

    let tax_org_ident = Digits4_4::unchecked("0000");

    let today = Date::unchecked(chrono::Local::now().date_naive());

    let mchd_system_info_str = format!("{}{}", MCHD_SYSTEM_INFO, mchd_num);

    let mchd_system_info: String1_1000 = String1_1000::unchecked(mchd_system_info_str);


    let metadata:PoaMetadata = PoaMetadata { 
        revocable_type: PoaTypeRevocable::Revocable, 
        redelegate_type: PoaTypeRedelegatable::Single, 
        doc_num: data.poa_number.clone(), 
        mchd_num, 
        notar_number: None, 
        extra_num: None, 
        registration_date: None, 
        issue_date: today, 
        life_date: data.poa_end_date.clone(), 
        tax_org_ident: Some(tax_org_ident.clone()), 
        tax_org_idents: vec!(tax_org_ident), 
        mchd_system_info, 
        irrevocable_poa: None,  
    };

    metadata
}