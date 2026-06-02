
use shared_lib::Status;
use shared_lib::primitives::frozen::implements::{CompInn, Kpp, BoxUuid};
use shared_lib::sql_models::person::implements::Person;
use shared_lib::service::auth_service::implements::{
    AuthStep,
    TextInfo
};

pub(crate) type FailedData<'a> = (&'a Person, &'a CompInn, &'a Kpp, &'a BoxUuid);

pub(crate) fn validate_field<'a, T: PartialEq + std::fmt::Debug>(
    field_name: &str,
    verify_val: Option<T>,
    actual_val: &T,
    failed_data: &'a FailedData<'a>,
) -> Result<(), AuthStep> {
    
    match verify_val {
        Some(v) => {
            if &v != actual_val {
                tracing::warn!(
                    failed_data = ?failed_data,
                    "FUN register_new_user FAILED - ANOTHER {field_name}"
                );
                return Err(AuthStep::NeedRegistration { text: TextInfo::WrongPerson });
            }
        }
        None => {
            tracing::warn!(
                failed_data = ?failed_data,
                "FUN register_new_user FAILED - MISSING {field_name}"
            );
            return Err(AuthStep::TryLater {text: TextInfo::BackApiError});
            }
    }
    Ok(())
}