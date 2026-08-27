
use crate::service::auth_service::general::ActiveSession;
use crate::{Status, ProcessError};
use crate::primitives::frozen::text::{DocNum, Currency, Integ, BoxUuid, CompInn, CompStatus, CompType, Date, DateTime, Kpp, PersInn, RubF};

pub async fn make_null_postgress_elements(
	session: &ActiveSession
) -> Result<(), Status> {

	let null_uuid = BoxUuid::unchecked(uuid::Uuid::nil());
	let null_pers_inn = PersInn::unchecked("202000000000".to_string());
	let null_comp_inn = CompInn::unchecked("0010000000".to_string());
	let null_kpp = Kpp::unchecked("000000000".to_string());
	let null_date = Date::unchecked(chrono::Local::now().date_naive());
	let null_date_time = DateTime::unchecked(chrono::Local::now());

	let null_rub: RubF = RubF::unchecked(rust_decimal::Decimal::ZERO);
	let null_i32: Integ = Integ::unchecked(0);

	let null_json = serde_json::Value::default();

	let null_comp_type = CompType::ComEnt;
	let null_comp_status = CompStatus::Liquidated;

	let _ = sqlx::query_file!(
		"src/client/sql_queries/persons/insert/person_no_sync.sql",
		null_uuid,
		null_pers_inn,
		null_json,
		null_date_time
	).fetch_optional(&session.local_db).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	sqlx::query_file!(
		"src/client/sql_queries/companys/add/add_company.sql",
		null_uuid,
		null_comp_inn,
		null_kpp,
		null_comp_type,
		null_comp_status,
		null_json,
		null_date_time
	).execute(&session.local_db).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;


	let _ = sqlx::query_file!(
		"src/client/sql_queries/contracts/add/new_contract.sql",
		null_uuid,
		null_uuid,
		null_uuid,
		null_uuid,
		"",
		null_date,
		"",
		null_date,
		null_date,
		"RUB",
		null_rub,
		null_i32,
		false,
		"",
		null_date,
		true
	).fetch_one(&session.local_db).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	sqlx::query_file!(
		"src/client/sql_queries/operations/add/new_operation.sql",
		null_uuid,
		null_uuid,
		null_uuid,
		null_uuid,
		null_uuid,
		"",
		"",
		null_rub,
		null_date,
		"",
		"",
		null_date,
		false,
		true,
		null_date
	).execute(&session.local_db).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	Ok(())
}
