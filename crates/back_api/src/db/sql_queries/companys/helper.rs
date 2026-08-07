
use std::collections::HashSet;

use shared_lib::{Status, ProcessError};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::sql_models::company::implements::{Company, CompanyDto, InnKppMapAcc};


pub(crate) fn dto_to_company_vec(
    dtos: Vec<CompanyDto>
) -> Result<Vec<Company>, Status> {

    let mut res:Vec<Company> = vec!();

    for dto in dtos {
        let company = dto
            .clone()
            .try_into()
            .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;
        res.push(company)
    }

    Ok(res)
}

pub(crate) fn fresh_bank_acc(
    data: &mut InnKppMapAcc, 
    seen_companys: &mut [Company]
) {
    for company in seen_companys.iter_mut() {
        let pair = (company.comp_inn.clone(), company.kpp.clone());
        if let Some(new_acc) = data.remove(&pair) {
            let mut prev_acc = company
                .metadata
                .bank_acc
                .drain(..)
                .collect::<HashSet<RasBicAcc>>();
            for acc in new_acc {
                prev_acc.insert(acc);
            }
            company.metadata.bank_acc = prev_acc.into_iter().collect();
        }
    }
}