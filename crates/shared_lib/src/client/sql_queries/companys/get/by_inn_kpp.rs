use crate::{Status, ClientState, ProcessError};
use crate::primitives::frozen::text::{Kpp, CompInn, BoxUuid, CompStatus, CompType, DateTime};
use crate::sql_models::company::implements::{Company, CompanyDto};

pub async fn get_company_by_inn_kpp(
    state: &ClientState,
    comp_inn: &Option<CompInn>,
    kpp: &Kpp
) -> Result<Option<Company>, Status> {

    let comp_inn = match comp_inn {
        Some(i) => i,
        None => return Ok(None)
    };

    let session = state.get_session().await
        .map_err(|err| err.process_err(err, ""))?; 

    let connect_options = session.local_db.connect_options();

    // 2. Берем базовое имя файла (оно может быть относительным, например "crates/shared_lib/data_base.db")
    let raw_path = connect_options.get_filename();

    // 3. Превращаем его в полный абсолютный путь
    // Если файл существует, canonicalize вернет PathBuf с полным путем
    let full_path = std::fs::canonicalize(raw_path)
        .unwrap_or_else(|_| std::path::PathBuf::from(raw_path));

    let var1= comp_inn.as_ref();
    let var2 = kpp.as_ref();

    let company_dto_option = sqlx::query_file_as!(
            CompanyDto,
            "src/client/sql_queries/companys/get/company_by_inn_kpp.sql",
            var1,
            var2
        ).fetch_optional(&session.local_db)
        .await
        .map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?; 

    let company_dto = match company_dto_option {
        Some(d) => d,
        None => return Ok(None)
    };

    let company: Company = company_dto.try_into()
        .map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?; 

    Ok(Some(company))
}