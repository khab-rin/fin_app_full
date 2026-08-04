use crate::{Status, ClientState};
use crate::primitives::composite::implements::RasBicAcc;
use crate::parsers::dadata::implements::CtrprtyMetadata;
use crate::primitives::frozen::text::{CompInn, Kpp};

pub async fn get_bank_accs_by_comp_id(
    state: &ClientState,
    comp_inn: &Option<CompInn>,
    kpp: &Option<Kpp> 
) -> Result<Vec<RasBicAcc>, Status> {

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
        Some(c) => c,
        None => &session.session_user.company.comp_inn
    };

    let kpp = match kpp {
        Some(k) => k,
        None => &session.session_user.company.kpp
    };

    let metadata_str_option = match sqlx::query_file_scalar!(
        "src/client/sql_queries/companys/get/bank_acc_by_comp_inn_kpp.sql",
        comp_inn,
        kpp
    ).fetch_optional(&session.local_db).await {
        Ok(o) => o,
        Err(err) => {
            log::error!(
                "tech_err = {:?}, locdl_err = {:?}, FUN get_bank_accs_by_comp_id FAILED BY WRONG SQL QUERY LOGIC",
                err, Status::SqlQueryWrongLogic
            );
            return Err(Status::SqlQueryWrongLogic);
        }
    };

    let metadata_str = match metadata_str_option {
        Some(m) => m,
        None => return Ok(vec!())
    };

    let metadata: CtrprtyMetadata = match serde_json::from_str(&metadata_str) {
        Ok(m) => m,
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_er = {:?}, FUN get_bank_accs_by_comp_id FAILED BY CtrprtyMetadata MAPPING",
                err, Status::MappingError
            );
            return Err(Status::MappingError);
        }
    };


    Ok(metadata.bank_acc)

}