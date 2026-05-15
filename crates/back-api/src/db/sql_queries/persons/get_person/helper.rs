use shared_lib::sql_models::person_models::implements::{Person, PersonDto};
use shared_lib::Status;


pub(crate) fn dto_to_person(persons_dto: Vec<PersonDto> ) -> Vec<Person> {
    let mut res: Vec<Person> = vec!();

    for dto in persons_dto {
        match dto.clone().try_into() {
            Ok(person) => { res.push(person) }
            Err(err) => {
                tracing::error!(
                    tech_err = ?err,
                    custom_err = ?Status::BackSqlGetPersonTryFromDto,
                    person = ?dto
                )
            }
        }
    }
    res
}