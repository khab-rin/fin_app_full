use shared_lib::sql_models::person::implements::{Person, PersonDto};
use shared_lib::Status;


pub(crate) fn dtos_to_persons(persons_dto: Vec<PersonDto> ) -> Vec<Person> {
    let mut res: Vec<Person> = vec!();

    for dto in persons_dto {
        match dto.clone().try_into() {
            Ok(person) => { res.push(person) }
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    custom_err = ?Status::MappingError,
                    person = ?dto
                )
            }
        }
    }
    res
}