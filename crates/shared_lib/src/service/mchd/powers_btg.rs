use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
pub struct BTGPowerInfo {
    pub code: &'static str,
    pub name: &'static str,
}

#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
pub struct BTGPowerLimitType {
    pub code: &'static str,
    pub name: &'static str,
}

#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
pub struct BTGDocLimit {
    pub code: &'static str,
    pub name: &'static str,
}