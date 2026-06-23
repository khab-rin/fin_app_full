use uuid::Uuid;
use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements::{BoxUuid, CompStatus, CompType, DateTime, CompInn, Kpp};
use crate::parsers::dadata::implements::CtrprtyMetadata;

#[derive(Serialize, Deserialize, Clone)]
#[derive(sqlx::FromRow, Debug)]
pub struct CompanyDto {
    pub comp_id: BoxUuid,
    pub comp_inn: CompInn,
    pub kpp: Kpp,
    pub comp_type: CompType,
    pub comp_status: CompStatus,
    pub metadata: serde_json::Value,
    pub last_update: DateTime
}


#[derive(Serialize, Deserialize, Clone, ts_rs::TS)]
#[derive(sqlx::FromRow, Debug)]
pub struct Company {
    pub comp_id: BoxUuid,
    pub comp_inn: CompInn,
    pub kpp: Kpp,
    pub comp_type: CompType,
    pub comp_status: CompStatus,
    pub metadata: CtrprtyMetadata,
    pub last_update: DateTime
}

impl std::convert::TryFrom<CompanyDto> for Company {
    type Error = serde_json::Error;
    fn try_from(dto: CompanyDto) -> Result<Self, Self::Error> {
        Ok(Company { 
            comp_id: dto.comp_id,
            comp_inn: dto.comp_inn,
            kpp: dto.kpp,
            comp_type: dto.comp_type,
            comp_status: dto.comp_status,
            metadata: serde_json::from_value(dto.metadata)?,
            last_update: dto.last_update
         })
    }
}

#[derive(Serialize, Deserialize)]
#[derive(sqlx::FromRow, Debug)]
pub struct CompanyCurt {
    pub comp_id: Uuid,
    pub comp_inn: CompInn,
    pub kpp: Kpp
}
