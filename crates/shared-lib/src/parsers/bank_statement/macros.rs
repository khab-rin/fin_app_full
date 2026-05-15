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
                        $field: <$id_type as $crate::primitives::traits::ParseFromStrMapValue>
                        ::parse_from_str_map_value(
                            map.get($serde_key)
                        )?,
                    )*
                })
            }
        } 
    };
}






