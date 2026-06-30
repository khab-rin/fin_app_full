use std::collections::{HashSet, HashMap};

use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements::{BoxUuid, DateTime, Email, Phone};
use crate::service::mchd::implements::MchdPower;


#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
pub struct User {
    pub user_id: BoxUuid,

    pub guids: HashSet<BoxUuid>,

    #[serde(default)]
    pub tax_powers: std::collections::HashMap<MchdPower, BoxUuid>,
    #[serde(default)]
    pub home_powers: std::collections::HashMap<MchdPower, BoxUuid>,

    pub last_update: DateTime
}

pub struct UserDto {
    pub user_id: BoxUuid,
    pub guids: Vec<BoxUuid>,
    pub last_update: DateTime
}


impl std::convert::TryFrom<UserDto> for User {
    type Error = serde_json::Error;
    fn try_from(dto: UserDto) -> Result<Self, Self::Error> {
        Ok(User { 
            user_id: dto.user_id, 
            guids: dto.guids.into_iter().collect(),

            tax_powers: HashMap::new(),
            home_powers: HashMap::new(),

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

    pub guids: HashSet<BoxUuid>,
}

