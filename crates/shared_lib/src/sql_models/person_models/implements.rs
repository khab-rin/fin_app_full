use axum::extract::State;
use serde::{Serialize, Deserialize};


use crate::primitives::frozen::implements::*;
use crate::primitives::frozen::implplemets_base::*;
use crate::primitives::composite::implements::Fio;
use crate::parsers::dadata::implements::AdrWrap;
use crate::parsers::mchd::implements::Gender;
use crate::Status;


#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct Person {
    pub pers_id: BoxUuid,
    pub inn: Inn,
    pub metadata: PersonMetadata,
    pub last_update: DateTime,
}


#[derive(Serialize, Deserialize, sqlx::FromRow, Debug, Clone)]
pub struct PersonDto {
    pub pers_id: BoxUuid,
    pub inn: Inn,
    pub metadata: serde_json::Value,
    pub last_update: DateTime,
}

impl std::convert::TryFrom<PersonDto> for Person {
    type Error = Status;
    fn try_from(dto: PersonDto) -> Result<Self, Self::Error> {
        Ok(Person { 
            pers_id: dto.pers_id,
            inn: dto.inn,
            metadata: serde_json::
                from_value(dto.metadata)
                .map_err(|_| Status::PersonMappingError)?,
            last_update: dto.last_update
        })
    }
}

#[derive(Serialize, Deserialize, Debug, sqlx::Type, Clone)]
#[sqlx(type_name = "jsonb")]
pub struct PersonMetadata {
    pub snils: Snils,
    pub fio: Fio,
    pub passport: Option<PassportRf>,
    pub address: Option<AdrWrap>,
    pub gender: Option<Gender>,
    pub birth_day: Option<Date>,
    pub tel_number: Option<Phone>,
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

