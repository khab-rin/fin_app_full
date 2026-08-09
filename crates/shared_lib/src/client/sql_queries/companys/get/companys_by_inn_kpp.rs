use std::collections::HashSet;

use crate::{Status, ClientState, ProcessError};
use crate::primitives::frozen::text::{Kpp, CompInn, BoxUuid, CompStatus, CompType, DateTime};
use crate::sql_models::company::implements::{Company, CompanyDto};



pub
 async fn get_companys_by_inn_kpp(
    state: &ClientState,
    data: &HashSet<(CompInn, Kpp)>
) -> Result<Vec<Company>, Status> {

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

    let mut tx = session.local_db.begin()
        .await
        .map_err(|err| err.process_err(Status::SqLitePoolErr, ""))?; 


    let mut res: Vec<Company> = vec!();

    

    for (comp_inn, kpp) in data.iter() {
        let var1= comp_inn.as_ref();
        let var2 = kpp.as_ref();
        let company_dto_option = sqlx::query_file_as!(
                CompanyDto,
                "src/client/sql_queries/companys/get/company_by_inn_kpp.sql",
                var1,
                var2
            ).fetch_optional(&mut *tx)
            .await
            .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 


        let Some(company_dto) = company_dto_option else {
            continue;
        };

        let company: Company = company_dto
            .try_into()
            .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;

        res.push(company);  

    }

    tx.commit().await
        .map_err(|err| err.process_err(Status::SqliteCommitErr, ""))?; 


    Ok(res)
}