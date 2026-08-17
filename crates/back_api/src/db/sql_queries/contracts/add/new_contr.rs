use shared_lib::{Status, ProcessError};
use shared_lib::primitives::frozen::text::{BoxUuid, PersInn, DateTime};
use shared_lib::sql_models::contracts::implements::Contract;

use crate::config::BackApiState;


pub(crate) async fn add_contract(
    state: &BackApiState,
    person: Contract
) -> Result<Contract, Status> {

    Err(Status::Unknown)
    
}