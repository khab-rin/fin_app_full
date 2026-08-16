use std::collections::{HashMap, HashSet};

use crate::{Status, ClientState, ProcessError};
use crate::primitives::composite::implements::RasBicAcc;
use crate::primitives::frozen::text::{CompInn, Kpp};
use crate::sql_models::company::implements::{CompCrateData};
use crate::sql_models::company::implements::Company;

use crate::client::sql_queries::companys::add::new_companys::add_companys_by_inn_cpp_acc;

pub async fn add_company_by_inn_cpp_acc(
    state: &ClientState,
    comp_inn: &Option<CompInn>,
    kpp: &Option<Kpp>,
    ras_bic_acc: &Option<RasBicAcc>
) -> Result<Option<Company>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

    let comp_inn = match comp_inn {
        Some(c) => c.clone(),
        None => session.session_user.company.comp_inn.clone()
    };

    let kpp = match kpp {
        Some(k) => k.clone(),
        None => session.session_user.company.kpp.clone()
    };

    let mut bank_acc: HashSet<RasBicAcc> = HashSet::new();

    if let Some(r) = ras_bic_acc {
        bank_acc.insert(r.clone());
    }

    let data: Vec<CompCrateData> = vec!(CompCrateData{comp_inn, kpp, bank_acc});

    let companys = add_companys_by_inn_cpp_acc(state, &data).await
        .map_err(|err| err.process_err(err, ""))?; 

    let company_option = companys.into_iter().next();

    Ok(company_option) 

}