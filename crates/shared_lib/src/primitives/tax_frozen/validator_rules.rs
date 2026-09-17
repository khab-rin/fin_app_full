use rust_decimal::Decimal;

use crate::Status;
use crate::static_data::primitives_re::{get_scan_nds_rate_reg, get_nds_22_default};

pub(crate) fn init_nds22_from_str(nds: &str) -> Result<Decimal, Status> {
    get_scan_nds_rate_reg()
        .captures(nds.trim())
        .and_then(|cap| cap.get(1))
        .filter(|m| m.as_str() == "22")
        .map(|_| *get_nds_22_default())
        .ok_or(Status::ValidNds)
}

pub(crate) fn init_nds_22_default() -> Decimal {
    Decimal::from(22) / Decimal::from(122)
}


pub(crate) fn init_usn6_from_str(tax: &str) -> Result<Decimal, Status> {
    let tax = tax.trim();
	if tax == "0.06" || tax == "06" || tax == "6" {
		return Ok(Decimal::from(6) / Decimal::from(100));
	} else {
		return Err(Status::ValidUsn6);
	}
}

pub(crate) fn init_usn6_default() -> Decimal {
	Decimal::from(6) / Decimal::from(100)
}

pub(crate) fn init_usn15_from_str(tax: &str) -> Result<Decimal, Status> {
    let tax = tax.trim();
	if tax == "0.15" || tax == "15" {
		return Ok(Decimal::from(15) / Decimal::from(100));
	} else {
		return Err(Status::ValidUsn6);
	}
}

pub(crate) fn init_usn15_default() -> Decimal {
	Decimal::from(15) / Decimal::from(100)
}