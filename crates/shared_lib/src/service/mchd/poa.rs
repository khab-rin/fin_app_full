use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements_base::{String1_255, String1_5000};
use crate::service::mchd::implements::{FormatVersion, PoaWrap, PoaReqElemsFlag};

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
#[serde(rename = "Доверенность")]
pub struct PoaMchd {
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,

    #[serde(rename = "@xmlns:xsd")]
    pub xmlns_xsd: String,

    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    #[serde(rename = "@ВерсФорм")]
    pub version_format: FormatVersion,

    #[serde(rename = "@ПрЭлФорм")]
    pub required_elements: PoaReqElemsFlag,

    #[serde(rename = "@ИдФайл")]
    pub flie_identificator: String1_255,

    #[serde(rename = "@ИдФайлНО")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_file_identificator: Option<String1_255>,

    #[serde(rename = "@OID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<String1_255>,

    #[serde(rename = "ИнСвед")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_info: Option<String1_5000>,

    #[serde(rename = "Документ")]
    pub poa: PoaWrap

}