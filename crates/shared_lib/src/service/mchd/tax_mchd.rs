use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements_base::{String1_255, String6_255};
use crate::parsers::mchd::implements::MchdPower;
use crate::static_data::mchd_const::*;


#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum MchdTaxFields {
    #[serde(rename = "2_FNS02")]
    FNS02,

    #[serde(rename = "2_FNS03")]
    FNS03,

    #[serde(rename = "2_FNS05")]
    FNS05,

    #[serde(rename = "2_FNS06")]
    FNS06,

    #[serde(rename = "2_FNS07")]
    FNS07,

    #[serde(rename = "2_FNS08")]
    FNS08,

    #[serde(rename = "2_FNS09")]
    FNS09,

    #[serde(rename = "2_FNS10")]
    FNS10,

    #[serde(rename = "2_FNS11")]
    FNS11,

    #[serde(rename = "2_FNS17")]
    FNS17,

    #[serde(rename = "2_FNS18")]
    FNS18,

    #[serde(rename = "2_FNS19")]
    FNS19,

    #[serde(rename = "2_FNS40")]
    FNS40,

    #[serde(rename = "2_FNS24")]
    FNS24,

    #[serde(rename = "2_FNS25")]
    FNS25,

    #[serde(rename = "2_FNS28")]
    FNS28,

    #[serde(rename = "2_FNS29")]
    FNS29,

    #[serde(rename = "2_FNS30")]
    FNS30,

    #[serde(rename = "2_FNS31")]
    FNS31,

    #[serde(rename = "2_FNS32")]
    FNS32,

    #[serde(rename = "2_FNS33")]
    FNS33,

    #[serde(rename = "2_FNS35")]
    FNS35,

    #[serde(rename = "2_FNS36")]
    FNS36,

    #[serde(rename = "2_FNS20")]
    FNS20,

    #[serde(rename = "2_FNS21")]
    FNS21,

    #[serde(rename = "2_FNS22")]
    FNS22,

    #[serde(rename = "2_FNS23")]
    FNS23,

    #[serde(rename = "2_FNS39")]
    FNS39,

    #[serde(rename = "2_FNS37")]
    FNS37,

    #[serde(rename = "2_FNS38")]
    FNS38,

    #[serde(rename = "2_FNS99")]
    FNS99,

    #[serde(rename = "2_FNS41")]
    FNS41,

    FNS_RAM_AU_RAU,

    FNS_RAM_SROAU_RAU,

    FNS_OFR_340FZ,

    FNS_OFR_173FZ,

    FNS_VPD_001,

    FNS_VPD_002,

    FNS_VPD_003,
}

impl MchdTaxFields {
    pub fn get_power_info(&self) -> &'static MchdPowerInfo {
        match self {
            Self::FNS02 { .. } => &FNS02,
            Self::FNS03 { .. } => &FNS03,
            Self::FNS05 { .. } => &FNS05,
            Self::FNS06 { .. } => &FNS06,
            Self::FNS07 { .. } => &FNS07,
            Self::FNS08 { .. } => &FNS08,
            Self::FNS09 { .. } => &FNS09,
            Self::FNS10 { .. } => &FNS10,
            Self::FNS11 { .. } => &FNS11,
            Self::FNS17 { .. } => &FNS17,
            Self::FNS18 { .. } => &FNS18,
            Self::FNS19 { .. } => &FNS19,
            Self::FNS20 { .. } => &FNS20,
            Self::FNS21 { .. } => &FNS21,
            Self::FNS22 { .. } => &FNS22,
            Self::FNS23 { .. } => &FNS23,
            Self::FNS24 { .. } => &FNS24,
            Self::FNS25 { .. } => &FNS25,
            Self::FNS28 { .. } => &FNS28,
            Self::FNS29 { .. } => &FNS29,
            Self::FNS30 { .. } => &FNS30,
            Self::FNS31 { .. } => &FNS31,
            Self::FNS32 { .. } => &FNS32,
            Self::FNS33 { .. } => &FNS33,
            Self::FNS35 { .. } => &FNS35,
            Self::FNS36 { .. } => &FNS36,
            Self::FNS37 { .. } => &FNS37,
            Self::FNS38 { .. } => &FNS38,
            Self::FNS39 { .. } => &FNS39,
            Self::FNS40 { .. } => &FNS40,
            Self::FNS41 { .. } => &FNS41,
            Self::FNS99 { .. } => &FNS99,
            Self::FNS_RAM_AU_RAU { .. } => &FNS_RAM_AU_RAU,
            Self::FNS_RAM_SROAU_RAU { .. } => &FNS_RAM_SROAU_RAU,
            Self::FNS_OFR_340FZ { .. } => &FNS_OFR_340FZ,
            Self::FNS_OFR_173FZ { .. } => &FNS_OFR_173FZ,
            Self::FNS_VPD_001 { .. } => &FNS_VPD_001,
            Self::FNS_VPD_002 { .. } => &FNS_VPD_002,
            Self::FNS_VPD_003 { .. } => &FNS_VPD_003,
        }
    }

    pub fn make_mchd_power(self) -> MchdPower {
        let data = self.get_power_info();
        let code = String6_255::unchecked(data.code);
        let name = String1_255::unchecked(data.name);
        MchdPower { powers_mnemonic: None, powers_code: code, powers_name: name, poa_limitations: vec!() }
    }

    pub fn get_all_powers() -> Vec<MchdTaxFields> {
        vec![
            MchdTaxFields::FNS02, 
            MchdTaxFields::FNS03, 
            MchdTaxFields::FNS05, 
            MchdTaxFields::FNS06, 
            MchdTaxFields::FNS07, 
            MchdTaxFields::FNS08, 
            MchdTaxFields::FNS09, 
            MchdTaxFields::FNS10, 
            MchdTaxFields::FNS11, 
            MchdTaxFields::FNS17, 
            MchdTaxFields::FNS18, 
            MchdTaxFields::FNS19, 
            MchdTaxFields::FNS40, 
            MchdTaxFields::FNS24, 
            MchdTaxFields::FNS25, 
            MchdTaxFields::FNS28, 
            MchdTaxFields::FNS29, 
            MchdTaxFields::FNS30, 
            MchdTaxFields::FNS31, 
            MchdTaxFields::FNS32, 
            MchdTaxFields::FNS33, 
            MchdTaxFields::FNS35, 
            MchdTaxFields::FNS36, 
            MchdTaxFields::FNS20, 
            MchdTaxFields::FNS21, 
            MchdTaxFields::FNS22, 
            MchdTaxFields::FNS23, 
            MchdTaxFields::FNS39, 
            MchdTaxFields::FNS37, 
            MchdTaxFields::FNS38, 
            MchdTaxFields::FNS99, 
            MchdTaxFields::FNS41, 
            MchdTaxFields::FNS_RAM_AU_RAU, 
            MchdTaxFields::FNS_RAM_SROAU_RAU, 
            MchdTaxFields::FNS_OFR_340FZ, 
            MchdTaxFields::FNS_OFR_173FZ, 
            MchdTaxFields::FNS_VPD_001, 
            MchdTaxFields::FNS_VPD_002, 
            MchdTaxFields::FNS_VPD_003]
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
pub struct MchdPowerInfo {
    pub code: &'static str,
    pub name: &'static str,
}