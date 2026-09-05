use serde::{Serialize, Deserialize};

use crate::primitives::frozen::text_base::*;
use crate::primitives::frozen::text::*;


//СумТип
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnIpQuaterAmnts {
	#[serde(rename="@СумЗаКв")]
	first_qu: Option<u32>,

	#[serde(rename="@СумЗаПг")]
	second_qu: Option<u32>,

	#[serde(rename="@СумЗа9м")]
	third_qu: Option<u32>,

	#[serde(rename="@СумЗаНалПер")]
	fourth_qu: u32,
}

//СумСтрТип
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnIpQuterAmntInfo {
	#[serde(rename="@ФиксРазм")]
	fix_amnt: u32,

	#[serde(rename="@ДохПрев300")]
	one_perc: u32,

	#[serde(rename="@РасПерТекГод")]
	one_perc_prev_year: u32,

	#[serde(rename="@РасПерПредГод")]
	one_perc_curr_year: u32,
}
