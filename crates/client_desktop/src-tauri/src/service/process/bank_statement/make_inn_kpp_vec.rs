use std::collections::{HashMap};

use shared_lib::Status;
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::parsers::bank_statement::implements::ParsedOperation;
use shared_lib::alias_types::implements::InnKppAccMap;

use crate::state::ClientState;

pub(crate) async fn make_inn_kpp_map_func(
    state: &ClientState,
    correct_lines:&[ParsedOperation]
) -> Result<InnKppAccMap, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!("MISS SEESION IN FUN make_inn_kpp_map_func");
            return Err(err)
        }
    };

    let own_comp_inn = &session.session_user.company.comp_inn;

    let mut companys:InnKppAccMap = HashMap::new();

    for operation in correct_lines.iter() {
        let read_fields = &operation.read_fields;

        if read_fields.pay_inn != own_comp_inn {
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
            
        } else if read_fields.rec_inn != own_comp_inn {
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
 
    Ok(companys)
}

