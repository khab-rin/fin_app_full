use shared_lib::Status;
use shared_lib::sql_models::person::implements::Person;

pub(crate) fn person_checker(
    text: &str,
    person: &Person
) -> Result<bool, Status> {
    let signer_line = text
        .lines()
        .find(|line| line.contains("Signer:"))
        .ok_or(Status::BackApiError)?;

    let re_inn = regex::Regex::new(r"\b\d{12}\b").map_err(|_| Status::BackApiError)?;
    let parsed_inn = re_inn
        .captures(signer_line)
        .ok_or(Status::BackApiError)?
        .get(0)
        .map_or("", |m| m.as_str());

    let clean_fio = signer_line
        .replace("Signer:", "")
        .trim()
        .to_string();

    let fio_parts: Vec<&str> = clean_fio.split(',').map(|s| s.trim()).collect();
    
    if fio_parts.len() < 2 {
        tracing::warn!("FUN person_checker: line splitting failed, too few comma segments");
        return Ok(false);
    }

    let parsed_sur_name = fio_parts[0]; // "ХАБИПОВ"
    
    let name_and_mid: Vec<&str> = fio_parts[1].split_whitespace().collect();
    let parsed_first_name = name_and_mid.get(0).copied().unwrap_or("");
    let parsed_mid_name = name_and_mid.get(1).copied().unwrap_or("");

    let sur_name: &str = person.metadata.fio.sur_name.as_ref();
    let first_name: &str = person.metadata.fio.first_name.as_ref();
    let mid_name: &str = person.metadata.fio.mid_name.as_ref().map_or("", |m| m.as_ref());
    let pers_inn: &str = person.pers_inn.as_ref();

    let is_sur_name_match = sur_name.to_lowercase() == parsed_sur_name.to_lowercase();
    let is_first_name_match = first_name.to_lowercase() == parsed_first_name.to_lowercase();
    let is_mid_name_match = mid_name.to_lowercase() == parsed_mid_name.to_lowercase();
    let is_inn_match = pers_inn == parsed_inn;

    let is_valid = is_sur_name_match 
                && is_first_name_match 
                && is_mid_name_match 
                && is_inn_match;
    
    Ok(is_valid)
}