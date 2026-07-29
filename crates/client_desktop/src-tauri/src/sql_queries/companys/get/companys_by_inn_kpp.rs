use std::collections::HashSet;

use shared_lib::Status;
use shared_lib::primitives::frozen::text::{Kpp, CompInn, BoxUuid, CompStatus, CompType, DateTime};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};

use crate::state::ClientState;

pub(crate) async fn get_companys_by_inn_kpp(
    state: &ClientState,
    data: &HashSet<(CompInn, Kpp)>
) -> Result<Vec<Company>, Status> {

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN get_companys_by_inn_kpp FAILED BY EMPTY SESSION", err
            );
            return Err(err);
        }
    };

    let mut tx = match session.local_db.begin().await {
        Ok(t) => t,
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN get_companys_by_inn_kpp FAILED BY session.local_db.begin()",
                err, Status::SqLitePoolErr
            );
            return Err(Status::SqLitePoolErr);
        }
    };

    let mut res: Vec<Company> = vec!();

    

    for (comp_inn, kpp) in data.iter() {
        let var1= comp_inn.as_ref();
        let var2 = kpp.as_ref();
        let company_dto_option = match sqlx::query_file_as!(
            CompanyDto,
            "src/sql_queries/companys/get/company_by_inn_kpp.sql",
            var1,
            var2
        ).fetch_optional(&mut *tx).await {
            Ok(o) => o,
            Err(err) => {
                log::error!(
                    "tech_err = {:?}, local_err = {:?}, FUN get_companys_by_inn_kpp FAILED BY SQL QUERY",
                    err, Status::SqlQueryWrongLogic
                );
                return Err(Status::SqlQueryWrongLogic);
            }
        };

        let Some(company_dto) = company_dto_option else {
            continue;
        };

        let company: Company = match company_dto.try_into() {
            Ok(c) => c,
            Err(err) => {
                log::error!(
                    "tech_err = {:?}, local_err = {:?}, FUN get_companys_by_inn_kpp FAILED BY MAPPING Company",
                    err, Status::MappingError
                );
                return Err(Status::MappingError);
            }
        };

        res.push(company);  

    }

    match tx.commit().await {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, FUN get_companys_by_inn_kpp FAILED BY tx.commit().await",
                err, Status::SqliteCommitErr
            );
            return Err(Status::SqliteCommitErr);
        }
    }


    Ok(res)
}