use std::borrow::Cow;

use shared_lib::Status;

pub(crate) fn parse_cryptcp_output(
    _text: &Cow<'_, str>
) -> Result<String, Status> {
    Ok("".to_string())
}