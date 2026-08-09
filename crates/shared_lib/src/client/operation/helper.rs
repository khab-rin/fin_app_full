use std::collections::HashMap;

use crate::{ClientState, ProcessError, Status};
use crate::primitives::composite::implements::RasBicAcc;
use crate::primitives::frozen::text::{CompInn, Kpp};
use crate::client::sql_queries::companys::add::new_company::add_company_by_inn_cpp_acc;



pub fn make_statement_block_map(
    s: &str
) -> HashMap<&str, &str> {
    let mut res: HashMap<&str, &str> = HashMap::new();

    for line in s.lines() {
        if let Some((key, value)) = line.split_once('=') {
            res.insert(key.trim(), value.trim());
        }
    }

    res
}

pub async fn add_bank_acc_by_inn_kpp(
    state: &ClientState,
    comp_inn: &Option<CompInn>,
    kpp: &Option<Kpp>,
    ras_bic_acc: &Option<RasBicAcc>
) -> Result<Vec<RasBicAcc>, Status> {

    let company = add_company_by_inn_cpp_acc(
            state, 
            comp_inn, 
            kpp, 
            ras_bic_acc)
        .await
        .map_err(|err| err.process_err(err, ""))?; 

    Ok(company.metadata.bank_acc)
}