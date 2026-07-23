

use crate::primitives::frozen::implements::{BoxUuid, RubF, DateTime, Date};
use crate::sql_models::operation::account::Account;


pub struct Operation {
    oper_id: BoxUuid,
    user_id: BoxUuid,
    comp_id: BoxUuid,
    debet: Account,
    credit: Account,
    amount: RubF,
    oper_date: Date,
    doc_type: String,
    doc_num: String,
    doc_date: Date,
    entr_date: DateTime,
    is_storno: bool,
    is_del: bool,
    is_sync: Option<bool>
}