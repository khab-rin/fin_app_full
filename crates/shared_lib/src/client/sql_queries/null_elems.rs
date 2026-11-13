
use crate::service::auth_service::general::ActiveSession;
use crate::{Status, ProcessError};
use crate::primitives::frozen::text::{
	Snils, DocNum, Currency, Integ, BoxUuid, CompInn, CompStatus, 
	CompType, Date, DateTime, Kpp, PersInn, RubF, SurName, FirstName
};
use crate::primitives::composite::implements::Fio;
use crate::service::parsers::dadata::implements::CtrprtyMetadata;
use crate::sql_models::person::implements::PersonMetadata;

pub async fn make_null_postgress_elements(
	session: &ActiveSession
) -> Result<(), Status> {

	let null_uuid = BoxUuid::unchecked(uuid::Uuid::nil());
	let null_pers_inn = PersInn::unchecked("202000000000".to_string());
	let null_comp_inn = CompInn::unchecked("202000000000".to_string());
	let null_kpp = Kpp::unchecked("0".to_string());
	let null_date = Date::unchecked(chrono::Local::now().date_naive());
	let null_date_time = DateTime::unchecked(chrono::Local::now());
	let null_snills = Snils::unchecked("00000000000");

	let null_rub: RubF = RubF::unchecked(rust_decimal::Decimal::ZERO);
	let null_i32: Integ = Integ::unchecked(0);

	let null_pers_metadata: PersonMetadata = PersonMetadata {
		snils: null_snills,
		fio: Fio {
			sur_name: SurName::unchecked("".to_string()),
			first_name: FirstName::unchecked("".to_string()),
			mid_name: None,
		},
		passport: None,
		address: None,
		gender: None,
		birth_day: None,
		phone: std::collections::HashSet::new(),
		email: std::collections::HashSet::new(),
	};
	let null_pers_metadata_json = serde_json::to_value(&null_pers_metadata)
		.map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;


	let null_comp_type = CompType::ComEnt;
	let null_comp_status = CompStatus::Liquidated;

	let _ = sqlx::query_file!(
		"src/client/sql_queries/persons/insert/person_no_sync.sql",
		null_uuid,
		null_pers_inn,
		null_pers_metadata_json,
		null_date_time
	).fetch_optional(&session.local_db).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	let null_comp_metadata = CtrprtyMetadata {
		kpp: None,
		main_manager: None,
		branch_type: None,
		kind: None,
		is_active: None,
		opf: None,
		comp_name: None,
		inn: None,
		ogrn: None,
		oktmo_company: None,
		okogu: None,
		okfs: None,
		okved: None,
		ogrn_date_dadata: None,
		address: None,
		e_mail: None,
		phone: None,
		bank_acc: vec![],
		ogrn_date_date: None,
		okpo: None
	};
	let null_comp_metadata_json = serde_json::to_value(&null_comp_metadata)
		.map_err(|err: serde_json::Error| err.process_err(Status::MappingError, ""))?;

	sqlx::query_file!(
		"src/client/sql_queries/companys/add/que_insert_update.sql",
		null_uuid,
		null_comp_inn,
		null_kpp,
		null_comp_type,
		null_comp_status,
		null_comp_metadata_json,
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
