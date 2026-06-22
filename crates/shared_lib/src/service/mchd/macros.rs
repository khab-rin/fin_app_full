#[macro_export]
macro_rules! make_mchd_enum {
    ($name:ident, { $($variant:ident => $val:expr),* $(,)? }) => {
        #[derive(Debug, Serialize, Deserialize, Copy, PartialEq, Eq, sqlx::Type, Clone, ts_rs::TS)]
        pub enum $name {
            $(
                #[serde(rename = $val)]
                $variant,
            )*
        }

        impl std::str::FromStr for $name {
            type Err = $crate::err_models::implements::Status;

            fn from_str(val: &str) -> Result<Self, Self::Err> {
                match val {
                    $(
                        $val => Ok(Self::$variant),
                    )*
                    _ => Err($crate::err_models::implements::Status::ValidEnum),
                }
            }
        }
    };
}