use std::collections::{HashMap};

use crate::config::Config;

use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::parsers::bank_statement::implements::ParsedOperation;
use shared_lib::alias_types::implements::InnKppAccMap;


pub(crate) fn make_inn_kpp_map_func(correct_lines:&[ParsedOperation]) -> InnKppAccMap {
    let own_inn = &Config::global().own_company.own_inn;
    let mut companys:InnKppAccMap = HashMap::new();

    for operation in correct_lines.iter() {
        let read_fields = &operation.read_fields;

        if read_fields.pay_inn != own_inn {
            let kpp = &read_fields.pay_kpp;
            let inn = &read_fields.pay_inn;
            match RasBicAcc
                ::new(&read_fields.pay_bic, &read_fields.pay_ras_acc) {
                    Ok(ras_bic_acc) => {companys
                        .entry((inn.clone(), kpp.clone()))
                        .or_default()
                        .insert(ras_bic_acc);}
                    Err(_) => continue
                };
            
        } else if read_fields.rec_inn != own_inn {
            let kpp = &read_fields.rec_kpp;
            let inn = &read_fields.rec_inn;
            match RasBicAcc
                ::new(&read_fields.rec_bic, &read_fields.rec_ras_acc) {
                    Ok(ras_bic_acc) => {companys
                        .entry((inn.clone(), kpp.clone()))
                        .or_default()
                        .insert(ras_bic_acc);}
                    Err(_) => continue
                };
        }
    }
 
    companys
}

