use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements::{BoxUuid, Date, CompInn, PersInn, Kpp, Ogrn, Phone, Region, RubF, Snils};
use crate::primitives::frozen::implements_base::*;
use crate::primitives::composite::implements::Fio;

// АдрТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct PostalAddress {
    #[serde(rename = "@Регион")]
    pub region: Region,

    #[serde(rename = "@ИдФИАС", skip_serializing_if = "Option::is_none")]
    pub fias_id: Option<BoxUuid>,

    #[serde(rename = "$value", skip_serializing_if = "Option::is_none")] 
    pub address: Option<AddressChoice>,
}

//ВриоНотТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct ActingNotary {
    #[serde(rename = "@Должность")]
    pub position: String1_255,

    #[serde(rename = "@РегНомНот")]
    pub exam_certificate_num: String3_8,

    #[serde(rename = "ФИОВриоНот")]
    pub fio: Fio,
}



//СвНотДействТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct Notary {
    #[serde(rename = "@Должность")]
    pub position: String1_255,

    #[serde(rename = "@РегНомНот")]
    pub notary_registry_num: String3_13,

    #[serde(rename = "ФИОНотДейств")]
    pub fio: Fio,    
}



//ДокПдтвТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct DirectAuthorityDoc  {
    #[serde(rename = "@НаимДокПдтв", skip_serializing_if = "Option::is_none")]
    pub name: Option<String1_120>,

    #[serde(rename = "@ДатаВыд", skip_serializing_if = "Option::is_none")]
    pub issue_date: Option<Date>,

    #[serde(rename = "@КемВыд", skip_serializing_if = "Option::is_none")]
    pub issued_by: Option<String1_1000>,

    #[serde(rename = "@СвУдДок", skip_serializing_if = "Option::is_none")]
    pub cert_info: Option<String1_1000>,
}



//СвИнОргТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct ForeignOrg {
    #[serde(rename = "@СтУчНД", skip_serializing_if = "Option::is_none")]
    pub principal_notarial_status: Option<PrincipalNotarialStatus>,

    #[serde(rename = "@НаимИО")]
    pub for_org_name: String1_1000,

    #[serde(rename = "@ИННЮЛ", skip_serializing_if = "Option::is_none")]
    pub inn: Option<CompInn>,
    
    #[serde(rename = "@КПП", skip_serializing_if = "Option::is_none")]
    pub kpp: Option<Kpp>,
    
    #[serde(rename = "@НЗА", skip_serializing_if = "Option::is_none")]
    pub nza: Option<String11_11>,

    #[serde(rename = "@СтрРег", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<Digits3_3>,

    #[serde(rename = "@НаимРегОрг", skip_serializing_if = "Option::is_none")]
    pub reg_org_name: Option<String1_255>,

    #[serde(rename = "@РегНомер", skip_serializing_if = "Option::is_none")]
    pub reg_number: Option<String1_80>,

    #[serde(rename = "@КодНПРег", skip_serializing_if = "Option::is_none")]
    pub foreign_tax_id: Option<String1_80>,

    #[serde(rename = "@КонтактТлф", skip_serializing_if = "Option::is_none")]
    pub tel_number: Option<Phone>,
    
    #[serde(rename = "@АдрЭлПочт", skip_serializing_if = "Option::is_none")]
    pub email: Option<String3_129>,

    #[serde(rename = "@АдрСтрРег", skip_serializing_if = "Option::is_none")]
    pub foreign_address: Option<String1_1000>,

    #[serde(rename = "АдрМНФакт", skip_serializing_if = "Option::is_none")]
    pub russian_address: Option<PostalAddress>,
}



//УдЛичнФЛ
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct PersonDocum {
    #[serde(rename = "@КодВидДок")]
    pub doc_code: RussDocumCode,

    #[serde(rename = "@СерНомДок")]
    pub doc_ser_num: String1_25,

    #[serde(rename = "@ДатаДок")]
    pub doc_date: Date,

    #[serde(rename = "@ВыдДок", skip_serializing_if = "Option::is_none")]
    pub issued_by: Option<String1_4000>,

    #[serde(rename = "@КодВыдДок", skip_serializing_if = "Option::is_none")]
    pub issued_code: Option<String7_7>,

    #[serde(rename = "expDate", skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<Date>,
}



//СведФЛТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct PersonMchd {
    #[serde(rename = "@Пол", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,

    #[serde(rename = "@ПрГражд", skip_serializing_if = "Option::is_none")]
    pub is_citizen: Option<IsCitizen>,

    #[serde(rename = "@НомЕРН", skip_serializing_if = "Option::is_none")]
    pub ern_num: Option<Digits12_12>,

    #[serde(rename = "@ДатаРожд", skip_serializing_if = "Option::is_none")]
    pub birth_day: Option<Date>,

    #[serde(rename = "@МестоРожд", skip_serializing_if = "Option::is_none")]
    pub birth_place: Option<String1_250>,

    #[serde(rename = "@Гражданство", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<Digits3_3>,

    #[serde(rename = "@КонтактТлф", skip_serializing_if = "Option::is_none")]
    pub tel_number: Option<Phone>,

    #[serde(rename = "@АдрЭлПочт", skip_serializing_if = "Option::is_none")]
    pub email: Option<String3_129>,

    #[serde(rename = "ФИО")]
    pub fio: Fio,

    #[serde(rename = "АдрМЖ", skip_serializing_if = "Option::is_none")]
    pub address: Option<PostalAddress>,

    #[serde(rename = "УдЛичнФЛ", skip_serializing_if = "Option::is_none")]
    pub person_docums: Option<PersonDocum>,
}



//СвФЛТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct WrapPerson {
    #[serde(rename = "@СтУчНД", skip_serializing_if = "Option::is_none")]
    pub principal_notarial_status: Option<PrincipalNotarialStatus>,

    #[serde(rename = "@ИННФЛ", skip_serializing_if = "Option::is_none")]
    pub inn: Option<PersInn>,

    #[serde(rename = "@СНИЛС", skip_serializing_if = "Option::is_none")]
    pub snils: Option<Snils>,

    #[serde(rename = "@Должность", skip_serializing_if = "Option::is_none")]
    pub position: Option<String1_255>,

    #[serde(rename = "ДокПдтв", skip_serializing_if = "Option::is_none")]
    pub direct_authority_doc: Option<DirectAuthorityDoc>,

    #[serde(rename = "СведФЛ")]
    pub person: PersonMchd
}



//СвОргТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct RussOrganization {
    #[serde(rename = "@СтУчНД", skip_serializing_if = "Option::is_none")]
    pub principal_notarial_status: Option<PrincipalNotarialStatus>,

    #[serde(rename = "@НаимОрг")]
    pub name: CompanyName,

    #[serde(rename = "@ИННЮЛ", skip_serializing_if = "Option::is_none")]
    pub comp_inn: Option<CompInn>,

    #[serde(rename = "@КПП")]
    pub kpp: Kpp,

    #[serde(rename = "@ОГРН", skip_serializing_if = "Option::is_none")]
    pub ogrn: Option<Ogrn>,

    #[serde(rename = "@РегНомер", skip_serializing_if = "Option::is_none")]
    pub reg_num: Option<String1_80>,

    #[serde(rename = "@НаимУчрДок", skip_serializing_if = "Option::is_none")]
    pub founding_doc : Option<String1_1000>,

    #[serde(rename = "@КонтактТлф", skip_serializing_if = "Option::is_none")]
    pub phone : Option<Phone>,

    #[serde(rename = "@АдрЭлПочт", skip_serializing_if = "Option::is_none")]
    pub email: Option<String3_129>,

    #[serde(rename = "ДокПдтв", skip_serializing_if = "Option::is_none")]
    pub direct_authority_doc: Option<DirectAuthorityDoc>,

    #[serde(rename = "АдрРег", skip_serializing_if = "Option::is_none")]
    pub address: Option<PostalAddress>
}



//ИнОргДоверТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct ForeignOrgPrincipal {
    #[serde(rename = "СвИнОрг")]
    pub foreign_org: ForeignOrg,

    #[serde(rename = "СвРукОП")]
    pub managers: Vec<WrapPerson>
}



//СведИПТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct IpPrincipal {
    #[serde(rename = "@СтУчНД", skip_serializing_if = "Option::is_none")]
    pub principal_notarial_status: Option<PrincipalNotarialStatus>,

    #[serde(rename = "@НаимИП", skip_serializing_if = "Option::is_none")]
    pub name: Option<String1_1000>,

    #[serde(rename = "@ОГРНИП")]
    pub ogrnip: Ogrn,

    #[serde(rename = "@ИННФЛ")]
    pub inn: PersInn,

    #[serde(rename = "@СНИЛС")]
    pub snils: Snils,

    #[serde(rename = "ДокПдтв", skip_serializing_if = "Option::is_none")]
    pub direct_authority_doc: Option<DirectAuthorityDoc>,

    #[serde(rename = "СведФЛ")]
    pub person: PersonMchd
}



//СВЮЛ
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct PrimeManagerOrg {
    #[serde(rename = "СвЮЛЕИО")]
    pub organization: RussOrganization,

    #[serde(rename = "СвФЛ")]
    pub managers: Vec<WrapPerson>
}



//ЛицоБезДовТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct RootManager {
    #[serde(rename = "@ПолнЮЛ")]
    pub management_type: ManagementType,

    #[serde(rename = "СВЮЛ", skip_serializing_if = "Option::is_none")]
    pub prime_manager_org: Option<PrimeManagerOrg>,

    #[serde(rename = "СвФЛ", skip_serializing_if = "Option::is_none")]
    pub prime_manager_person: Option<WrapPerson>,

    #[serde(rename = "СвИП", skip_serializing_if = "Option::is_none")]
    pub prime_manager_ip: Option<IpPrincipal>
}



//РосОргДоверТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct RussOrgPrincipal {
    #[serde(rename = "@ЕИОУК")]
    pub root_manager_yk: Flag,

    #[serde(rename = "@ЕИОФЛ")]
    pub root_manager_person: Flag,

    #[serde(rename = "@ЕИОИП")]
    pub root_manager_ip: Flag,

    #[serde(rename = "СвРосОрг")]
    pub organization: RussOrganization,

    #[serde(rename = "ЛицоБезДов")]
    pub root_managers: Vec<RootManager>
}



//СвДовТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct PoaMetadata {
    #[serde(rename = "@ВидДовер")]
    pub revocable_type: PoaTypeRevocable,

    #[serde(rename = "@ПрПередов")]
    pub redelegate_type: PoaTypeRedelegatable,

    #[serde(rename = "@ВнНомДовер")]
    pub doc_num: String1_50,

    #[serde(rename = "@НомДовер")]
    pub mchd_num: BoxUuid,

    #[serde(rename = "@НомРНДДовер", skip_serializing_if = "Option::is_none")]
    pub notar_number: Option<String1_28>,

    #[serde(rename = "@ДопИдДовер", skip_serializing_if = "Option::is_none")]
    pub extra_num: Option<String1_50>,

    #[serde(rename = "@ДатаВнРегДовер", skip_serializing_if = "Option::is_none")]
    pub registration_date: Option<Date>,

    #[serde(rename = "@ДатаВыдДовер")]
    pub issue_date: Date,

    #[serde(rename = "@СрокДейст")]
    pub life_date: Date,

    #[serde(rename = "@КодНО", skip_serializing_if = "Option::is_none")]
    pub tax_org_ident: Option<Digits4_4>,

    #[serde(rename = "КодНОДейст", default, skip_serializing_if = "Vec::is_empty")]
    pub tax_org_idents: Vec<Digits4_4>,

    #[serde(rename = "СведСист")]
    pub mchd_system_info: String1_1000,

    #[serde(rename = "Безотзыв", skip_serializing_if = "Option::is_none")]
    pub irrevocable_poa: Option<IrrevocablePoa>,
}



//Безотзыв
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct IrrevocablePoa {
    #[serde(rename = "@ПрПерБезДов")]
    pub redelegate_type: IrrevocablePoaRedelegationType,

    #[serde(rename = "@УслОтзыва")]
    pub revocation_conditin: IrrevocablePoaRevocationCondition,

    #[serde(rename = "ОписУслОт", skip_serializing_if = "Option::is_none")]
    pub revocation_condition_text: Option<String1_1000>,
}



//ПодпРукопис
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct Signature  {
    #[serde(rename = "ХешPDFДок")]
    pub pdf_hash: String1_16000,

    #[serde(rename = "ПодпИзобр")]
    pub picture: String1_16000,

    #[serde(rename = "ХешПодп")]
    pub picture_hash: String1_16000,

    #[serde(rename = "ДатаВремПодп")]
    pub sign_date: i64,

    #[serde(rename = "ФИООтв")]
    pub fio: Fio
}



//СвНотУдТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct NotarialCertification {
    #[serde(rename = "@МестоДовер")]
    pub issuance_place: String1_250,

    #[serde(rename = "@НапрДокДовЕПГУ", skip_serializing_if = "Option::is_none")]
    pub send_to_principal_epgu: Option<Flag>,

    #[serde(rename = "@НапрДокПовЕПГУ", skip_serializing_if = "Option::is_none")]
    pub send_to_representative_epgu: Option<Flag>,

    #[serde(rename = "@НапрДокЗвлФНП", skip_serializing_if = "Option::is_none")]
    pub send_to_applicant_fnp: Option<Flag>,

    #[serde(rename = "@НапрДокПовФНП", skip_serializing_if = "Option::is_none")]
    pub send_to_representative_fnp: Option<Flag>,

    #[serde(rename = "@УплНотДейст")]
    pub notary_fee: RubF,

    #[serde(rename = "@ЛьготаСум", skip_serializing_if = "Option::is_none")]
    pub discount: Option<RubF>,

    #[serde(rename = "@ДрИнфСист", skip_serializing_if = "Option::is_none")]
    pub system_info: Option<String1_255>,

    #[serde(rename = "@ДрСпосВыд", skip_serializing_if = "Option::is_none")]
    pub other_issuance_method: Option<String1_255>,

    #[serde(rename = "@ДопСвНотДовер", skip_serializing_if = "Option::is_none")]
    pub extra_info: Option<String1_2500>,

    #[serde(rename = "ИнСвУдНадпис", skip_serializing_if = "Option::is_none")]
    pub other_info: Option<String1_2500>,

    #[serde(rename = "СвНотДейств")]
    pub notary: Notary,

    #[serde(rename = "ПодпРукопис")]
    pub signature: Vec<Signature>,

    #[serde(rename = "ВриоНот", skip_serializing_if = "Option::is_none")]
    pub acting_notary : Option<ActingNotary >
}



//ФЛДоверТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct PersonPrincipal {
    #[serde(rename = "@СтУчНД")]
    pub principal_notarial_status: Option<PrincipalNotarialStatus>,

    #[serde(rename = "@ПрДеесп", skip_serializing_if = "Option::is_none")]
    pub legal_capacity: Option<Flag>,

    #[serde(rename = "@ПрНалРук", skip_serializing_if = "Option::is_none")]
    pub assistant_signatory: Option<Flag>,

    #[serde(rename = "@ДокНедеесп", skip_serializing_if = "Option::is_none")]
    pub incapacity_proof_doc: Option<String1_255>,

    #[serde(rename = "@ИННФЛ", skip_serializing_if = "Option::is_none")]
    pub inn: Option<PersInn>,

    #[serde(rename = "@СНИЛС")]
    pub snils: Snils,

    #[serde(rename = "СведФЛ")]
    pub person: PersonMchd,

    #[serde(rename = "СвЗакПредРук", skip_serializing_if = "Option::is_none")]
    pub root_manager: Option<RootManager>,

}



//Пред
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct Delegate {
    #[serde(rename = "СведОрг", skip_serializing_if = "Option::is_none")]
    pub organization: Option<RussOrganization>,

    #[serde(rename = "СведИП", skip_serializing_if = "Option::is_none")]
    pub ip: Option<IpPrincipal>,

    #[serde(rename = "СведФизЛ", skip_serializing_if = "Option::is_none")]
    pub person: Option<WrapPerson>,

    #[serde(rename = "СведФилиал", skip_serializing_if = "Option::is_none")]
    pub filial: Option<RussOrganization>,

    #[serde(rename = "СведИО", skip_serializing_if = "Option::is_none")]
    pub foreign_organization: Option<ForeignOrg>,
}



//СвУпПредТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct DelegateWrap {
    #[serde(rename = "@ТипПред")]
    pub delegate_type: DelegateType,

    #[serde(rename= "Пред")]
    pub delegate: Delegate
}



//ДоверитПерв
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct InitPrincipal {
    #[serde(rename = "РосОргДовер", skip_serializing_if = "Option::is_none")]
    pub organization: Option<RussOrganization>,

    #[serde(rename = "ИнОргДовер", skip_serializing_if = "Option::is_none")]
    pub foreign_organization: Option<ForeignOrg>,

    #[serde(rename = "ИПДовер", skip_serializing_if = "Option::is_none")]
    pub ip: Option<IpPrincipal>,

    #[serde(rename = "ФЛДовер", skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonPrincipal>
}



//СвДоверПерв
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct InitPrincipalWrap {
    #[serde(rename = "@ТипДоверит")]
    pub initial_principal_type: PrincipalIdentity,

    #[serde(rename = "ДоверитПерв")]
    pub init_principal: InitPrincipal
}



//СвПервДоверТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct OriginalPoa {
    #[serde(rename = "@ПрДовер")]
    pub origin_principal_type: InitPrincipalTypeShort,

    #[serde(rename = "@ФормДовер")]
    pub poa_legal_form: PoaLegalForm,

    #[serde(rename = "@ДатаВыдДовер")]
    pub issue_date: Date,

    #[serde(rename = "@СрокДейст")]
    pub expiration_date: Date,

    #[serde(rename = "@ВнНомДоверПерв", skip_serializing_if = "Option::is_none")]
    pub origin_poa_num: Option<String1_50>,

    #[serde(rename = "@ВнНомДоверN", skip_serializing_if = "Option::is_none")]
    pub revoc_poa_num: Option<String1_50>,

    #[serde(rename = "@НомДоверПерв", skip_serializing_if = "Option::is_none")]
    pub origin_poa_mchd_num: Option<BoxUuid>,

    #[serde(rename = "@НомДоверN", skip_serializing_if = "Option::is_none")]
    pub revoc_poa_mchd_num: Option<BoxUuid>,

    #[serde(rename = "@НомРНДПерв", skip_serializing_if = "Option::is_none")]
    pub origin_poa_notar_num: Option<String1_28>,

    #[serde(rename = "@НомРНДN", skip_serializing_if = "Option::is_none")]
    pub revoc_poa_notar_num: Option<String1_28>,

    #[serde(rename = "СвДоверПерв", default, skip_serializing_if = "Vec::is_empty")]
    pub origin_principal: Vec<InitPrincipalWrap>,

    #[serde(rename = "СвНотДейств", skip_serializing_if = "Option::is_none")]
    pub notary: Option<Notary>,

    #[serde(rename = "ВриоНот", skip_serializing_if = "Option::is_none")]
    pub acting_notary : Option<ActingNotary >
}



//ОгрСвПолн
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, ts_rs::TS)]
pub struct PoaLimitations {
    #[serde(rename = "@ИдОгр")]
    pub number: i32,

    #[serde(rename = "@КодОгр")]
    pub code: String1_255,

    #[serde(rename = "@НаимОгр")]
    pub name: String1_255,

    #[serde(rename = "@НаимЗначОгр", skip_serializing_if = "Option::is_none")]
    pub value_name: Option<String1_255>,

    #[serde(rename = "$value")]
    pub value: ConfineValue,
}



//МашПолн
#[derive(Debug, Serialize, Deserialize, Eq, Hash, PartialEq, Clone, ts_rs::TS)]
pub struct MchdPower {
    #[serde(rename = "@МнПолн", skip_serializing_if = "Option::is_none")]
    pub powers_mnemonic: Option<String6_255>,

    #[serde(rename = "@КодПолн")]
    pub powers_code: String6_255,

    #[serde(rename = "@НаимПолн")]
    pub powers_name: String1_255,

    #[serde(rename = "ОгрСвПолн", default, skip_serializing_if = "Vec::is_empty")]
    pub poa_limitations: Vec<PoaLimitations>,
}



//СвПолнТип
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct DelegatePowers {
    #[serde(rename = "@ТипПолн")]
    pub power_type: PowerType,

    #[serde(rename = "@ПрСовмПолн")]
    pub power_common_type: PowerCommonType,

    #[serde(rename = "@ПрУтрПолн", skip_serializing_if = "Option::is_none")]
    pub redelegate_power_loss: Option<RedelegatePowerLossType>,

    #[serde(rename = "ТекстПолн", skip_serializing_if = "Option::is_none")]
    pub power_text: Option<String1_10000>,

    #[serde(rename = "МашПолн", default, skip_serializing_if = "Vec::is_empty")]
    pub mchd_powers: Vec<MchdPower>
}



//Довер
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct RootPoa {
    #[serde(rename = "СвДов")]
    pub poa_metadata: PoaMetadata,

    #[serde(rename = "СвДоверит")]
    pub principal: Vec<PrincipalWrap>,

    #[serde(rename = "СвУпПред")]
    pub delegate: Vec<DelegateWrap>,

    #[serde(rename = "СвПолн")]
    pub delegate_powers: DelegatePowers,

    #[serde(rename = "СвНотУд", skip_serializing_if = "Option::is_none")]
    pub notarial_certification: Option<NotarialCertification>,
}



//Доверит
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct Principal {
    #[serde(rename = "РосОргДовер", skip_serializing_if = "Option::is_none")]
    pub russian_org: Option<RussOrgPrincipal>,

    #[serde(rename = "ИнОргДовер", skip_serializing_if = "Option::is_none")]
    pub foreign_org: Option<ForeignOrgPrincipal>,

    #[serde(rename = "ИПДовер", skip_serializing_if = "Option::is_none")]
    pub ip: Option<IpPrincipal>,

    #[serde(rename = "ФЛДовер", skip_serializing_if = "Option::is_none")]
    pub person: Option<PersonPrincipal>,
}



//СвДоверит
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct PrincipalWrap {
    #[serde(rename = "@ТипДоверит")]
    pub principal_identity: PrincipalIdentity,

    #[serde(rename = "Доверит")]
    pub principal: Principal,
}


//Передов
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct DerivedPoa {
    #[serde(rename = "@ПрНалПередов")]
    pub redelegate_flag: Flag,

    #[serde(rename = "СвПервДовер")]
    pub original_poa: OriginalPoa,

    #[serde(rename = "СвПередов", skip_serializing_if = "Option::is_none")]
    pub prev_poa: Option<OriginalPoa>,

    #[serde(rename = "СвПереДовер")]
    pub poa_metadata: PoaMetadata,

    #[serde(rename = "СвПередПолн")]
    pub sub_principals: Vec<SubPrincipal>,

    #[serde(rename = "СвПолучПолн")]
    pub delegates: Vec<DelegateWrap>,

    #[serde(rename = "СвПолн")]
    pub delegate_powers: DelegatePowers,

    #[serde(rename = "СвНотУд", skip_serializing_if = "Option::is_none")]
    pub notary_sertification: Option<NotarialCertification>,
}

//ФЛПерПолн
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct RedelegatePerson {
    #[serde(rename = "@СтУчНД", skip_serializing_if = "Option::is_none")]
    pub principal_notarial_status: Option<PrincipalNotarialStatus>,

    #[serde(rename = "@ПрНалРук", skip_serializing_if = "Option::is_none")]
    pub signatory_flag: Option<Flag>,

    #[serde(rename = "@ИННФЛ")]
    pub inn: PersInn,

    #[serde(rename = "@СНИЛС")]
    pub snils: Snils,

    #[serde(rename = "СведРукоп", skip_serializing_if = "Option::is_none")]
    pub signatory: Option<WrapPerson>,

    #[serde(rename = "СведФЛ")]
    pub person: WrapPerson,
}


//ФилПерПолн
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct RedelegaterFilial {
    #[serde(rename = "СвФил")]
    pub filial: RussOrganization,

    #[serde(rename = "СвРукФил")]
    pub manager: WrapPerson,
}


// ПередПолн
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct SubPrincipalInfo {
    #[serde(rename = "РосОргПерПолн", skip_serializing_if = "Option::is_none")]
    pub russian_organization: Option<RussOrgPrincipal>,

    #[serde(rename = "ИППерПолн", skip_serializing_if = "Option::is_none")]
    pub ip: Option<IpPrincipal>,

    #[serde(rename = "ФЛПерПолн", skip_serializing_if = "Option::is_none")]
    pub person: Option<RedelegatePerson>,

    #[serde(rename = "ФилПерПолн", skip_serializing_if = "Option::is_none")]
    pub filial: Option<RedelegaterFilial>,

    #[serde(rename = "ИнПерПолн", skip_serializing_if = "Option::is_none")]
    pub foreign_organizetioan: Option<ForeignOrgPrincipal>
}

//СвПередПолн
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct SubPrincipal {
    #[serde(rename = "@ТипПерПолн")]
    pub sub_principal_type: DelegateType,

    #[serde(rename = "ПередПолн")]
    pub sub_principal: SubPrincipalInfo
}


//Документ
#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub struct PoaWrap {
    #[serde(rename = "@КНД", skip_serializing_if = "Option::is_none")]
    pub code_knd: Option<Digits7_7>,

    #[serde(rename = "$value")]
    pub poa_doc: PoaRootKind,
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub enum PoaRootKind {
    #[serde(rename = "Довер")]
    RootPoa(Box<RootPoa>),

    #[serde(rename = "Передов")]
    DerivedPoa(Box<DerivedPoa>), 
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, ts_rs::TS)]
pub enum ConfineValue {
    #[serde(rename = "КодЗначОгр")]
    Code(String1_255),
    
    #[serde(rename = "ТексЗначОгр")]
    Text(String1_255),
}

#[derive(Debug, Serialize, Deserialize, Clone, ts_rs::TS)]
pub enum AddressChoice {
    #[serde(rename = "АдрРФ")]
    AdrRf(String1_1000),
    
    #[serde(rename = "ФИАСАдрРФ")]
    FiasAdrRf(String1_1000),
}

make_mchd_enum!(Gender, {
    Male => "1",
    Female => "2",
});

make_mchd_enum!(IsCitizen, {
    Citizen => "1",
    ForeignCitizen => "2",
    StatelessPerson => "3",
});

make_mchd_enum!(ManagementType, {
    Sole => "1",
    Joint => "2",
});

make_mchd_enum!(Flag, {
    FalseFlag => "0",
    TrueFlag => "1",
});

make_mchd_enum!(PoaTypeRevocable, {
    Revocable => "1",
    Irrevocable => "2",
});


make_mchd_enum!(PoaTypeRedelegatable, {
    Single => "1",
    Once => "2",
    Successive => "3",
});

make_mchd_enum!(IrrevocablePoaRedelegationType, {
    Static => "1",
    Single => "2",
});

make_mchd_enum!(IrrevocablePoaRevocationCondition, {
    Conditional => "1",
    Unconditional => "2",
});

make_mchd_enum!(DelegateType, {
    LegalEntity => "1",
    IndividualEnt => "2",
    PhysicalPerson => "3",
    Branch => "4",
    ForeignBranch => "5",
});

make_mchd_enum!(PrincipalIdentity, {
    RussianLegalEntity => "1",
    ForeignLegalEntity => "2",
    IndividualEntrepreneur => "3",
    PhysicalPerson => "4",
});

make_mchd_enum!(InitPrincipalTypeShort, {
    LegalEntity => "1",
    PhysicalPerson => "2",
});

make_mchd_enum!(PoaLegalForm, {
    Simple => "1",
    Notarial => "2",
});

make_mchd_enum!(PowerType, {
    Textual => "0",
    MachineReadable => "1",
});

make_mchd_enum!(PowerCommonType, {
    Individual => "1",
    Joint => "2",
});

make_mchd_enum!(RedelegatePowerLossType, {
    Lost => "1",
    Retained => "2",
});

make_mchd_enum!(RedelegationStatus, {
    Original => "0",
    Redelegated => "1",
});

make_mchd_enum!(FormatVersion, {
    Emchd1 => "EMCHD_1",
});

make_mchd_enum!(PrincipalNotarialStatus, {
    RussionPerson => "101",
    ForeignPerson => "102",
    Ip => "299",
    RussianOrganization => "301",
    ForeignOrganization => "303",
    OtherForeignOrganizations => "399"
});


make_mchd_enum!(RussDocumCode, {
    PasspRf => "21",
    BirthCert => "03",
    ForeignPassp => "10",
    MilitaryId => "07",
    ResidencyPermit => "12",
    InterRfPassport => "22",
    DriverLicense => "91",
    Snils => "14"
});

make_mchd_enum!(PoaReqElemsFlag, {
    Main => "00000000",
    MainEsia => "01000000",
    MainNotarial => "00100000",
    MainTax => "00010000",
    MainEsiaNotarial => "01100000",
    MainNotarialTax => "00110000",
    MainEsiaNotarialTax => "01110000"
});

