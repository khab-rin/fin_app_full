use crate::{Status, ClientState, ProcessError};
use crate::primitives::composite::implements::RasBicAcc;
use crate::parsers::dadata::implements::CtrprtyMetadata;
use crate::primitives::frozen::text::{CompInn, Kpp};

pub async fn get_bank_accs_by_comp_id(
    state: &ClientState,
    comp_inn: &Option<CompInn>,
    kpp: &Option<Kpp> 
) -> Result<Vec<RasBicAcc>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

    let comp_inn = match comp_inn {
        Some(c) => c,
        None => &session.session_user.company.comp_inn
    };

    let kpp = match kpp {
        Some(k) => k,
        None => &session.session_user.company.kpp
    };

    let metadata_str_option = sqlx::query_file_scalar!(
            "src/client/sql_queries/companys/get/bank_acc_by_comp_inn_kpp.sql",
            comp_inn,
            kpp
        ).fetch_optional(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    let metadata_str = match metadata_str_option {
        Some(m) => m,
        None => return Ok(vec!())
    };

    let metadata: CtrprtyMetadata = serde_json::from_str(&metadata_str)
        .map_err(|err| err.process_err(Status::MappingError, ""))?; 

    Ok(metadata.bank_acc)

}