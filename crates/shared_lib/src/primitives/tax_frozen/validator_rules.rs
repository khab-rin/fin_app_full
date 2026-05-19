use rust_decimal::Decimal;

use crate::Status;
use crate::static_data::primitives_re::{get_scan_nds_rate_reg, get_nds_22_default};

pub(crate) fn init_nds22_from_str(nds: &str) -> Result<Decimal, Status> {
    get_scan_nds_rate_reg()
        .captures(nds.trim())
        .and_then(|cap| cap.get(1))
        .filter(|m| m.as_str() == "22")
        .map(|_| *get_nds_22_default())
        .ok_or(Status::ValidWrongNdsValue)
}

pub(crate) fn init_nds_22_default() -> &'static Decimal {
    get_nds_22_default()
}