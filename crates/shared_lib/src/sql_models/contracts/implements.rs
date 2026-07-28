use crate::primitives::frozen::implements::{BoxUuid, Date, RubF, DateTime};

pub struct Contract {
    contract_id: BoxUuid,
    
    user_id: BoxUuid,
    comp_id: BoxUuid,
    ctrpty_id: BoxUuid,

    contract_num: String,

    contract_date: Date,
    title: String,

    start_date: Date,
    end_date: Date,

    currency: String,

    total_amount: RubF,

    payment_deferral_days: i32,

    is_active: i32,
    description: String,
    
    entr_date: Date,
    updated_at: DateTime,
    is_del: i32,
    external_id: String 
}