use shared_lib::Status;
use shared_lib::sql_models::company_models::implements::{Company, CompanyDto};

pub(crate) fn dto_to_company_vec(
    dtos: Vec<CompanyDto>
) -> Vec<Company> {
    let mut res:Vec<Company> = vec!();
    for dto in dtos {
        match dto.clone().try_into() {
            Ok(company) => res.push(company),
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    local_er = ?Status::BackSqlAddHelperDtoToCompany,
                    elemet = ?dto
                )
            }
        }
    }
    res
}