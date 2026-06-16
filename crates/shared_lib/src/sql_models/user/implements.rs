use serde::{Serialize, Deserialize};


use crate::primitives::frozen::implements::{BoxUuid, DateTime, Email, Phone};
use crate::parsers::mchd::implements::MchdPower;


#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
pub struct User {
    pub user_id: BoxUuid,

    pub mchd_tax_guid: Option<BoxUuid>,
    pub tax_powers: std::collections::HashSet<MchdPower>,

    pub mchd_home_guid: Option<BoxUuid>,
    pub home_powers: std::collections::HashSet<MchdPower>,

    pub last_update: DateTime
}

pub struct UserDto {
    pub user_id: BoxUuid,

    pub mchd_tax_guid: Option<BoxUuid>,
    pub tax_powers: serde_json::Value,

    pub mchd_home_guid: Option<BoxUuid>,
    pub home_powers: serde_json::Value,

    pub last_update: DateTime
}


impl std::convert::TryFrom<UserDto> for User {
    type Error = serde_json::Error;
    fn try_from(dto: UserDto) -> Result<Self, Self::Error> {
        Ok(User { 
            user_id: dto.user_id, 
            mchd_tax_guid: dto.mchd_tax_guid, 
            tax_powers: serde_json::from_value(dto.tax_powers)?,
            mchd_home_guid: dto.mchd_home_guid, 
            home_powers: serde_json::from_value(dto.home_powers)?, 
            last_update: dto.last_update 
        })
    }
}

#[derive(Debug)]
pub struct UserSetData {
    pub pers_id: BoxUuid,
    pub comp_id: BoxUuid,

    pub phone: Phone,
    pub password_hash: String,
    pub email: Email,

    pub mchd_tax_guid: Option<BoxUuid>,
    pub tax_powers: std::collections::HashSet<MchdPower>,

    pub mchd_home_guid: Option<BoxUuid>,
    pub home_powers: std::collections::HashSet<MchdPower>,
}

