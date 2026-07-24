use crate::state::ClientState;


use shared_lib::parsers::bank_statement::implements::{StatementFields, OperationParseData};
use shared_lib::sql_models::operation::implements::Operation;

pub(crate) fn make_operation(
    state: &ClientState,
    statement_fields: StatementFields,
    comment_data: OperationParseData
) {

}