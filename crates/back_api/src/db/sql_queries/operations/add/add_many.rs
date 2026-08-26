use shared_lib::{Status, ProcessError};

use shared_lib::sql_models::operation::implements::Operation;

use crate::config::BackApiState;

pub async fn add_operations_many(
	state: &BackApiState,
	operations: Vec<Operation>
) -> Result<(), Status> {
	let mut oper_ids: Vec<uuid::Uuid> = vec!();
	let mut user_ids: Vec<uuid::Uuid> = vec!();
	let mut comp_ids: Vec<uuid::Uuid> = vec!();
	let mut ctrpty_ids: Vec<uuid::Uuid> = vec!();
	let mut contract_ids: Vec<uuid::Uuid> = vec!();
	let mut debets: Vec<String> = vec!();
	let mut credits: Vec<String> = vec!();
	let mut amounts: Vec<rust_decimal::Decimal> = vec!();
	let mut oper_dates: Vec<chrono::NaiveDate> = vec!();
	let mut doc_types: Vec<String> = vec!();
    let mut doc_nums: Vec<String> = vec!();
    let mut doc_dates: Vec<chrono::NaiveDate> = vec!();
    let mut is_stornos: Vec<bool> = vec!();
    let mut is_dels: Vec<bool> = vec!();
    let mut entr_dates: Vec<chrono::NaiveDate> = vec!();

	for oper in operations {
		oper_ids.push(*oper.oper_id);
		user_ids.push(*oper.user_id);
		comp_ids.push(*oper.comp_id);
		ctrpty_ids.push(*oper.ctrpty_id);
		contract_ids.push(*oper.contract_id);
		debets.push(oper.debet.as_ref().to_string());
		credits.push(oper.credit.to_string());
		amounts.push(*oper.amount.as_ref());
		oper_dates.push(*oper.oper_date.as_ref());
		doc_types.push(oper.doc_type.as_str().to_string());
		doc_nums.push(oper.doc_num.as_str().to_string());
		doc_dates.push(*oper.doc_date);
		is_stornos.push(oper.is_storno);
		is_dels.push(oper.is_del);
		entr_dates.push(*oper.entr_date.as_ref());
	}

	sqlx::query_file!(
		"src/db/sql_queries/operations/add/add_many.sql",
		&oper_ids[..],
		&user_ids[..],
		&comp_ids[..],
		&ctrpty_ids[..],
		&contract_ids[..],
		&debets[..],
		&credits[..],
		&amounts[..],
		&oper_dates[..],
		&doc_types[..],
		&doc_nums[..],
		&doc_dates[..],
		&is_stornos[..],
		&is_dels[..],
		&entr_dates[..],
	).execute(&state.pool_long).await
	.map_err(|err| err.process_err(Status::SqlQueryWrongLogic, ""))?;

	Ok(())
}