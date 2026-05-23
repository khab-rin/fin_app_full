use std::sync::Arc;
use futures::stream::{self, StreamExt};

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{BoxUuid, Inn, Kpp, CompType, CompStatus};
use shared_lib::sql_models::company::implements::{Company, CompanyDto};
use shared_lib::alias_types::implements::{InnKppAccMap, InnKppAccVec};

use crate::config::BackApiState;
use crate::db::parsers::dadata::parser::dadata_reqwest_func;
use crate::db::sql_queries::companys::get::companys_by_inn_kpp::get_companys_by_inn_kpp;

use crate::db::sql_queries::companys::add::helper::{
    make_inn_kpp_pairs,
    fresh_bank_acc,
    make_company,
    make_insert_data,
};
use crate::db::sql_queries::companys::helper::dto_to_company_vec;

pub(crate) async fn sync_server_companys(
    state: &Arc<BackApiState>, 
    data_vec:InnKppAccVec
) -> Result<Vec<Company>, Status> {

    let mut data:InnKppAccMap = data_vec.into_iter().collect();

    let inn_kpp_data = make_inn_kpp_pairs(&data);

    let mut companys = match get_companys_by_inn_kpp(state, &inn_kpp_data).await {
        Ok(c) => c,
        Err(err) => {
            tracing::error!(
                err = ?err,
                "FUN sync_server_companys FAILED BY get_companys_by_inn_kpp FUN"
            );
            return Err(err);
        }
    };

    fresh_bank_acc(&mut data, &mut companys); 

    let mut dadata_stream = stream::iter(data)
        .map(|((inn, kpp), accounts)| {
            async move {
                let func_res = dadata_reqwest_func(state, &inn, &kpp).await;
                (inn, kpp, accounts, func_res)
            }
        }).buffer_unordered(4);


    while let Some((inn, kpp, accounts, func_res)) = dadata_stream.next().await {
                match make_company(inn.clone(), kpp.clone(), accounts, func_res) {
            Ok(new_company) => companys.push(new_company),
            Err(err) => {
                tracing::error!(
                    err_name = ?err,
                    inn = %inn,
                    kpp = %kpp 
                )
            }
        }
    }
   
    let (
        inn_d, 
        kpp_d, 
        type_d, 
        status_d, 
        mt_d
        ) = make_insert_data(companys);

    let seen_companys_dto = sqlx::
        query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/add/by_cimpays.sql",
            &inn_d[..],
            &kpp_d[..],
            &type_d[..],
            &status_d[..],
            &mt_d[..]
        ).fetch_all(&state.pool)
        .await
        .inspect_err(|err| {
            tracing::error!(
                tech_err = ?err,
                stat_err = ?Status::SqlQueryWrongLogic
            );
        })
        .map_err(|_| Status::SqlQueryWrongLogic)?;

    
    
    Ok(dto_to_company_vec(seen_companys_dto))
}

    
   
