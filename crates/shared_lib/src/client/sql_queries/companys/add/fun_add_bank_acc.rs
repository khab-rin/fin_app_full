use std::collections::HashSet;


use crate::{Status, ClientState, ProcessError};
use crate::primitives::composite::implements::RasBicAcc;
use crate::primitives::frozen::text::{CompInn, Kpp};


use crate::client::sql_queries::companys::get::fun_get_or_parse_insert_one::get_or_parse_insert_company_function;
use crate::client::sql_queries::companys::add::que_insert_update::sql_insert_update_company_query;

pub async fn add_new_bank_acc(
    state: &ClientState,
    comp_inn: &Option<CompInn>,
    kpp: &Option<Kpp>,
    ras_bic_acc: RasBicAcc
) -> Result<Vec<RasBicAcc>, Status> {

	let session = state.get_session().await
		.map_err(|err| err.process_err(err, None))?;

	let (comp_inn, kpp) = match (comp_inn, kpp) {
		(Some(i), Some(k)) => (i, k),
		_ => (&session.session_user.company.comp_inn, &session.session_user.company.kpp)
	};

	let comp_option = get_or_parse_insert_company_function(
		state, comp_inn, kpp
	).await.map_err(|err| err.process_err(err, None))?;

	let mut company = match comp_option {
		Some(c) => c,
		None => {return Ok(vec![]);}
	};

	let mut prev_accounts: HashSet<RasBicAcc> = company.metadata.bank_acc.into_iter().collect();

	prev_accounts.insert(ras_bic_acc);

	company.metadata.bank_acc = prev_accounts.into_iter().collect();

	sql_insert_update_company_query(&session.local_db, &company).await
		.map_err(|err| err.process_err(err, None))?;

	Ok(company.metadata.bank_acc)

}