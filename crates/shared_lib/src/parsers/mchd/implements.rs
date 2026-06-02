use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements::{BoxUuid, Date, CompInn, PersInn, Kpp, Ogrn, Phone, Region, RubF, Snils, ParticipantStatus};
use crate::primitives::frozen::implements_base::*;
use crate::primitives::composite::implements::Fio;



// АдрТип
#[derive(Debug, Serialize, Deserialize)]
pub struct PostalAddress {
    #[serde(rename = "@Регион")]
    pub region: Region,

    #[serde(rename = "@ИдФИАС")]
    pub fias_id: Option<BoxUuid>,

    #[serde(rename = "$value")] 
    pub address: Option<AddressChoice>,
}



//ВриоНотТип
#[derive(Debug, Serialize, Deserialize)]
pub struct ActingNotary {
    #[serde(rename = "@Должность")]
    pub position: String1_255,

    #[serde(rename = "@РегНомНот")]
    pub exam_certificate_num: String3_8,

    #[serde(rename = "ФИОВриоНот")]
    pub fio: Fio,
}



//СвНотДействТип
#[derive(Debug, Serialize, Deserialize)]
pub struct Notary {
    #[serde(rename = "@Должность")]
    pub position: String1_255,

    #[serde(rename = "@РегНомНот")]
    pub notary_registry_num: String3_13,

    #[serde(rename = "ФИОНотДейств")]
    pub fio: Fio,    
}



//ДокПдтвТип
#[derive(Debug, Serialize, Deserialize)]
pub struct DirectAuthorityDoc  {
    #[serde(rename = "@НаимДокПдтв")]
    pub name: Option<String1_120>,

    #[serde(rename = "@ДатаВыд")]
    pub issue_date: Option<Date>,

    #[serde(rename = "@КемВыд")]
    pub issued_by: Option<String1_1000>,

    #[serde(rename = "@СвУдДок")]
    pub cert_info: Option<String1_1000>,
}



//СвИнОргТип
#[derive(Debug, Serialize, Deserialize)]
pub struct ForeignOrg {
    #[serde(rename = "@СтУчНД")]
    pub participant_status: Option<ParticipantStatus>,

    #[serde(rename = "@НаимИО")]
    pub for_org_name: String1_1000,

    #[serde(rename = "@ИННЮЛ")]
    pub inn: Option<CompInn>,
    
    #[serde(rename = "@КПП")]
    pub kpp: Option<Kpp>,
    
    #[serde(rename = "@НЗА")]
    pub nza: Option<String11_11>,

    #[serde(rename = "@СтрРег")]
    pub country_code: Option<Digits3_3>,

    #[serde(rename = "@НаимРегОрг")]
    pub reg_org_name: Option<String1_255>,

    #[serde(rename = "@РегНомер")]
    pub reg_number: Option<String1_80>,

    #[serde(rename = "@КодНПРег")]
    pub foreign_tax_id: Option<String1_80>,

    #[serde(rename = "@КонтактТлф")]
    pub tel_number: Option<Phone>,
    
    #[serde(rename = "@АдрЭлПочт")]
    pub email: Option<String3_129>,

    #[serde(rename = "@АдрСтрРег")]
    pub foreign_address: Option<String1_1000>,

    #[serde(rename = "АдрМНФакт")]
    pub russian_address: Option<PostalAddress>,
}



//УдЛичнФЛ
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonDocums {
    #[serde(rename = "@КодВидДок")]
    pub doc_code: Digits2_2,

    #[serde(rename = "@СерНомДок")]
    pub doc_ser_num: String1_25,

    #[serde(rename = "@ДатаДок")]
    pub doc_date: Date,

    #[serde(rename = "@ВыдДок")]
    pub issued_by: Option<String1_4000>,

    #[serde(rename = "@КодВыдДок")]
    pub issued_code: Option<String7_7>,

    #[serde(rename = "expDate")]
    pub exp_date: Option<Date>,
}



//СведФЛТип
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonMchd {
    #[serde(rename = "@Пол")]
    pub gender: Option<Gender>,

    #[serde(rename = "@ПрГражд")]
    pub is_citizen: Option<IsCitizen>,

    #[serde(rename = "@НомЕРН")]
    pub ern_num: Option<Digits12_12>,

    #[serde(rename = "@ДатаРожд")]
    pub birth_day: Option<Date>,

    #[serde(rename = "@МестоРожд")]
    pub birth_place: Option<String1_250>,

    #[serde(rename = "@Гражданство")]
    pub country_code: Option<Digits3_3>,

    #[serde(rename = "@КонтактТлф")]
    pub tel_number: Option<Phone>,

    #[serde(rename = "@АдрЭлПочт")]
    pub email: Option<String3_129>,

    #[serde(rename = "ФИО")]
    pub fio: Fio,

    #[serde(rename = "АдрМЖ")]
    pub address: Option<PostalAddress>,

    #[serde(rename = "УдЛичнФЛ")]
    pub person_docums: Option<PersonDocums>,
}



//СвФЛТип
#[derive(Debug, Serialize, Deserialize)]
pub struct JuridicalWrapPerson {
    #[serde(rename = "@СтУчНД")]
    pub participant_status: Option<ParticipantStatus>,

    #[serde(rename = "@ИННФЛ")]
    pub inn: Option<PersInn>,

    #[serde(rename = "@СНИЛС")]
    pub snils: Option<Snils>,

    #[serde(rename = "@Должность")]
    pub position: Option<String1_255>,

    #[serde(rename = "ДокПдтв")]
    pub direct_authority_doc: Option<DirectAuthorityDoc>,

    #[serde(rename = "СведФЛ")]
    pub person: PersonMchd
}



//СвОргТип
#[derive(Debug, Serialize, Deserialize)]
pub struct RussOrganization {
    #[serde(rename = "@СтУчНД")]
    pub participant_statustatus: Option<ParticipantStatus>,

    #[serde(rename = "@НаимОрг")]
    pub name: String1_1000,

    #[serde(rename = "@ИННЮЛ")]
    pub inn: Option<CompInn>,

    #[serde(rename = "@КПП")]
    pub kpp: Kpp,

    #[serde(rename = "@ОГРН")]
    pub ogrn: Option<Ogrn>,

    #[serde(rename = "@РегНомер")]
    pub reg_num: Option<String1_80>,

    #[serde(rename = "@НаимУчрДок")]
    pub founding_doc : Option<String1_1000>,

    #[serde(rename = "@КонтактТлф")]
    pub phone : Option<Phone>,

    #[serde(rename = "@АдрЭлПочт")]
    pub email: Option<String3_129>,

    #[serde(rename = "ДокПдтв")]
    pub direct_authority_doc: Option<DirectAuthorityDoc>,

    #[serde(rename = "АдрРег")]
    pub address: Option<PostalAddress>
}



//ИнОргДоверТип
#[derive(Debug, Serialize, Deserialize)]
pub struct ForeignOrgPrincipal {
    #[serde(rename = "СвИнОрг")]
    pub foreign_org: ForeignOrg,

    #[serde(rename = "СвРукОП")]
    pub managers: Vec<JuridicalWrapPerson>
}



//СведИПТип
#[derive(Debug, Serialize, Deserialize)]
pub struct Ip {
    #[serde(rename = "@СтУчНД")]
    pub participant_statustatus: Option<ParticipantStatus>,

    #[serde(rename = "@НаимИП")]
    pub name: Option<String1_1000>,

    #[serde(rename = "@ОГРНИП")]
    pub ogrnip: Ogrn,

    #[serde(rename = "@ИННФЛ")]
    pub inn: PersInn,

    #[serde(rename = "@СНИЛС")]
    pub snils: Snils,

    #[serde(rename = "ДокПдтв")]
    pub direct_authority_doc: Option<DirectAuthorityDoc>,

    #[serde(rename = "СведФЛ")]
    pub person: PersonMchd
}



//СВЮЛ
#[derive(Debug, Serialize, Deserialize)]
pub struct PrimeManagerOrg {
    #[serde(rename = "СвЮЛЕИО")]
    pub organization: RussOrganization,

    #[serde(rename = "СвФЛ")]
    pub managers: Vec<JuridicalWrapPerson>
}



//ЛицоБезДовТип
#[derive(Debug, Serialize, Deserialize)]
pub struct LegalRep {
    #[serde(rename = "@ПолнЮЛ")]
    pub management_type: ManagementType,

    #[serde(rename = "СВЮЛ")]
    pub prime_manager_org: Option<PrimeManagerOrg>,

    #[serde(rename = "СвФЛ")]
    pub prime_manager_person: Option<JuridicalWrapPerson>,

    #[serde(rename = "СвИП")]
    pub prime_manager_ip: Option<Ip>
}



//РосОргДоверТип
#[derive(Debug, Serialize, Deserialize)]
pub struct RussOrgPrincipal {
    #[serde(rename = "@ЕИОУК")]
    pub direct_authority_yk: Flag,

    #[serde(rename = "@ЕИОФЛ")]
    pub direct_authority_yk_person: Flag,

    #[serde(rename = "@ЕИОИП")]
    pub direct_authority_yk_ip: Flag,

    #[serde(rename = "СвРосОрг")]
    pub organization: RussOrganization,

    #[serde(rename = "ЛицоБезДов")]
    pub managers: Vec<LegalRep>
}



//СвДовТип
#[derive(Debug, Serialize, Deserialize)]
pub struct PoaMetadata {
    #[serde(rename = "@ВидДовер")]
    pub revocable_type: PoaTypeRevocable,

    #[serde(rename = "@ПрПередов")]
    pub redelegate_type: PoaTypeRedelegatable,

    #[serde(rename = "@ВнНомДовер")]
    pub doc_num: String1_50,

    #[serde(rename = "@НомДовер")]
    pub mchd_num: BoxUuid,

    #[serde(rename = "@НомРНДДовер")]
    pub notar_number: Option<String1_28>,

    #[serde(rename = "@ДопИдДовер")]
    pub extra_num: Option<String1_50>,

    #[serde(rename = "@ДатаВнРегДовер")]
    pub registration_date: Option<Date>,

    #[serde(rename = "@ДатаВыдДовер")]
    pub issue_date: Date,

    #[serde(rename = "@СрокДейст")]
    pub life_date: Date,

    #[serde(rename = "@КодНО")]
    pub tax_org_ident: Option<Digits4_4>,

    #[serde(rename = "КодНОДейст", default, skip_serializing_if = "Vec::is_empty")]
    pub tax_org_idents: Vec<Digits4_4>,

    #[serde(rename = "СведСист")]
    pub system_info: String1_1000,

    #[serde(rename = "Безотзыв")]
    pub irrevocable_poa: Option<IrrevocablePoa>,
}



//Безотзыв
#[derive(Debug, Serialize, Deserialize)]
pub struct IrrevocablePoa {
    #[serde(rename = "@ПрПерБезДов")]
    pub redelegate_type: IrrevocablePoaRedelegationType,

    #[serde(rename = "@УслОтзыва")]
    pub revocation_conditin: IrrevocablePoaRevocationCondition,

    #[serde(rename = "ОписУслОт")]
    pub revocation_condition_text: Option<String1_1000>,
}



//ПодпРукопис
#[derive(Debug, Serialize, Deserialize)]
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
#[derive(Debug, Serialize, Deserialize)]
pub struct NotarialCertification {
    #[serde(rename = "@МестоДовер")]
    pub issuance_place: String1_250,

    #[serde(rename = "@НапрДокДовЕПГУ")]
    pub send_to_principal_epgu: Option<Flag>,

    #[serde(rename = "@НапрДокПовЕПГУ")]
    pub send_to_representative_epgu: Option<Flag>,

    #[serde(rename = "@НапрДокЗвлФНП")]
    pub send_to_applicant_fnp: Option<Flag>,

    #[serde(rename = "@НапрДокПовФНП")]
    pub send_to_representative_fnp: Option<Flag>,

    #[serde(rename = "@УплНотДейст")]
    pub notary_fee: RubF,

    #[serde(rename = "@ЛьготаСум")]
    pub discount: Option<RubF>,

    #[serde(rename = "@ДрИнфСист")]
    pub system_info: Option<String1_255>,

    #[serde(rename = "@ДрСпосВыд")]
    pub other_issuance_method: Option<String1_255>,

    #[serde(rename = "@ДопСвНотДовер")]
    pub extra_info: Option<String1_2500>,

    #[serde(rename = "ИнСвУдНадпис")]
    pub other_info: Option<String1_2500>,

    #[serde(rename = "СвНотДейств")]
    pub notary: Notary,

    #[serde(rename = "ПодпРукопис")]
    pub signature: Vec<Signature>,

    #[serde(rename = "ВриоНот")]
    pub acting_notary : Option<ActingNotary >
}



//ФЛДоверТип
#[derive(Debug, Serialize, Deserialize)]
pub struct PrincipalPerson {
    #[serde(rename = "@СтУчНД")]
    pub participant_status: Option<ParticipantStatus>,

    #[serde(rename = "@ПрДеесп")]
    pub legal_capacity: Option<Flag>,

    #[serde(rename = "@ПрНалРук")]
    pub assistant_signatory: Option<Flag>,

    #[serde(rename = "@ДокНедеесп")]
    pub incapacity_proof_doc: Option<String1_255>,

    #[serde(rename = "@ИННФЛ")]
    pub inn: Option<PersInn>,

    #[serde(rename = "@СНИЛС")]
    pub snils: Snils,

    #[serde(rename = "СведФЛ")]
    pub person: PersonMchd,

    #[serde(rename = "СвЗакПредРук")]
    pub legal_rep: Option<LegalRep>,

}



//Пред
#[derive(Debug, Serialize, Deserialize)]
pub struct Delegate {
    #[serde(rename = "СведОрг")]
    pub organization: Option<RussOrganization>,

    #[serde(rename = "СведИП")]
    pub ip: Option<Ip>,

    #[serde(rename = "СведФизЛ")]
    pub person: Option<JuridicalWrapPerson>,

    #[serde(rename = "СведФилиал")]
    pub filial: Option<RussOrganization>,

    #[serde(rename = "СведИО")]
    pub foreign_organization: Option<ForeignOrg>,
}



//СвУпПредТип
#[derive(Debug, Serialize, Deserialize)]
pub struct DelegateInfo {
    #[serde(rename = "@ТипПред")]
    pub delegate_type: DelegateType,

    #[serde(rename= "Пред")]
    pub delegate: Delegate
}



//ДоверитПерв
#[derive(Debug, Serialize, Deserialize)]
pub struct InitPrincipal {
    #[serde(rename = "РосОргДовер")]
    pub organization: Option<RussOrganization>,

    #[serde(rename = "ИнОргДовер")]
    pub foreign_organization: Option<ForeignOrg>,

    #[serde(rename = "ИПДовер")]
    pub ip: Option<Ip>,

    #[serde(rename = "ФЛДовер")]
    pub person: Option<PrincipalPerson>
}



//СвДоверПерв
#[derive(Debug, Serialize, Deserialize)]
pub struct InitPrincipalWrap {
    #[serde(rename = "@ТипДоверит")]
    pub initial_principal_type: PrincipalIdentity,

    #[serde(rename = "ДоверитПерв")]
    pub init_principal: InitPrincipal
}



//СвПервДоверТип
#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalPoa {
    #[serde(rename = "@ПрДовер")]
    pub origin_principal_type: InitPrincipalTypeShort,

    #[serde(rename = "@ФормДовер")]
    pub poa_legal_form: PoaLegalForm,

    #[serde(rename = "@ДатаВыдДовер")]
    pub issue_date: Date,

    #[serde(rename = "@СрокДейст")]
    pub expiration_date: Date,

    #[serde(rename = "@ВнНомДоверПерв")]
    pub origin_poa_num: Option<String1_50>,

    #[serde(rename = "@ВнНомДоверN")]
    pub revoc_poa_num: Option<String1_50>,

    #[serde(rename = "@НомДоверПерв")]
    pub origin_poa_mchd_num: Option<BoxUuid>,

    #[serde(rename = "@НомДоверN")]
    pub revoc_poa_mchd_num: Option<BoxUuid>,

    #[serde(rename = "@НомРНДПерв")]
    pub origin_poa_notar_num: Option<String1_28>,

    #[serde(rename = "@НомРНДN")]
    pub revoc_poa_notar_num: Option<String1_28>,

    #[serde(rename = "СвДоверПерв", default, skip_serializing_if = "Vec::is_empty")]
    pub origin_principal: Vec<InitPrincipalWrap>,

    #[serde(rename = "СвНотДейств")]
    pub notary: Option<Notary>,

    #[serde(rename = "ВриоНот")]
    pub acting_notary : Option<ActingNotary >
}



//ОгрСвПолн
#[derive(Debug, Serialize, Deserialize)]
pub struct PoaLimitations {
    #[serde(rename = "@ИдОгр")]
    pub number: i32,

    #[serde(rename = "@КодОгр")]
    pub code: String1_255,

    #[serde(rename = "@НаимОгр")]
    pub name: String1_255,

    #[serde(rename = "@НаимЗначОгр")]
    pub value_name: Option<String1_255>,

    #[serde(rename = "$value")]
    pub value: ConfineValue,
}



//МашПолн
#[derive(Debug, Serialize, Deserialize)]
pub struct MchdPower {
    #[serde(rename = "@МнПолн")]
    pub powers_mnemonic: Option<String6_255>,

    #[serde(rename = "@КодПолн")]
    pub powers_code: String6_255,

    #[serde(rename = "@НаимПолн")]
    pub powers_name: String6_255,

    #[serde(rename = "ОгрСвПолн", default, skip_serializing_if = "Vec::is_empty")]
    pub poa_limitations: Vec<PoaLimitations>,
}



//СвПолнТип
#[derive(Debug, Serialize, Deserialize)]
pub struct DelegatePowers {
    #[serde(rename = "@ТипПолн")]
    pub power_type: PowerType,

    #[serde(rename = "@ПрСовмПолн")]
    pub power_common_type: PowerCommonType,

    #[serde(rename = "@ПрУтрПолн")]
    pub redelegate_power_loss: Option<RedelegatePowerLossType>,

    #[serde(rename = "ТекстПолн")]
    pub power_text: Option<String1_10000>,

    #[serde(rename = "МашПолн", default, skip_serializing_if = "Vec::is_empty")]
    pub mchd_powers: Vec<MchdPower>
}



//Довер
#[derive(Debug, Serialize, Deserialize)]
pub struct Poa {
    #[serde(rename = "СвДов")]
    pub poa_metadata: PoaMetadata,

    #[serde(rename = "СвДоверит")]
    pub principal: Vec<PrincipalInfo>,

    #[serde(rename = "СвУпПред")]
    pub delegate: Vec<DelegateInfo>,

    #[serde(rename = "СвПолн")]
    pub delegate_powers: DelegatePowers,

    #[serde(rename = "СвНотУд")]
    pub notarial_certification: Option<NotarialCertification>,
}



//Доверит
#[derive(Debug, Serialize, Deserialize)]
pub struct Principal {
    #[serde(rename = "РосОргДовер")]
    pub russian_org: Option<RussOrgPrincipal>,

    #[serde(rename = "ИнОргДовер")]
    pub foreign_org: Option<ForeignOrgPrincipal>,

    #[serde(rename = "ИПДовер")]
    pub ip: Option<Ip>,

    #[serde(rename = "ФЛДовер")]
    pub person: Option<PrincipalPerson>,
}



//СвДоверит
#[derive(Debug, Serialize, Deserialize)]
pub struct PrincipalInfo {
    #[serde(rename = "@ТипДоверит")]
    pub principal_identity: PrincipalIdentity,

    #[serde(rename = "Доверит")]
    pub principal: Principal,
}


//Передов
#[derive(Debug, Serialize, Deserialize)]
pub struct RedelegatedPoa {
    #[serde(rename = "@ПрНалПередов")]
    pub redelegate_flag: Flag,

    #[serde(rename = "СвПервДовер")]
    pub original_poa: OriginalPoa,

    #[serde(rename = "СвПередов")]
    pub prev_poa: Option<OriginalPoa>,

    #[serde(rename = "СвПереДовер")]
    pub poa_metadata: PoaMetadata,

    #[serde(rename = "СвПередПолн")]
    pub sub_principals: Vec<SubPrincipal>,

    #[serde(rename = "СвПолучПолн")]
    pub delegates: Vec<DelegateInfo>,

    #[serde(rename = "СвПолн")]
    pub delegate_powers: DelegatePowers,

    #[serde(rename = "СвНотУд")]
    pub notary_sertification: Option<NotarialCertification>,
}

//ФЛПерПолн
#[derive(Debug, Serialize, Deserialize)]
pub struct RedelegatePerson {
    #[serde(rename = "@СтУчНД")]
    pub participant_status: Option<ParticipantStatus>,

    #[serde(rename = "@ПрНалРук")]
    pub signatory_flag: Option<Flag>,

    #[serde(rename = "@ИННФЛ")]
    pub inn: PersInn,

    #[serde(rename = "@СНИЛС")]
    pub snils: Snils,

    #[serde(rename = "СведРукоп")]
    pub signatory: Option<JuridicalWrapPerson>,

    #[serde(rename = "СведФЛ")]
    pub person: JuridicalWrapPerson,
}


//ФилПерПолн
#[derive(Debug, Serialize, Deserialize)]
pub struct RedelegaterFilial {
    #[serde(rename = "СвФил")]
    pub filial: RussOrganization,

    #[serde(rename = "СвРукФил")]
    pub manager: JuridicalWrapPerson,
}


// ПередПолн
#[derive(Debug, Serialize, Deserialize)]
pub struct SubPrincipalInfo {
    #[serde(rename = "РосОргПерПолн")]
    pub russian_organization: Option<RussOrgPrincipal>,

    #[serde(rename = "ИППерПолн")]
    pub ip: Option<Ip>,

    #[serde(rename = "ФЛПерПолн")]
    pub person: Option<RedelegatePerson>,

    #[serde(rename = "ФилПерПолн")]
    pub filial: Option<RedelegaterFilial>,

    #[serde(rename = "ИнПерПолн")]
    pub foreign_organizetioan: Option<ForeignOrgPrincipal>
}

//СвПередПолн
#[derive(Debug, Serialize, Deserialize)]
pub struct SubPrincipal {
    #[serde(rename = "@ТипПерПолн")]
    pub sub_principal_type: DelegateType,

    #[serde(rename = "ПередПолн")]
    pub sub_principal: SubPrincipalInfo
}


//Документ
#[derive(Debug, Serialize, Deserialize)]
pub struct PoaWrap {
    #[serde(rename = "@КНД")]
    pub code_knd: Option<String7_7>,

    #[serde(rename = "$value")]
    pub poa_doc: PoaRedelegateaChoice,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, sqlx::Type, ts_rs::TS)]
pub enum Gender {
    #[serde(rename = "1")]
    Male,
    #[serde(rename = "2")]
    Female,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum IsCitizen {
    #[serde(rename = "1")]
    Citizen,
    #[serde(rename = "2")]
    ForeignCitizen,
    #[serde(rename = "3")]
    StatelessPerson,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ManagementType {
    #[serde(rename = "1")]
    Sole,
    #[serde(rename = "2")]
    Joint,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Flag {
    #[serde(rename = "0")]
    FalseFlag,
    #[serde(rename = "1")]
    TrueFlag,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PoaTypeRevocable {
    #[serde(rename = "1")]
    Revocable,

    #[serde(rename = "2")]
    Irrevocable
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PoaTypeRedelegatable {
    #[serde(rename = "1")]
    Single,

    #[serde(rename = "2")]
    Once,

    #[serde(rename = "3")]
    Successive
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum IrrevocablePoaRedelegationType {
    #[serde(rename = "1")]
    Static,

    #[serde(rename = "2")]
    Single
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum IrrevocablePoaRevocationCondition {
    #[serde(rename = "1")]
    Conditional,

    #[serde(rename = "2")]
    Unconditional,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum DelegateType {
    #[serde(rename = "1")]
    LegalEntity,
    #[serde(rename = "2")]
    IndividualEnt,
    #[serde(rename = "3")]
    PhysicalPerson,
    #[serde(rename = "4")]
    Branch,
    #[serde(rename = "5")]
    ForeignBranch,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PrincipalIdentity  {
    #[serde(rename = "1")]
    RussianLegalEntity,
    
    #[serde(rename = "2")]
    ForeignLegalEntity,
    
    #[serde(rename = "3")]
    IndividualEntrepreneur,
    
    #[serde(rename = "4")]
    PhysicalPerson,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum InitPrincipalTypeShort {
    #[serde(rename = "1")]
    LegalEntity,

    #[serde(rename = "2")]
    PhysicalPerson,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PoaLegalForm {
    #[serde(rename = "1")]
    Simple,

    #[serde(rename = "2")]
    Notarial,
}



#[derive(Debug, Serialize, Deserialize)]
pub enum ConfineValue {
    #[serde(rename = "КодЗначОгр")]
    Code(String1_255),
    
    #[serde(rename = "ТексЗначОгр")]
    Text(String1_255),
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PowerType {
    #[serde(rename = "0")]
    Textual,

    #[serde(rename = "1")]
    MachineReadable,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum PowerCommonType {
    #[serde(rename = "1")]
    Joint,

    #[serde(rename = "2")]
    Individual,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum RedelegatePowerLossType {
    #[serde(rename = "1")]
    Lost,

    #[serde(rename = "2")]
    Retained,
}



#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum RedelegationStatus {
    #[serde(rename = "0")]
    Original,
    
    #[serde(rename = "1")]
    Redelegated,
}



#[derive(Debug, Serialize, Deserialize)]
pub enum AddressChoice {
    #[serde(rename = "АдрРФ")]
    AdrRf(String1_1000),
    
    #[serde(rename = "ФИАСАдрРФ")]
    FiasAdrRf(String1_1000),
}



#[derive(Debug, Serialize, Deserialize)]
pub enum FormatVersion {
    #[serde(rename = "EMCHD_1")]
    Emchd1,
}



#[derive(Debug, Serialize, Deserialize)]
pub enum PoaRedelegateaChoice {
    #[serde(rename = "Довер")]
    Poa(Box<Poa>),

    #[serde(rename = "Передов")]
    RedelegatedPoa(Box<RedelegatedPoa>), 
}