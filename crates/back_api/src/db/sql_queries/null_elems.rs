
use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{DocNum, Currency, Integ, BoxUuid, CompInn, CompStatus, CompType, Date, DateTime, Kpp, PersInn, RubF};

use crate::config::BackApiState;

pub async fn make_null_postgress_elements(
	state: &BackApiState
) -> Result<(), Status> {
	let null_uuid = BoxUuid::unchecked(uuid::Uuid::nil());
	let null_pers_inn = PersInn::unchecked("000000000000".to_string());
	let null_comp_inn = CompInn::unchecked("0000000000".to_string());
	let null_kpp = Kpp::unchecked("000000000".to_string());
	let null_date = Date::unchecked(chrono::Local::now().date_naive());

	let null_rub: RubF = RubF::unchecked(rust_decimal::Decimal::ZERO);
	let null_i32: Integ = Integ::unchecked(0);


	let _ = sqlx::query_file!(
		"src/db/sql_queries/persons/add/by_person.sql",
		*null_uuid,
		*null_pers_inn,
		&serde_json::Value::default()
	).fetch_one(&state.pool_fast).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	let _ = sqlx::query_file!(
		"src/db/sql_queries/companys/add/company.sql",
		*null_uuid,
		*null_comp_inn,
		*null_kpp,
		CompType::ComEnt.as_str(),
		CompStatus::Liquidating.as_str(),
		&serde_json::Value::default()
	).fetch_one(&state.pool_fast).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	let _ = sqlx::query_file!(
		"src/db/sql_queries/users/add/user.sql",
		*null_uuid,
		*null_uuid,
		*null_uuid,
		"",
		"",
		"",
		&Vec::<uuid::Uuid>::new()[..]
	).fetch_one(&state.pool_fast).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	let _ = sqlx::query_file!(
		"src/db/sql_queries/contracts/add/new_contr.sql",
		*null_uuid,
		*null_uuid,
		*null_uuid,
		*null_uuid,
		"",
		*null_date,
		"",
		*null_date,
		*null_date,
		"RUB",
		*null_rub,
		*null_i32,
		false,
		"",
		*null_date,
		true
	).fetch_one(&state.pool_fast).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	sqlx::query_file!(
		"src/db/sql_queries/operations/add/add_one.sql",
		*null_uuid,
		*null_uuid,
		*null_uuid,
		*null_uuid,
		*null_uuid,
		"",
		"",
		*null_rub,
		*null_date,
		"",
		"",
		*null_date,
		false,
		true,
		*null_date
	).execute(&state.pool_fast).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	Ok(())
}
