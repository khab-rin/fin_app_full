use serde::{Serialize, Deserialize};

use crate::primitives::frozen::text::{Integ, BoxUuid, Currency, Date, DateTime, DocNum, RubF};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::FromRow, ts_rs::TS)]
pub struct Contract {
    pub contract_id: BoxUuid,
    
    pub user_id: BoxUuid,
    pub comp_id: BoxUuid,
    pub ctrpty_id: BoxUuid,

    pub contract_num: DocNum,

    pub contract_date: Date,
    pub title: String,

    pub st_date: Date,
    pub end_date: Date,

    pub currency: Currency,

    pub total_amount: RubF,

    pub payment_deferral_days: Integ,

    pub is_active: bool,
    pub descrip: String,
    
    pub entr_date: Date,

    pub is_del: bool,
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

    pub st_date: Date,
    pub end_date: Date,

    pub currency: String,

    pub total_amount: RubF,

    pub payment_deferral_days: u32,

    pub is_active: Integ,
    pub descrip: String,
    
    pub entr_date: Date,
    pub updated_at: DateTime,
    pub is_del: Integ,
    pub external_id: String 
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::FromRow, ts_rs::TS)]
pub struct NewContrData {
    pub ctrpty_id: BoxUuid,
    pub contract_num: DocNum,
    pub contract_date: Date,
    pub contract_title: String,
    pub contract_st_date: Date,
    pub contract_end_date: Date,
    pub contract_currency: Currency,
    pub contract_tot_amnt: RubF,
    pub contract_def_days: Integ,
    pub contract_descr: String
}
