use shared_lib::{Status, ProcessError, LL};
use shared_lib::service::database::service::{DatabaseInfo, DatabaseStep};
use shared_lib::sql_models::person::implements::Person;

use crate::config::BackApiState;

use crate::db::sql_queries::persons::get::que::sql_get_person_by_id_function;
use crate::db::sql_queries::persons::add::que::sql_insert_update_person_function;

pub async fn sync_all_persons(
	state: &BackApiState,
	local_step: DatabaseStep
) -> Result<DatabaseStep, Status> {

	let mut failed_result: DatabaseStep = DatabaseStep::TryLater {text: DatabaseInfo::BackApiError};

	let (local_persons, user_id) = if let DatabaseStep::SyncPersons {persons, user_id, ..} = local_step {
		(persons, user_id)
	} else {
		Status::log(LL::Error, "WRONG STEP IN FUNCTION sync_all_persons");
		return Ok(failed_result);
	};

	let mut updated_persons: Vec<Person> = Vec::with_capacity(local_persons.len());

	let mut tx = match state.pool_fast.begin().await {
		Ok(t) => t,
		Err(err) => {
			err.process_err(Status::SqlPoolErr, "");
			return Ok(failed_result);
		}
	};

	for local_person in local_persons {
		let server_person_option = match sql_get_person_by_id_function(&mut *tx, &local_person.pers_id).await {
			Ok(opt) => opt,
			Err(err) => {
				err.process_err(err, "");
				failed_result.process_info(&err);
            	return Ok(failed_result); 
			}
		};
		if let Some(mut server_person) = server_person_option {
			server_person.metadata.merge(local_person.metadata);
			updated_persons.push(server_person);
		} else {
			updated_persons.push(local_person);
		}
	}

	for updated_person in updated_persons.iter() {
		if let Err(err) = sql_insert_update_person_function(&mut *tx, updated_person).await {
			err.process_err(err, "");
			failed_result.process_info(&err);
            return Ok(failed_result); 
		}
	}

	match tx.commit().await {
		Ok(_) => Ok(DatabaseStep::SyncPersons { text: DatabaseInfo::Notning, persons: updated_persons, user_id }),
		Err(err) => {
			err.process_err(Status::SqlPoolErr, "");
			Ok(failed_result)
		}
	}

}