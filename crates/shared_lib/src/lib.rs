pub mod alias_types;
pub mod err_models;
pub mod parsers;
pub mod primitives;
pub mod static_data;
pub mod sql_models;
pub mod service;

#[cfg(feature = "client")]
pub mod client;


use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub use crate::err_models::implements::Status;
pub use crate::err_models::api_status::ProcessError;

#[cfg(feature = "client")]
pub use crate::service::auth_service::client_state::ClientState;

#[cfg(test)]
mod ts_tests {

use super::*;
    use ts_rs::TS;

    #[test]
    fn generate_types_for_svelte() {

        let output_dir = "../lite_mobile/ui/src/lib/models/rustModels";

        primitives::svelte_validate::SvelteValidator::export_all_to(output_dir)
            .expect("Не удалось экспортировать SvelteValidator");

        sql_models::operation::implements::OperationTSTS::export_all_to(output_dir)
            .expect("Не удалось экспортировать OperationTSTS");

        service::auth_service::implements::AuthTSRS::export_all_to(output_dir)
         .expect("Не удалось экспортировать AuthTSRS");

        service::mchd::implements::MchdTSRS::export_all_to(output_dir)
            .expect("Не удалось экспортировать MchdTSRS");

    }
}

#[derive(Debug, Clone, Ord, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct UsdF {
    data: Decimal
}


use crate::primitives::frozen::validators::init_rubf_from_str;
use crate::primitives::frozen::formatters::default_fmt;
use std::str::FromStr;


impl UsdF {
    pub(crate) const LABEL: &'static str = "USD";

    pub fn new(value: &str) -> Result<Self, Status> {
        init_rubf_from_str(value).map(|v| Self { data: v })
    }

    #[allow(dead_code)]
    pub(crate) fn label(&self) -> &'static str {
        Self::LABEL
    }

    pub fn unchecked(val: Decimal) -> Self {
        Self { data: val }
    }

    pub fn beat_string(&self) -> String {
        default_fmt(&self.data)
    }
}

impl std::str::FromStr for UsdF {
    type Err = Status;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl std::convert::TryFrom<String> for UsdF {
    type Error = Status;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl std::convert::TryFrom<&str> for UsdF {
    type Error = Status;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl std::ops::Deref for UsdF {
    type Target = Decimal;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl std::convert::AsRef<Decimal> for UsdF {
    fn as_ref(&self) -> &Decimal {
        &self.data
    }
}

impl std::convert::From<UsdF> for String {
    fn from(value: UsdF) -> Self {
        value.data.to_string()
    }
}

impl std::convert::From<&UsdF> for String {
    fn from(value: &UsdF) -> Self {
        value.data.to_string()
    }
}

impl std::fmt::Display for UsdF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.data)
    }
}

impl sqlx::Type<sqlx::Postgres> for UsdF {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <Decimal as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, sqlx::Postgres> for UsdF {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError>
    {
        <Decimal as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(&self.data, buf)
    }

    fn encode(self, buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError>
    where
        Self: Sized,
    {
        <Decimal as sqlx::Encode<'q, sqlx::Postgres>>::encode(self.data, buf)
    }
}

impl<'q> sqlx::Decode<'q, sqlx::Postgres> for UsdF {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'q>
    ) -> Result<Self, sqlx::error::BoxDynError> {

        let dec_val = <Decimal as sqlx::Decode<'q, sqlx::Postgres>>::decode(value)?;
        Ok(Self {data: dec_val})
    }
}


#[allow(dead_code)]
fn helper() {
    let a = UsdF::from_str("123").expect("msg");
    let b = i32::from_str("123").expect("msg");

    let c = String::from(&a);
    let d = &a.to_string();

    std::println!("{:?},{:?}", a, b);

}

 


