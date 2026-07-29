use serde::{Serialize, Deserialize};

use crate::primitives::frozen::text::{BoxUuid, Date, RubF, DateTime};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ts_rs::TS)]
pub struct Contract {
    pub contract_id: BoxUuid,
    
    pub user_id: BoxUuid,
    pub comp_id: BoxUuid,
    pub ctrpty_id: BoxUuid,

    pub contract_num: String,

    pub contract_date: Date,
    pub title: String,

    pub start_date: Date,
    pub end_date: Date,

    pub currency: String,

    pub total_amount: RubF,

    pub payment_deferral_days: i32,

    pub is_active: i32,
    pub description: String,
    
    pub entr_date: Date,
    pub updated_at: DateTime,
    pub is_del: i32,
    pub external_id: String 
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ts_rs::TS)]
pub struct ContractDto {
    pub contract_id: BoxUuid,
    
    pub user_id: BoxUuid,
    pub comp_id: BoxUuid,
    pub ctrpty_id: BoxUuid,

    pub contract_num: String,

    pub contract_date: Date,
    pub title: String,

    pub start_date: Date,
    pub end_date: Date,

    pub currency: String,

    pub total_amount: RubF,

    pub payment_deferral_days: i32,

    pub is_active: i32,
    pub description: String,
    
    pub entr_date: Date,
    pub updated_at: DateTime,
    pub is_del: i32,
    pub external_id: String 
}