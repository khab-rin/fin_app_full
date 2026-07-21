
use shared_lib::Status;
use shared_lib::service::mchd::service::{MchdStep, MchdInfo};
use shared_lib::primitives::frozen::implements::BoxUuid;

use crate::config::BackApiState;

pub(crate) async fn show_powers(
    state: &BackApiState,
    user_is: &BoxUuid
) -> Result<MchdStep, Status> {
    
    Err(Status::Unknown)
}