use serde::{Serialize, Deserialize};
use rust_decimal::Decimal;

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