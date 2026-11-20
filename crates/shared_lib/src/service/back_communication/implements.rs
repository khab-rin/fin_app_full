use serde::{Serialize, Deserialize};


#[derive(Clone, Serialize, Deserialize, ts_rs::TS)]
pub struct ProgressBar {
	pub label: &'static str,
	pub percent: u8
}