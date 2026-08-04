use std::collections::{HashMap, HashSet};

use crate::{Status, ClientState};
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
) -> Result<Company, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN get_bank_accs_by_comp_id FAILED BY MISS SESSION", err
            );
            return Err(err);
        }
    };

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

    let companys = match add_companys_by_inn_cpp_acc(state, &data).await {
        Ok(c) => c,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN add_company_by_inn_cpp_acc FAILED BY FUN add_companys_by_inn_cpp_acc", err
            );
            return Err(err);
        }
    };

    let company_option = companys.into_iter().next();

    match company_option {
        Some(c) => Ok(c),
        None => {
            log::error!(
                "local_err = {:?}, FUN add_company_by_inn_cpp_acc FAILED BY SYSEM LOGIC ERROR", Status::SystemLogicErr
            );
            Err(Status::SystemLogicErr)
        }
    }

}