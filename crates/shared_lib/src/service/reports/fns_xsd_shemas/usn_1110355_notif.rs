use serde::{Serialize, Deserialize};

use crate::make_xls_enum;
use crate::primitives::composite::implements::Fio;
use crate::primitives::frozen::text::{CompInn, Kpp, Oktmo, Date};
use crate::primitives::calculated::implements::RubC;
use crate::service::reports::fns_xsd_shemas::common::*;
use crate::primitives::frozen::text_base::{
	String1_40, Digits4_4, String1_120, String1_255, String1_1000
};



//НПЮЛ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnNotifTaxPayerCompany {
	#[serde(rename="@ИННЮЛ")]
	pub comp_inn: CompInn,

	#[serde(rename="@КПП")]
	pub kpp: Kpp,
}

//НПИП
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnNotifTaxPayerPerson {
	#[serde(rename="@ИННФЛ")]
	pub pers_inn: CompInn,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UsnNotifTaxPayerChoice {
	#[serde(rename="НПЮЛ")]
	Company(UsnNotifTaxPayerCompany),

	#[serde(rename="НПИП")]
	Person(UsnNotifTaxPayerPerson)
}


//СвПред
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnNotifDelegateInfo {

	#[serde(rename="@НаимДок")]
	pub delegate_doc_info: String1_120,

	#[serde(rename="@НаимОрг", skip_serializing_if = "Option::is_none")]
	pub delegate_comp_name: Option<String1_1000>
}

//Подписант
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnNotifSigner {
	#[serde(rename="@ПрПодп")]
	pub signer_type: FnsSignerType,

	#[serde(rename="СвПред", skip_serializing_if = "Option::is_none")]
	pub delegate_info: Option<UsnNotifDelegateInfo>,

	#[serde(rename="ФИО")]
	pub fio: Fio,
}

//УвИсчСумНалог
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnNotifNotification {
	#[serde(rename="@КППДекл", skip_serializing_if = "Option::is_none")]
	pub kpp: Option<Kpp>,

	#[serde(rename="@ОКТМО")]
	pub oktmo: Oktmo,

	#[serde(rename="@КБК")]
	pub kbk: UsnNotifKbk,

	#[serde(rename="@СумНалогАванс")]
	pub avans_amnt: RubC,

	#[serde(rename="@Период")]
	pub period: UsnNotifPeriod,

	#[serde(rename="@НомерМесКварт")]
	pub qu_month_num: UsnNotifPeriodNum,

	#[serde(rename="@Год")]
	pub year: Digits4_4,
}



//Документ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnNotifDocument {
	#[serde(rename="@КНД")]
	pub knd: FnsKnd,

	#[serde(rename="@ДатаДок")]
	pub doc_date: Date,

	#[serde(rename = "@КодНО")]
	pub branch_code: Digits4_4,

	#[serde(rename="$value")]
	pub tax_payer: UsnNotifTaxPayerChoice,

	#[serde(rename="Подписант")]
	pub signer: UsnNotifSigner,

	#[serde(rename="УвИсчСумНалог")]
	pub notifications: vec1::Vec1<UsnNotifNotification>,
}

//Файл
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnNotifFile {
	#[serde(rename = "@ИдФайл")]
	pub file_id: String1_255,

	#[serde(rename = "@ВерсПрог")]
	pub program_version: String1_40,

	#[serde(rename = "@ВерсФорм")]
	pub format_version: FnsDocFormVersion,

	#[serde(rename="Документ")]
	pub document: UsnNotifDocument,
}





make_xls_enum!(UsnNotifPeriod, {
    FirstQuarter => "21",
    HalfYear     => "31",
    NineMonths   => "33",
    Year         => "34",
});


make_xls_enum!(UsnNotifPeriodNum, {
    QuOne => "01",
    QuTwo => "02",
	QuThree => "03",
    QuFour => "04",
});




make_xls_enum!(UsnNotifKbk, {
    UsnNotifSix => "18210501011011000110",
	UsnNotifFifteen => "18210501021011000110"
});