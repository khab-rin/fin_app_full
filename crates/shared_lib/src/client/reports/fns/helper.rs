use crate::primitives::frozen::text::{Date};
use crate::{ProcessError, Status};
use crate::service::reports::service::QuartDates;

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