use std::collections::HashSet;
use serde::{Serialize, Deserialize};

use crate::sql_models::company::implements::{Company, CompanyCurt};
use crate::Status;


#[derive(Default, Serialize, Deserialize)]
pub struct BankStatementProceedResult {
    pub companys: Vec<Company>,
    pub complete_operations: Vec<String>,
    pub partial_operations: Vec<String>,
    pub status: HashSet<Status>,
    pub companys_curt: Vec<CompanyCurt>
}