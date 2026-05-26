
use shared_lib::primitives::frozen::implements::{Inn, Kpp, BoxUuid};
use shared_lib::sql_models::person::implements::Person;
use shared_lib::service::auth_service::implements::{
    AuthStep
};

pub(crate) type FailedData<'a> = (&'a Person, &'a Inn, &'a Kpp, &'a BoxUuid);

pub(crate) fn validate_field<'a, T: PartialEq + std::fmt::Debug>(
    field_name: &str,
    verify_val: Option<T>,
    actual_val: &T,
    device_id: &BoxUuid,
    failed_data: &'a FailedData<'a>,
) -> Result<(), AuthStep> {
    
    match verify_val {
        Some(v) => {
            if &v != actual_val {
                tracing::warn!(
                    failed_data = ?failed_data,
                    "FUN register_new_user FAILED - ANOTHER {field_name}"
                );
                return Err(AuthStep::WrongPerson {});
            }
        }
        None => {
            tracing::warn!(
                failed_data = ?failed_data,
                "FUN register_new_user FAILED - MISSING {field_name}"
            );
            return Err(AuthStep::TryLater {});
            }
    }
    Ok(())
}