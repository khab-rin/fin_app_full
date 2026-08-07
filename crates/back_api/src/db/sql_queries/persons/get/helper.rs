use shared_lib::sql_models::person::implements::{Person, PersonDto};
use shared_lib::{ProcessError, Status};


pub(crate) fn dtos_to_persons(persons_dto: Vec<PersonDto> ) -> Result<Vec<Person>, Status> {
    let mut res: Vec<Person> = vec!();

    for dto in persons_dto {
        let person = dto.clone()
            .try_into()
            .map_err(|err:serde_json::Error| err.process_err(Status::MappingError, ""))?;
        res.push(person)
    }
    Ok(res)
}