use crate::primitives::frozen::implements::{BoxUuid, DateTime, Email, Phone};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: BoxUuid,

    pub mchd_tax_guid: Option<BoxUuid>,
    pub mchd_tax_file: Option<String>,

    pub mchd_home_guid: Option<BoxUuid>,
    pub mchd_home_file: Option<String>,

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