use uuid::Uuid;

use shared_lib::sql_models::company::implements::{Company, CompanyCurt};
use shared_lib::primitives::frozen::implements::{CompInn, Kpp};
use shared_lib::err_models::implements::Status;

use crate::state::ClientState;

pub(crate) async fn sync_local_companys(
    state: &ClientState, 
    companys: &[Company]
) -> Result<Vec<CompanyCurt>, Status> {

    log::info!("Запуск синхронизации локальных контрагентов (количество: {})", companys.len());

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!("MISS SEESION IN FUN sync_local_companys");
            return Err(err)
        }
    };

    let pool = &session.local_db;

    let mut sqlite_session = pool
        .begin()
        .await
        .inspect_err(|err| {
            log::error!("tech_err = {:?}, stat_err = {:?}",
            err,
            Status::SqLitePoolErr)
        })
        .map_err(|_| Status::SqLitePoolErr)?;

    let mut id_inn_kpp_pairs:Vec<CompanyCurt> = vec!();

    for company in companys.iter() {
        let inn_ref = company.comp_inn.as_ref();
        let kpp_ref = company.kpp.as_ref();
        let copm_type_ref = company.comp_type.as_str();
        let comp_status_ref = company.comp_status.as_str();
        let metadata_text = serde_json::to_string(&company.metadata).unwrap_or_else(|_| "{}".to_string());

        let res = sqlx::query_file_as!(
            CompanyCurt,
            "src/sql_queries/companys/sync_company/query.sql",
            company.comp_id,
            inn_ref,
            kpp_ref,
            copm_type_ref,
            comp_status_ref,            
            metadata_text,
            company.last_update
        ).fetch_one(&mut *sqlite_session)
        .await;

        match res {
            Ok(uuid_inn_kpp_pair) => {id_inn_kpp_pairs.push(uuid_inn_kpp_pair);}
            Err(err) => {
                log::error!("tech_err = {:?}, stat_err = {:?}, inn = {:?}, kpp = {:?}",
                    err,
                    Status::SqlQueryWrongLogic,
                    inn_ref,
                    kpp_ref);
            }
        }
    }

    sqlite_session.commit()
        .await
        .inspect_err(|err| {
            log::error!(
                "tech_err = {:?}, stat_err = {:?}",
                err,
                Status::SqliteCommitErr
            );
        })
        .map_err(|_| Status::SqliteCommitErr)?;

    Ok(id_inn_kpp_pairs)

}
