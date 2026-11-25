use shared_lib::sql_models::operation::service::OperationInfo::SuccessRaw;
use shared_lib::{ClientState, ProcessError, Status};
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::sql_models::operation::service::{OperationStep, OperationInfo};
use shared_lib::service::mchd::home_mchd_power::HomeMchdPower;
use shared_lib::client::operation::statement_parser::parser::parse_statement;
use shared_lib::client::mchd::show_powers::check_access;

pub async fn make_bank_statement_operations(
    state: &ClientState,
    ras_bic_acc: &RasBicAcc,
    path: &str
) -> Result<OperationStep, Status> {

	match check_access(state, vec![
		HomeMchdPower::H210,
		HomeMchdPower::H110,
		HomeMchdPower::H510,
		HomeMchdPower::H810,

	]).await {
        Ok(true) => {},
        Ok(false) => {
            return Ok(OperationStep::TryLater { text: OperationInfo::AccessDenied })
        },
        Err(err) => {
            err.process_err(err, None);
			return Ok(OperationStep::from_err(&err));
        }
    }

    let all_operations = match parse_statement(
            state, 
            ras_bic_acc, 
            path).await {
        Ok(OperationStep::StatementSuccess { operations, ..}) => operations,
        Ok(res) => return Ok(res),
        Err(err) => {
            err.process_err(err, None);
            return Ok(OperationStep::from_err(&err));
        }
    };

    Ok(OperationStep::StatementSuccess { text: SuccessRaw, operations: all_operations })

  
}