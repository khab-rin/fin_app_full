use serde::{Serialize, Deserialize};
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
    init_usn6_from_str,
    init_usn6_default,
    "УСН_06%");


#[derive(Debug, Serialize, Deserialize, Clone)]
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
}