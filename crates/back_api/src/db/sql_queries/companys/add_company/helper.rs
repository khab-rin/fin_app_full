use std::collections::HashSet;
use shared_lib::parsers::dadata::implements::CtrprtyMetadata;
use uuid::Uuid;
use chrono::DateTime;
use serde_json::Value;


use shared_lib::Status;
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::primitives::frozen::implements::{Inn, Kpp, CompType, Date};
use shared_lib::sql_models::company_models::implements::Company;
use shared_lib::alias_types::implements::{InnKppAccMap, InsertData};



pub(crate) fn make_inn_kpp_pairs(
    data: &InnKppAccMap
) -> (Vec<String>, Vec<String>) {

    let mut inn_data: Vec<String> = vec!();
    let mut kpp_data: Vec<String> = vec!();
    for ((inn, kpp), _) in data.iter() {
        inn_data.push(inn.to_string());
        kpp_data.push(kpp.to_string());
    }
    (inn_data, kpp_data)
}



pub(crate) fn fresh_bank_acc(
    data: &mut InnKppAccMap, 
    seen_companys: &mut [Company]
) {
    for company in seen_companys.iter_mut() {
        let pair = (company.inn.clone(), company.kpp.clone());
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


pub(crate) fn make_company(
    inn:Inn, 
    mut kpp:Kpp, 
    accounts:HashSet<RasBicAcc>, 
    func_res:Result<CtrprtyMetadata, Status>
) -> Result<Company, Status> {

    match func_res {
        Ok(mut metadata) => {
            let okved = metadata
                .okved
                .as_ref()
                .ok_or(Status::AddCompanyMissOkved)
                .inspect_err(|err| {
                    tracing::error!(
                        status_err = ?err,
                        inn = %inn, 
                        kpp = %kpp);
                })?;
            
            let opf_code = metadata
                    .opf
                    .as_ref()
                    .ok_or(Status::AddCompanyMissOpf)
                    .inspect_err(|err| {
                        tracing::error!(
                            status_err = ?err,
                            inn = %inn, 
                            kpp = %kpp);
                    })?
                    .opf_code
                    .as_ref()
                    .ok_or(Status::AddCompanyMissOpf)
                    .inspect_err(|err| {
                        tracing::error!(
                            status_err = ?err,
                            inn = %inn, 
                            kpp = %kpp);
                    })?;
            
            let comp_type = if okved.starts_with("64.1") || okved.starts_with("64.92") {
                    CompType::Bank
                } else if opf_code.starts_with('7') || opf_code.starts_with('6') {
                    CompType::Gov
                } else if opf_code.starts_with('5') {
                    CompType::Ip
                } else {
                    CompType::ComEnt
                };
                
                if comp_type != CompType::Bank {
                    for ras_bic_acc in accounts {
                        metadata.bank_acc.push(ras_bic_acc);
                    }
                }

                let company_state = metadata
                    .is_active
                    .as_ref()
                    .ok_or(Status::AddCompanyMissCompState)
                    .inspect_err(|err| {
                        tracing::error!(
                            status_err = ?err,
                            inn = %inn, 
                            kpp = %kpp);
                    })?
                    .status
                    .as_ref()
                    .ok_or(Status::AddCompanyMissCompState)
                    .inspect_err(|err| {
                        tracing::error!(
                            status_err = ?err,
                            inn = %inn, 
                            kpp = %kpp);
                    })?;

                if let Some(ms) = metadata.ogrn_date_dadata {
                    if let Some(dt) = DateTime::from_timestamp_millis(ms) {
                        let date_str = dt.naive_utc().date().to_string();
                        metadata.ogrn_date_date = Some(Date::new(date_str.as_str())?);
                    }
                }

                

                if Some(&kpp) != metadata.kpp.as_ref() {
                    if let Some(new_kpp) = &metadata.kpp {
                        kpp = new_kpp.clone();
                    }
                }

                Ok(Company {
                    comp_id: Uuid::new_v4(),
                    inn,
                    kpp,
                    comp_type,  
                    comp_status:company_state.clone(),
                    metadata,
                    last_update: chrono::Utc::now()
                })
        }
        Err(err) => {Err(err)}
    }
}


pub(crate) fn make_insert_data(
    new_companys:Vec<Company>
) -> InsertData {
    
    let mut seen_companys: HashSet<(Inn, Kpp)> = HashSet::new();
    let n = new_companys.len();
    let mut inn_d:Vec<String> = Vec::with_capacity(n);
    let mut kpp_d:Vec<String> = Vec::with_capacity(n);
    let mut type_d:Vec<String> = Vec::with_capacity(n);
    let mut status_d:Vec<String> = Vec::with_capacity(n);
    let mut mt_d:Vec<Value> = Vec::with_capacity(n);
    
    for company in new_companys.into_iter() {
        let pair:(Inn, Kpp) = (company.inn.clone(), company.kpp.clone());
        
        if !seen_companys.insert(pair) {
            continue;
        }

        inn_d.push(company.inn.to_string());
        kpp_d.push(company.kpp.to_string());
        type_d.push(company.comp_type.as_str().to_string());
        status_d.push(company.comp_status.as_str().to_string());
        mt_d.push(serde_json::to_value(&company.metadata).unwrap_or_default());
    }

    (inn_d, kpp_d, type_d, status_d, mt_d)
}
