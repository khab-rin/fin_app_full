use validator::Validate;
use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implplemets_mchd::{String1_255, String1_5000, PoaReqElemsFlag};
use crate::parsers::mchd::implements::{FormatVersion, PoaWrap};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PoaMchd {
    #[serde(rename = "@ВерсФорм")]
    pub version_format: FormatVersion,

    #[serde(rename = "@ПрЭлФорм")]
    required_elements: PoaReqElemsFlag,

    #[serde(rename = "@ИдФайл")]
    pub flie_identifivator: String1_255,

    #[serde(rename = "@ИдФайлНО")]
    pub tax_file_identificator: String1_255,

    #[serde(rename = "@OID")]
    pub oid: Option<String1_255>,

    #[serde(rename = "ИнСвед")]
    pub text_info: Option<String1_5000>,

    #[validate]
    #[serde(rename = "Документ")]
    poa: PoaWrap

}