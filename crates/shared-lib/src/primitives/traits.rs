use crate::Status;

pub(crate) trait ParseFromStrMapValue: Sized {
    fn parse_from_str_map_value(map_value: Option<&&str>) -> Result<Self, Status>;
}

impl<T> ParseFromStrMapValue for Option<T>
where 
    T: std::str::FromStr,
    Status: From<T::Err>
{
    fn parse_from_str_map_value(map_value: Option<&&str>) -> Result<Self, Status> {
        match map_value {
            Some(s) if !s.is_empty() => {
                Ok(Some(s.parse::<T>()
                    .map_err(Status::from)?))
            }
            _ => Ok(None)
        }
    }
}