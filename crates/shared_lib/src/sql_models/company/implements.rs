use uuid::Uuid;
use serde::{Serialize, Deserialize};
use chrono::Utc;

use crate::primitives::frozen::implements::{CompType, CompStatus, Inn, Kpp};
use crate::parsers::dadata::implements::CtrprtyMetadata;

use crate::Status;


#[derive(Serialize, Deserialize, Clone)]
#[derive(sqlx::FromRow, Debug)]
pub struct CompanyDto {
    pub comp_id: Uuid,
    pub inn: Inn,
    pub kpp: Kpp,
    pub comp_type: CompType,
    pub comp_status: CompStatus,
    pub metadata: serde_json::Value,
    pub last_update: chrono::DateTime<Utc>
}


#[derive(Serialize, Deserialize)]
#[derive(sqlx::FromRow, Debug)]
pub struct Company {
    pub comp_id: Uuid,
    pub inn: Inn,
    pub kpp: Kpp,
    pub comp_type: CompType,
    pub comp_status: CompStatus,
    pub metadata: CtrprtyMetadata,
    pub last_update: chrono::DateTime<Utc>
}

impl std::convert::TryFrom<CompanyDto> for Company {
    type Error = Status;
    fn try_from(dto: CompanyDto) -> Result<Self, Self::Error> {
        Ok(Company { 
            comp_id: dto.comp_id,
            inn: Inn::new(&dto.inn)?,
            kpp: Kpp::new(&dto.kpp)?,
            comp_type: dto.comp_type.clone(),
            comp_status: dto.comp_status.clone(),
            metadata: serde_json::
                from_value(dto.metadata.clone())
                .map_err(|_| Status::CompanyWrongMapping)?,
            last_update: dto.last_update
         })
    }
}

#[derive(Serialize, Deserialize)]
#[derive(sqlx::FromRow, Debug)]
pub struct CompanyCurt {
    pub comp_id: Uuid,
    pub inn: Inn,
    pub kpp: Kpp
}
