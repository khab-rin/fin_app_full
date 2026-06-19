use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements::{*};
use crate::primitives::frozen::implements_base::*;
use crate::primitives::composite::implements::Fio;
use crate::parsers::dadata::implements::AdrWrap;
use crate::parsers::mchd::implements::Gender;

#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone, ts_rs::TS)]
pub struct Person {
    pub pers_id: BoxUuid,
    pub pers_inn: PersInn,
    pub metadata: PersonMetadata,
    pub last_update: DateTime,
}


#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct PersonDto {
    pub pers_id: BoxUuid,
    pub pers_inn: PersInn,
    pub metadata: serde_json::Value,
    pub last_update: DateTime,
}

impl std::convert::TryFrom<PersonDto> for Person {
    type Error = serde_json::Error;
    fn try_from(dto: PersonDto) -> Result<Self, Self::Error> {
        Ok(Person { 
            pers_id: dto.pers_id,
            pers_inn: dto.pers_inn,
            metadata: serde_json::from_value(dto.metadata)?,
            last_update: dto.last_update
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type, ts_rs::TS)]
#[sqlx(type_name = "jsonb")]
pub struct PersonMetadata {
    pub snils: Snils,
    pub fio: Fio,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport: Option<PassportRf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AdrWrap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<Date>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<Phone>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
}


make_document!(PassportRf, PasspRfCode, PasspRfNumber);
make_document!(SnilsCert, SnilsCode, Snils);



make_doc_type!(PasspRfCode, PasspRf, "21");
make_doc_type!(BirthCertCode, BirthCert, "03");
make_doc_type!(ForeignPasspCode, ForeignPassp, "10");
make_doc_type!(MilitaryIdCode, MilitaryId, "07");
make_doc_type!(ResidencyPermitCode, ResidencyPermit, "12");
make_doc_type!(InterPasspRfCode, InterRfPassport, "22");
make_doc_type!(DriverLicenseCode, DriverLicense, "91");
make_doc_type!(SnilsCode, Snils, "14");

