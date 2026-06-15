use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements::{BoxUuid, DateTime, Email, Phone};
use crate::parsers::mchd::poa::PoaMchd;

#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
pub struct User {
    pub user_id: BoxUuid,

    pub mchd_tax_guid: Option<BoxUuid>,
    pub mchd_tax_file: Option<PoaMchd>,

    pub mchd_home_guid: Option<BoxUuid>,
    pub mchd_home_file: Option<PoaMchd>,

    pub last_update: DateTime
}

#[derive(Debug)]
pub struct UserSetData {
    pub pers_id: BoxUuid,
    pub comp_id: BoxUuid,

    pub phone: Phone,
    pub password_hash: String,
    pub email: Email,

    pub mchd_tax_guid: Option<BoxUuid>,
    pub mchd_tax_file: Option<String>,

    pub mchd_home_guid: Option<BoxUuid>,
    pub mchd_home_file: Option<String>,
}