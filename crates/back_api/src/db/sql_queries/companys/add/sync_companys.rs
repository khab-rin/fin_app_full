use futures::stream::{self, StreamExt};

use shared_lib::{ProcessError, Status};
use shared_lib::primitives::frozen::text::{BoxUuid, CompInn, Kpp, CompType, CompStatus, DateTime};
use shared_lib::sql_models::company::implements::{Company, CompanyDto, InnKppMapAcc, CompCrateData};


use crate::config::BackApiState;
use crate::db::sql_queries::companys::get::companys_by_inn_kpp::get_companys_by_inn_kpp;
use crate::db::parsers::dadata::inn_kpp_query::parse_company_by_inn_kpp;

use crate::db::sql_queries::companys::helper::{
    dto_to_company_vec, 
    fresh_bank_acc
};

pub(crate) async fn update_companys(
    state: &BackApiState, 
    data: Vec<CompCrateData>
) -> Result<Vec<Company>, Status> {

    let mut inn_kpp_acc_map: InnKppMapAcc = data
        .into_iter()
        .map(|x| ((x.comp_inn, x.kpp), x.bank_acc))
        .collect();


    let comp_inn_data: Vec<String> = inn_kpp_acc_map.keys().map(|x| x.0.to_string()).collect();
    let kpp_data: Vec<String> = inn_kpp_acc_map.keys().map(|x| x.1.to_string()).collect();

    let mut prev_companys = get_companys_by_inn_kpp(
            state,
            &comp_inn_data,
            &kpp_data)
        .await
        .map_err(|err| err.process_err(err, ""))?; 

    fresh_bank_acc(&mut inn_kpp_acc_map, &mut prev_companys); 

    let mut new_companys: Vec< Company> = vec!();

    let mut tasks_vec = vec!();

    for ((comp_inn, kpp), _) in inn_kpp_acc_map.iter() {
        let comp_inn_clone = comp_inn.clone();
        let kpp_clone = kpp.clone();
        tasks_vec.push(async move {
            parse_company_by_inn_kpp(state, &comp_inn_clone, &kpp_clone).await
        });
    };

    let mut dadata_stream = stream::iter(tasks_vec).buffer_unordered(4);

    while let Some(res) = dadata_stream.next().await {
        match res {
            Ok(c) => {
                new_companys.push(c)
            },
            Err(err) => {
                return Err(err.process_err(err, ""));
            } 
        }
    }

    fresh_bank_acc(&mut inn_kpp_acc_map, &mut new_companys); 

    for comp in prev_companys {
        new_companys.push(comp);
    }
       
   
    let mut comp_id: Vec<uuid::Uuid> = vec!();
    let mut comp_inn: Vec<String> = vec!();
    let mut kpp: Vec<String> = vec!();
    let mut comp_type: Vec<String> = vec!();
    let mut comp_status: Vec<String> = vec!();
    let mut metadata: Vec<serde_json::Value> = vec!();

    for comp in new_companys {
        comp_id.push(*comp.comp_id.as_ref());
        comp_inn.push(comp.comp_inn.to_string());
        kpp.push(comp.kpp.to_string());
        comp_type.push(comp.comp_type.as_str().to_string());
        comp_status.push(comp.comp_status.as_str().to_string());
        metadata.push(serde_json::to_value(&comp.metadata).unwrap_or_default());
        std::println!("inn = {:?}, kpp = {:?}", comp.comp_inn, comp.kpp);
    }


    let companys_dto = sqlx::query_file_as!(
            CompanyDto,
            "src/db/sql_queries/companys/add/sync_companys.sql",
            &comp_id[..],
            &comp_inn[..],
            &kpp[..],
            &comp_type[..],
            &comp_status[..],
            &metadata[..]
        ).fetch_all(&state.pool_long)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

    dto_to_company_vec(companys_dto)
}

    
   
