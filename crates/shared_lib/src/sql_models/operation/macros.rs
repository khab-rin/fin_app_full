use crate::Status;

pub(crate) trait ParseFromStrMapValue: Sized {
    fn parse_from_str_map_value(map_value: Option<&&str>) -> Result<Self, Status>;
}

impl<T> ParseFromStrMapValue for Option<T>
where 
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    fn parse_from_str_map_value(map_value: Option<&&str>) -> Result<Self, Status> {
        match map_value {
            Some(val) if !val.is_empty() => {
                match val.parse::<T>() {
                    Ok(res) => Ok(Some(res)),
                    Err(_) => {
                        Ok(None)
                    }
                }
            }
            _ => Ok(None)
        }
    }
}


macro_rules! make_struct {
    ($avail:vis $struct_name:ident, 
        [$( ($field:ident, $id_type:ty, $serde_key:literal) ),* $(,)?]) => {

        #[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
        $avail struct $struct_name {
            $(
                #[serde(rename = $serde_key)]
                pub $field: $id_type,
            )*
        }

        impl $struct_name {
            pub fn from_map(map: &std::collections::HashMap<&str, &str>) 
                -> Result<Self, Status> {
                Ok(Self {
                    $(
                        $field: <$id_type as $crate::sql_models::operation::macros::ParseFromStrMapValue>
                        ::parse_from_str_map_value(
                            map.get($serde_key)
                        )?,
                    )*
                })
            }
        } 
    };
}






