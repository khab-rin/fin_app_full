use crate::service::auth_service::general::ActiveSession;
use crate::sql_models::id_rules::IdRules;
use crate::sql_models::person::implements::Person;
use crate::sql_models::company::implements::Company;
use crate::sql_models::contracts::implements::Contract;
use crate::sql_models::operation::implements::Operation;
use crate::sql_models::operation::account::{Account};
use crate::sql_models::operation::implements::DocType;
use crate::{ProcessError, Status};
use crate::service::parsers::dadata::implements::CtrprtyMetadata;
use crate::sql_models::person::implements::PersonMetadata;
use crate::primitives::composite::implements::Fio;
use crate::primitives::frozen::text::{Email, Phone, CompInn, CompStatus, CompType, 
	Currency, Date, DateTime, DocNum, Integ, Kpp, PersInn, RubF, Snils, SurName, FirstName};

use crate::client::sql_queries::companys::add::que_insert_update::sql_insert_update_company_query;
use crate::client::sql_queries::persons::add::que_insert_update::sql_insert_update_person_function;

pub async fn make_null_sqlite_elements(
	session: &ActiveSession
) -> Result<(), Status> {

	let null_pers_inn = PersInn::unchecked("202000000000".to_string());
	let null_comp_inn = CompInn::unchecked("0010000000".to_string());
	let null_kpp = Kpp::unchecked("000000000".to_string());
	let null_date = Date::unchecked(chrono::NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());
	let null_snils = Snils::unchecked("00000000000".to_string());
	let null_amont: RubF = RubF::unchecked(rust_decimal::Decimal::ZERO);
	let null_doc_num = DocNum::unchecked("0".to_string());
	let null_string = "0".to_string();
	let null_currency = Currency::RUB;
	let null_integer: Integ = Integ::unchecked(0);
	let null_acc = Account::BankAcc;
	let null_doc_type = DocType::Other;


	let person_id_rule = IdRules::Person { pers_inn: null_pers_inn.clone(), snils: null_snils.clone() };
	let null_pers_id = person_id_rule.make_stat_id();
	let comp_id_rule = IdRules::Company { comp_inn: null_comp_inn.clone(), kpp: null_kpp.clone() };
	let null_comp_id = comp_id_rule.make_stat_id();
	let user_id_rule = IdRules::User { comp_id: null_comp_id.clone(), pers_id: null_pers_id.clone() };
	let null_user_id = user_id_rule.make_stat_id();
	let contract_id_rule = IdRules::Contract { comp_id: null_comp_id.clone(), ctrpty_id: null_comp_id.clone(), contract_num: null_doc_num.clone(), contrac_date: null_date.clone() };
	let null_contract_id = contract_id_rule.make_stat_id();
	let operation_id_rule = IdRules::Operation { doc_num: Some(null_doc_num.clone()), oper_date: Some(null_date.clone()), amount: Some(null_amont.clone()), ctrpty_id: Some(null_comp_id.clone()) };
	let null_operation_id = operation_id_rule.make_stat_id();
	
	
	let null_person = Person {
		pers_id: null_pers_id.clone(),
		pers_inn: null_pers_inn,
		metadata: PersonMetadata {
			snils: null_snils.clone(),
			fio: Fio {
				sur_name: SurName::unchecked("nullname".to_string()),
				first_name: FirstName::unchecked("nullfirstname".to_string()),
				mid_name: None,
			},
			passport: None,
			address: None,
			gender: None,
			birth_day: None,
			phone: std::collections::HashSet::new(),
			email: std::collections::HashSet::new(),
		},
		last_update: DateTime::unchecked(chrono::Utc::now())
	};

	let null_company = Company {
		comp_id: null_comp_id.clone(),
		comp_inn: null_comp_inn,
		kpp: null_kpp,
		comp_type: CompType::ComEnt,
		comp_status: CompStatus::Liquidated,
		metadata: CtrprtyMetadata {
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
		},
		last_update: DateTime::unchecked(chrono::Utc::now())
	};

	let null_contract = Contract {
		contract_id: null_contract_id.clone(),
		comp_id: null_comp_id.clone(),
		user_id: null_user_id.clone(),
		ctrpty_id: null_comp_id.clone(),
		contract_num: null_doc_num.clone(),
		contract_date: null_date.clone(),
		title: null_string.clone(),
		st_date: null_date.clone(),
		end_date: null_date.clone(),
		currency: null_currency.clone(),
		total_amount: null_amont.clone(),
		payment_deferral_days: null_integer.clone(),
		is_active: false,
		descrip: null_string.clone(),
		entr_date: null_date.clone(),
		is_del: true
	};

	let null_operation = Operation {
		oper_id: null_operation_id.clone(),
		user_id: null_user_id.clone(),
		comp_id: null_comp_id.clone(),
		ctrpty_id: null_comp_id.clone(),
		contract_id: null_contract_id.clone(),
		debet: null_acc.clone(),
		credit: null_acc.clone(),
		amount: null_amont.clone(),
		oper_date: null_date.clone(),
		doc_type: null_doc_type.clone(),
		doc_num: null_doc_num.clone(),
		doc_date: null_date.clone(),
		is_storno: false, 
		is_del: true,
		entr_date: null_date.clone()
	};

	let mut conn = session.local_db.begin().await.map_err(|err| err.process_err(Status::SqlCommitErr, ""))?;

	sql_insert_update_company_query(&mut *conn, &null_company).await
		.map_err(|err| err.process_err(err, ""))?;

	sql_insert_update_person_function(&mut *conn, &null_person, 1).await
		.map_err(|err| err.process_err(err, ""))?;

	

	conn.commit().await.map_err(|err| err.process_err(Status::SqlCommitErr, ""))?;

	Ok(())
}

