use crate::primitives::frozen::implements::{BoxUuid, DateTime, Phone};
use crate::primitives::frozen::implplemets_base::String3_129;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    user_id: BoxUuid,

    mchd_tax_guid: Option<BoxUuid>,
    mchd_tax_file: Option<String>,

    mchd_home_guid: Option<BoxUuid>,
    mchd_home_file: Option<String>,

    last_update: DateTime
}