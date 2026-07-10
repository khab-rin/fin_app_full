use std::collections::HashSet;

use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements_base::{String1_255, String1_10000, String6_255};
use crate::service::mchd::implements::MchdPower;
use crate::static_data::mchd_powers::powers_fns::*;
use crate::static_data::mchd_powers::powers_btb::*;
use crate::static_data::mchd_powers::powers_home::*;


#[derive(Serialize, Deserialize, Debug, Eq, Hash, PartialEq, Clone, ts_rs::TS)]
pub enum HomeMchdPower {
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
    #[serde(rename = "FNS_RAM_AU_RAU")]
    FNSRAMAURAU,
    #[serde(rename = "FNS_RAM_SROAU_RAU")]
    FNSRAMSROAURAU,
    #[serde(rename = "FNS_OFR_340FZ")]
    FNSOFR340FZ,
    #[serde(rename = "FNS_OFR_173FZ")]
    FNSOFR173FZ,
    #[serde(rename = "FNS_VPD_001")]
    FNSVPD001,
    #[serde(rename = "FNS_VPD_002")]
    FNSVPD002,
    #[serde(rename = "FNS_VPD_003")]
    FNSVPD003,

    #[serde(rename = "BTBO_00000001")]
    BTBO1,
    #[serde(rename = "BTBO_00000002")]
    BTBO2,
    #[serde(rename = "BTBO_00000003")]
    BTBO3,
    #[serde(rename = "BTBO_00000004")]
    BTBO4,
    #[serde(rename = "BTBO_00000005")]
    BTBO5,
    #[serde(rename = "BTBO_00000006")]
    BTBO6,
    #[serde(rename = "BTBO_00000007")]
    BTBO7,
    #[serde(rename = "BTBO_00000008")]
    BTBO8,
    #[serde(rename = "BTBO_00000009")]
    BTBO9,
    #[serde(rename = "BTBO_00000010")]
    BTBO10,
    #[serde(rename = "BTBO_00000011")]
    BTBO11,
    #[serde(rename = "BTBO_00000012")]
    BTBO12,
    #[serde(rename = "BTBO_00000013")]
    BTBO13,
    #[serde(rename = "BTBO_00000014")]
    BTBO14,
    #[serde(rename = "BTBO_00000015")]
    BTBO15,
    #[serde(rename = "BTBO_00000016")]
    BTBO16,
    #[serde(rename = "BTBO_00000017")]
    BTBO17,

    H110,
    H111,
    H210,
    H211,
    H310,
    H311,
    H410,
    H411,
    H510,
    H511,
    H610,
    H611,
    H710,
    H711,
    H810,
    H811,
    H910,
    H911,
    H1010,
    H1011,

    Unknown,
}

impl HomeMchdPower {
    pub fn get_power_info(&self) -> &'static HomePowerInfo {
        match self {
            Self::FNS02 => &FNS02,
            Self::FNS03 => &FNS03,
            Self::FNS05 => &FNS05,
            Self::FNS06 => &FNS06,
            Self::FNS07 => &FNS07,
            Self::FNS08 => &FNS08,
            Self::FNS09 => &FNS09,
            Self::FNS10 => &FNS10,
            Self::FNS11 => &FNS11,
            Self::FNS17 => &FNS17,
            Self::FNS18 => &FNS18,
            Self::FNS19 => &FNS19,
            Self::FNS20 => &FNS20,
            Self::FNS21 => &FNS21,
            Self::FNS22 => &FNS22,
            Self::FNS23 => &FNS23,
            Self::FNS24 => &FNS24,
            Self::FNS25 => &FNS25,
            Self::FNS28 => &FNS28,
            Self::FNS29 => &FNS29,
            Self::FNS30 => &FNS30,
            Self::FNS31 => &FNS31,
            Self::FNS32 => &FNS32,
            Self::FNS33 => &FNS33,
            Self::FNS35 => &FNS35,
            Self::FNS36 => &FNS36,
            Self::FNS37 => &FNS37,
            Self::FNS38 => &FNS38,
            Self::FNS39 => &FNS39,
            Self::FNS40 => &FNS40,
            Self::FNS41 => &FNS41,
            Self::FNS99 => &FNS99,
            Self::FNSRAMAURAU => &FNSRAMAURAU,
            Self::FNSRAMSROAURAU => &FNSRAMSROAURAU,
            Self::FNSOFR340FZ => &FNSOFR340FZ,
            Self::FNSOFR173FZ => &FNSOFR173FZ,
            Self::FNSVPD001 => &FNSVPD001,
            Self::FNSVPD002 => &FNSVPD002,
            Self::FNSVPD003 => &FNSVPD003,

            Self::BTBO1 => &BTBO1,
            Self::BTBO2 => &BTBO2,
            Self::BTBO3 => &BTBO3,
            Self::BTBO4 => &BTBO4,
            Self::BTBO5 => &BTBO5,
            Self::BTBO6 => &BTBO6,
            Self::BTBO7 => &BTBO7,
            Self::BTBO8 => &BTBO8,
            Self::BTBO9 => &BTBO9,
            Self::BTBO10 => &BTBO10,
            Self::BTBO11 => &BTBO11,
            Self::BTBO12 => &BTBO12,
            Self::BTBO13 => &BTBO13,
            Self::BTBO14 => &BTBO14,
            Self::BTBO15 => &BTBO15,
            Self::BTBO16 => &BTBO16,
            Self::BTBO17 => &BTBO17,


            Self::H110 => &H110,
            Self::H111 => &H111,
            Self::H210 => &H210,
            Self::H211 => &H211,
            Self::H310 => &H310,
            Self::H311 => &H311,
            Self::H410 => &H410,
            Self::H411 => &H411,
            Self::H510 => &H510,
            Self::H511 => &H511,
            Self::H610 => &H610,
            Self::H611 => &H611,
            Self::H710 => &H710,
            Self::H711 => &H711,
            Self::H810 => &H810,
            Self::H811 => &H811,
            Self::H910 => &H910,
            Self::H911 => &H911,
            Self::H1010 => &H1010,
            Self::H1011 => &H1011,

            Self::Unknown => &UNKNOWN

        }
    }

    pub fn make_mchd_power(self) -> MchdPower {
        let data = self.get_power_info();
        let code = String6_255::unchecked(data.code);
        let name = String1_255::unchecked(data.name);
        MchdPower { powers_mnemonic: None, powers_code: code, powers_name: name, poa_limitations: vec!() }
    }

    pub fn make_text_power(powers: &HashSet<HomeMchdPower>) -> String1_10000 {
        let mut powers_str: Vec<String> = vec!();
        for power in powers {
            let data = power.get_power_info();
            powers_str.push(data.name.to_string());
        }

        String1_10000::unchecked(powers_str.join(", "))
    }

    pub fn parse_text_power(power_text: &str) -> Self {
        let t = power_text.trim();

        if t == H110.name { Self::H110 }
        else if t == H111.name { Self::H111 }
        else if t == H210.name { Self::H210 }
        else if t == H211.name { Self::H211 }
        else if t == H310.name { Self::H310 }
        else if t == H311.name { Self::H311 }
        else if t == H410.name { Self::H410 }
        else if t == H411.name { Self::H411 }
        else if t == H510.name { Self::H510 }
        else if t == H511.name { Self::H511 }
        else if t == H610.name { Self::H610 }
        else if t == H611.name { Self::H611 }
        else if t == H710.name { Self::H710 }
        else if t == H711.name { Self::H711 }
        else if t == H810.name { Self::H810 }
        else if t == H811.name { Self::H811 }
        else if t == H910.name { Self::H910 }
        else if t == H911.name { Self::H911 }
        else if t == H1010.name { Self::H1010 }
        else if t == H1011.name { Self::H1011 }
        else { Self::Unknown }
    }

    pub fn get_all_tax_powers() -> Vec<HomeMchdPower> {
        vec![
            HomeMchdPower::FNS02, 
            HomeMchdPower::FNS03, 
            HomeMchdPower::FNS05, 
            HomeMchdPower::FNS06, 
            HomeMchdPower::FNS07, 
            HomeMchdPower::FNS08, 
            HomeMchdPower::FNS09, 
            HomeMchdPower::FNS10, 
            HomeMchdPower::FNS11, 
            HomeMchdPower::FNS17, 
            HomeMchdPower::FNS18, 
            HomeMchdPower::FNS19, 
            HomeMchdPower::FNS40, 
            HomeMchdPower::FNS24, 
            HomeMchdPower::FNS25, 
            HomeMchdPower::FNS28, 
            HomeMchdPower::FNS29, 
            HomeMchdPower::FNS30, 
            HomeMchdPower::FNS31, 
            HomeMchdPower::FNS32, 
            HomeMchdPower::FNS33, 
            HomeMchdPower::FNS35, 
            HomeMchdPower::FNS36, 
            HomeMchdPower::FNS20, 
            HomeMchdPower::FNS21, 
            HomeMchdPower::FNS22, 
            HomeMchdPower::FNS23, 
            HomeMchdPower::FNS39, 
            HomeMchdPower::FNS37, 
            HomeMchdPower::FNS38, 
            HomeMchdPower::FNS99, 
            HomeMchdPower::FNS41, 
            HomeMchdPower::FNSRAMAURAU, 
            HomeMchdPower::FNSRAMSROAURAU, 
            HomeMchdPower::FNSOFR340FZ, 
            HomeMchdPower::FNSOFR173FZ, 
            HomeMchdPower::FNSVPD001, 
            HomeMchdPower::FNSVPD002, 
            HomeMchdPower::FNSVPD003
        ]
    }

    pub fn get_all_btb_powers() -> Vec<HomeMchdPower> {
        vec![
            HomeMchdPower::BTBO1,
            HomeMchdPower::BTBO2,
            HomeMchdPower::BTBO3,
            HomeMchdPower::BTBO4,
            HomeMchdPower::BTBO5,
            HomeMchdPower::BTBO6,
            HomeMchdPower::BTBO7,
            HomeMchdPower::BTBO8,
            HomeMchdPower::BTBO9,
            HomeMchdPower::BTBO10,
            HomeMchdPower::BTBO11,
            HomeMchdPower::BTBO12,
            HomeMchdPower::BTBO13,
            HomeMchdPower::BTBO14,
            HomeMchdPower::BTBO15,
            HomeMchdPower::BTBO16,
            HomeMchdPower::BTBO17,
        ]
    }

    pub fn get_all_home_powers() -> Vec<HomeMchdPower> {
        vec![
            HomeMchdPower::H110,
            HomeMchdPower::H111,
            HomeMchdPower::H210,
            HomeMchdPower::H211,
            HomeMchdPower::H310,
            HomeMchdPower::H311,
            HomeMchdPower::H410,
            HomeMchdPower::H411,
            HomeMchdPower::H510,
            HomeMchdPower::H511,
            HomeMchdPower::H610,
            HomeMchdPower::H611,
            HomeMchdPower::H710,
            HomeMchdPower::H711,
            HomeMchdPower::H810,
            HomeMchdPower::H811,
            HomeMchdPower::H910,
            HomeMchdPower::H911,
            HomeMchdPower::H1010,
            HomeMchdPower::H1011,
        ]
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
pub struct HomePowerInfo {
    pub code: &'static str,
    pub name: &'static str,
}