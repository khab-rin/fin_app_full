use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements::{*};
use crate::primitives::frozen::implements_base::*;
use crate::primitives::composite::implements::Fio;
use crate::parsers::dadata::implements::AdrWrap;
use crate::service::mchd::implements::Gender;

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
        let parsed_metadata = match dto.metadata {
            serde_json::Value::String(json_str) => {
                serde_json::from_str(&json_str)?
            },
            other_value => {
                serde_json::from_value(other_value)?
            }
        };
        Ok(Person { 
            pers_id: dto.pers_id,
            pers_inn: dto.pers_inn,
            metadata: parsed_metadata,
            last_update: dto.last_update
        })
    }
}



#[derive(Serialize, Deserialize, Debug, Clone, ts_rs::TS)]
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
    #[serde(default)]
    pub phone: std::collections::HashSet<Phone>,
    #[serde(default)]
    pub email: std::collections::HashSet<Email>,
}

impl PersonMetadata {
    pub fn merge(&mut self, new: Self) {
        if new.passport.is_some()  { self.passport = new.passport; }
        if new.address.is_some()   { self.address = new.address; }
        if new.gender.is_some()    { self.gender = new.gender; }
        if new.birth_day.is_some() { self.birth_day = new.birth_day; }
        self.phone.extend(new.phone);
        self.email.extend(new.email);
    }
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

