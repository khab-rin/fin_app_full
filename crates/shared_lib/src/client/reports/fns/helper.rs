use chrono::Datelike;

use crate::primitives::frozen::text::{Date};
use crate::primitives::frozen::text_base::{Digits4_4, String1_255};
use crate::{ProcessError, Status};
use crate::service::auth_service::general::ActiveSession;
use crate::service::reports::service::QuartDates;
use crate::service::reports::fns_xsd_shemas::common::FnsKnd;

pub fn make_quaters(
	year: i32
) -> Result<QuartDates, Status> {
	let start_str = format!("01.01.{}", year);
	let start = Date::new(&start_str).map_err(|err| err.process_err(err, ""))?;

	let qu1_str = format!("31.03.{}", year);
	let qu2_str = format!("30.06.{}", year);
	let qu3_str = format!("30.09.{}", year);
	let qu4_str = format!("31.12.{}", year);
	let q1 = Date::new(&qu1_str).map_err(|err| err.process_err(err, ""))?; 
	let q2 = Date::new(&qu2_str).map_err(|err| err.process_err(err, ""))?; 
	let q3 = Date::new(&qu3_str).map_err(|err| err.process_err(err, ""))?; 
	let q4 = Date::new(&qu4_str).map_err(|err| err.process_err(err, ""))?; 

	Ok(QuartDates {start, q1, q2, q3, q4})
}

pub fn make_file_id(
	session: &ActiveSession,
	fns_branch_code: &Digits4_4,
	knd: FnsKnd
) -> Result<String1_255, Status> {

	let fns_code = fns_branch_code.to_string();

	let comp_inn = session.session_user.company.comp_inn.to_string();
	let kpp = session.session_user.company.kpp.to_string();

	let date = chrono::Utc::now().date_naive();
	let year = date.year();
	let month = date.month();
	let day = date.day();

	let uuid_code = uuid::Uuid::new_v4().simple().to_string();

	let file_id_str = format!("{}_{}_{}{}{}_{}_NO_{:04}{:02}{:02}_{}",
		"ON_UT",
		knd.to_string(),
		fns_code, comp_inn, kpp,
		fns_code,
		year, month, day,
		uuid_code
	);

	let file_id = String1_255::unchecked(file_id_str);

	Ok(file_id)
}