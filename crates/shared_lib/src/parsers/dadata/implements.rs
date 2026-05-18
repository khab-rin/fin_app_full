use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements::*;
use crate::primitives::frozen::implplemets_base::String3_129;
use crate::primitives::composite::implements::RasBicAcc;
use crate::primitives::frozen::implplemets_base::CompanyName;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DadaRespWrap {
    pub suggestions: Vec<DadaElem>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DadaElem {
    #[serde(alias = "value")]
    pub short_name_dadata: Option<CompanyName>,
    #[serde(alias = "unrestricted_value")]
    pub full_name_dadata: Option<CompanyName>,
    pub data: Option<CtrprtyMetadata>,
}


#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct CtrprtyMetadata {
    pub kpp: Option<Kpp>,
    #[serde(alias = "management")]
    pub main_manager: Option<MainManager>,
    pub branch_type: Option<BranchType>,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    #[serde(alias = "state")]
    pub is_active: Option<IsCompActive>,
    pub opf: Option<OpfData>,
    #[serde(alias = "name")]
    pub comp_name: Option<DadaCompName>,
    pub inn: Option<Inn>,
    pub ogrn: Option<Ogrn>,
    pub okpo: Option<Okpo>,
    #[serde(alias = "oktmo")]
    pub oktmo_company: Option<Oktmo>,
    pub okogu: Option<Okogu>,
    pub okfs: Option<Okfs>,
    pub okved: Option<Okved>,
    #[serde(alias = "ogrn_date")]
    #[serde(skip_serializing)]
    pub ogrn_date_dadata: Option<i64>,
    pub address: Option<AdrWrap>,
    #[serde(skip_deserializing)]
    pub e_mail: Option<String3_129>,
    #[serde(skip_deserializing)]
    pub phone: Option<Phone>,
    #[serde(skip_deserializing)]
    pub bank_acc: Vec<RasBicAcc>,
    #[serde(skip_deserializing)]
    pub ogrn_date_date: Option<Date>
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct AddressData {
    #[serde(alias = "source")]
    pub address_egrul: Option<Box<str>>,
    #[serde(alias = "fias_id")]
    pub fias_id_full: Option<Box<str>>,
    pub fias_level: Option<Box<str>>,
    #[serde(alias = "oktmo")]
    pub oktmo_address: Option<Box<str>>,
    pub postal_code: Option<Box<str>>,
    #[serde(alias = "region_type_full")]
    pub lev_1_type: Option<Box<str>>,
    #[serde(alias = "region")]
    pub lev_1_name: Option<Box<str>>,
    #[serde(alias = "region_fias_id")]
    pub lev_1_fias: Option<Box<str>>,

    #[serde(alias = "area_type_full")]
    pub lev_3_type: Option<Box<str>>,
    #[serde(alias = "area")]
    pub lev_3_name: Option<Box<str>>,
    #[serde(alias = "area_fias_id")]
    pub lev_3_fias: Option<Box<str>>,

    #[serde(alias = "city_type_full")]
    pub lev_4_type: Option<Box<str>>,
    #[serde(alias = "city")]
    pub lev_4_name: Option<Box<str>>,
    #[serde(alias = "city_fias_id")]
    pub lev_4_fias: Option<Box<str>>,
    
    #[serde(alias = "city_district_type_full")]
    pub lev_5_type: Option<Box<str>>,
    #[serde(alias = "city_district")]
    pub lev_5_name: Option<Box<str>>,
    #[serde(alias = "city_district_fias_id")]
    pub lev_5_fias: Option<Box<str>>,

    #[serde(alias = "settlement_type_full")]
    pub lev_6_type: Option<Box<str>>,
    #[serde(alias = "settlement")]
    pub lev_6_name: Option<Box<str>>,
    #[serde(alias = "settlement_fias_id")]
    pub lev_6_fias: Option<Box<str>>,

    #[serde(alias = "street_type_full")]
    pub lev_8_type: Option<Box<str>>,
    #[serde(alias = "street")]
    pub lev_8_name: Option<Box<str>>,
    #[serde(alias = "street_fias_id")]
    pub lev_8_fias: Option<Box<str>>,

    #[serde(alias = "house_type_full")]
    pub lev_9_type: Option<Box<str>>,
    #[serde(alias = "house")]
    pub lev_9_name: Option<Box<str>>,
    #[serde(alias = "house_fias_id")]
    pub lev_9_fias: Option<Box<str>>,

    #[serde(alias = "block_type_full")]
    pub lev_9_1_type: Option<Box<str>>,
    #[serde(alias = "block")]
    pub lev_9_1_name: Option<Box<str>>,

    #[serde(alias = "flat_type_full")]
    pub lev_10_type: Option<Box<str>>,
    #[serde(alias = "flat")]
    pub lev_10_name: Option<Box<str>>,
    #[serde(alias = "flat_fias_id")]
    pub lev_10_fias: Option<Box<str>>,
}


#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct AdrWrap {
    #[serde(alias = "value")]
    pub short_dada_address: Option<Box<str>>,
    #[serde(alias = "unrestricted_value")]
    pub full_dada_address: Option<Box<str>>,
    #[serde(alias = "data")]
    pub address_data: Option<AddressData>
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct DadaCompName {
    #[serde(alias = "full_with_opf")]
    pub full_egrul_name: Option<CompanyName>,
    #[serde(alias = "short_with_opf")]
    pub short_egrul_name: Option<CompanyName>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct OpfData {
    #[serde(alias = "full")]
    pub full_opf: Option<Box<str>>,
    #[serde(alias = "short")]
    pub short_opf: Option<Box<str>>,
    #[serde(alias = "code")]
    pub opf_code: Option<OpfCode>
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct IsCompActive {
    pub status: Option<CompStatus>,
    pub code: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct MainManager {
    #[serde(alias = "name")]
    pub main_manager_fio: Option<Box<str>>,
    #[serde(alias = "post")]
    pub manager_title: Option<Box<str>>,
}


impl CtrprtyMetadata {
    pub fn merge_dynamic(&mut self, other: Self) {
        let mut base = serde_json::to_value(&self).unwrap_or(serde_json::Value::Object(Default::default()));
        let extra = serde_json::to_value(&other).unwrap_or(serde_json::Value::Object(Default::default()));

        if let (serde_json::Value::Object(ref mut b), serde_json::Value::Object(e)) = (&mut base, extra) {
            for (k, v) in e {
                if !v.is_null() {
                    b.insert(k, v);
                }
            }
        }
        
        if let Ok(merged) = serde_json::from_value::<Self>(base) {
            *self = merged;
        }
    }
}
