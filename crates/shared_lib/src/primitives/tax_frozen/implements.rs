use serde::{Serialize, Deserialize};
use serde::ser::Serializer;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;

use crate::Status;
use crate::primitives::tax_frozen::validator_rules::*;

tax_primitives!(
    Nds22,
    init_nds22_from_str,
    init_nds_22_default,
    "НДС_22%");

tax_primitives!(
    Usn6,
    init_usn6_from_str,
    init_usn6_default,
    "УСН_06%");

tax_primitives!(
    Usn15,
    init_usn15_from_str,
    init_usn15_default,
    "УСН_15%");


#[derive(Debug, Deserialize, Clone)]
pub enum Tax {
	Nds22(Nds22),
    Usn6(Usn6),
    Usn15(Usn15),
}

impl Tax {
	pub fn multiply_i64(&self, val: i64) -> i64 {
		let val_dec = rust_decimal::Decimal::from(val);
		let rate = match self {
			Tax::Nds22(rate) => **rate,
			Tax::Usn15(rate) => **rate,
			Tax::Usn6(rate) => **rate
		};

		(val_dec * rate).round().to_i64().unwrap_or(0)
	
	}
	pub fn get_parts(&self) -> (i64, i64) {
		let t = Self::multiply_i64(self, 1000);
		let elm2 = t % 10;
		let elm1 = t / 10;
		(elm1, elm2)
	}
}

impl Serialize for Tax {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for Tax {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tax::Nds22(rate) => write!(f, "{}", rate),
            Tax::Usn6(rate) => write!(f, "{}", rate),
            Tax::Usn15(rate) => write!(f, "{}", rate),
        }
    }
}