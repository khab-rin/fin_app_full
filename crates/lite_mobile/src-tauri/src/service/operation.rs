use shared_lib::sql_models::operation::service::OperationInfo::SuccessRaw;
use shared_lib::{Status, ClientState};
use shared_lib::sql_models::operation::implements::OperationRaw;
use shared_lib::primitives::composite::implements::RasBicAcc;
use shared_lib::sql_models::operation::service::{OperationStep, OperationInfo};
use shared_lib::sql_models::operation::account::Account;
use shared_lib::client::operation::statement_parser::parser::parse_statement;

pub async fn make_bank_statement_operations(
    state: &ClientState,
    ras_bic_acc: &RasBicAcc,
    path: &str
) -> Result<OperationStep, Status> {

    let failed_result = OperationStep::TryLater { text: OperationInfo::ClientApiSystemError };

    let session = match state.get_session().await {
        Ok(s) => s,
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUN make_bank_statement_operations FAILED BY MISS SESSION", err
            );
            return Ok(failed_result);
        }
    };

    let all_operations = match parse_statement(
            state, 
            ras_bic_acc, 
            path).await {
        Ok(OperationStep::SuccessRaw { operations, ..}) => operations,
        Ok(res) => return Ok(res),
        Err(err) => {
            log::error!(
                "local_err = {:?}, FUB make_bank_statement_operations FAILED BY FUN parse_statement", err
            );
            return Ok(failed_result);
        }
    };

    let mut in_operations: Vec<OperationRaw> = vec!();

    for operation_row in all_operations {
        if operation_row.debet != Account::BankAcc {
            continue;
        }
        in_operations.push(operation_row);
    }


    Ok(OperationStep::SuccessRaw { text: SuccessRaw, operations: in_operations })

  
}