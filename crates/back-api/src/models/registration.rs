use axum_typed_multipart::{TryFromMultipart, FieldData};

use shared_lib::primitives::frozen::implements::{BoxUuid, Inn, Kpp};
use shared_lib::sql_models::person_models::implements::Person;

pub(crate) struct RegistrationRequest {
    pub(crate) person: Person,
    pub(crate) comp_inn: Inn,
    pub(crate) kpp: Kpp,
    pub(crate) password: String,
    pub(crate) device_id: BoxUuid,
    pub(crate)  document: FieldData<Vec<u8>>,  
    pub(crate)  signature: FieldData<Vec<u8>>,
}