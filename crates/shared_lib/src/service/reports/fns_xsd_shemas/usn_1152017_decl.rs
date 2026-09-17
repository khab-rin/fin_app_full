use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use vec1::Vec1;

use crate::make_xls_enum;
use crate::primitives::frozen::text_base::*;
use crate::primitives::frozen::text::*;
use crate::primitives::composite::implements::Fio;
use crate::service::reports::fns_xsd_shemas::common::*;


//СумТип
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclQuaterAmnts {
	#[serde(rename="@СумЗаКв", skip_serializing_if = "Option::is_none")]
	pub first_qu: Option<u64>,

	#[serde(rename="@СумЗаПг", skip_serializing_if = "Option::is_none")]
	pub second_qu: Option<u64>,

	#[serde(rename="@СумЗа9м", skip_serializing_if = "Option::is_none")]
	pub third_qu: Option<u64>,

	#[serde(rename="@СумЗаНалПер")]
	pub fourth_qu: u64,
}


//СумСтрТип
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclQuaterAmntsInfo {
	#[serde(rename="@ФиксРазм")]
	pub fix_amnt: u64,

	#[serde(rename="@ДохПрев300")]
	pub one_perc: u64,

	#[serde(rename="@РасПерТекГод")]
	pub one_perc_curr_year: u64,

	#[serde(rename="@РасПерПредГод")]
	pub one_perc_prev_year: u64,
}


//СвРеоргЮЛ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclTrnasformationInfo {
	#[serde(rename="@ФормРеорг")]
	pub liq_status: UsnDeclLiquidStatus,

	#[serde(rename="@ИННЮЛ", skip_serializing_if = "Option::is_none")]
	pub comp_inn: Option<CompInn>,

	#[serde(rename="@КПП", skip_serializing_if = "Option::is_none")]
	pub kpp: Option<Kpp>,
}


//НПЮЛ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclTaxPayerCompany {
	#[serde(rename="СвРеоргЮЛ", skip_serializing_if = "Option::is_none")]
	pub transformation_info: Option<UsnDeclTrnasformationInfo>,

	#[serde(rename="@НаимОрг")]
	pub comp_name: String1_1000,

	#[serde(rename="@ИННЮЛ")]
	pub comp_inn: CompInn,

	#[serde(rename="@КПП")]
	pub kpp: Kpp,

}


//НПФЛ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclTaxPayerPerson {
	#[serde(rename="ФИО")]
	pub fio: Fio,

	#[serde(rename="@ИННФЛ")]
	pub pers_inn: CompInn,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UsnDeclTaxPayerType {
	#[serde(rename = "НПЮЛ")]
	Company(UsnDeclTaxPayerCompany),
	#[serde(rename = "НПФЛ")]
	Person(UsnDeclTaxPayerPerson)
}


//СвНП
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclTaxPayer {
	#[serde(rename = "$value")]
	pub tax_payer: UsnDeclTaxPayerType,
	#[serde(rename="@Тлф", skip_serializing_if = "Option::is_none")]
	pub tel: Option<Phone>
}


//СвПред
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclDelegateInfo {
	#[serde(rename = "@НаимДок")]
	pub doc_name: String1_120
}


//Подписант
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclSigner {
	#[serde(rename="ФИО", skip_serializing_if = "Option::is_none")]
	pub fio: Option<Fio>,

	#[serde(rename="СвПред", skip_serializing_if = "Option::is_none")]
	pub delegate_info: Option<UsnDeclDelegateInfo>,

	#[serde(rename="@ПрПодп")]
	pub signer: FnsSignerType
}


//Ставка
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclRate {
	#[serde(rename="@СтавкаКв", skip_serializing_if = "Option::is_none")]
	pub qu_one: Option<Decimal>,

	#[serde(rename="@СтавкаПг", skip_serializing_if = "Option::is_none")]
	pub qu_two: Option<Decimal>,

	#[serde(rename="@Ставка9м", skip_serializing_if = "Option::is_none")]
	pub qu_three: Option<Decimal>,

	#[serde(rename="@СтавкаНалПер")]
	pub qu_four: Decimal,

	#[serde(rename="@КодЛьгот", skip_serializing_if = "Option::is_none")]
	pub rate_reason: Option<String>
}


//ТоргСборФакт
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclPaidTradeFee {
	#[serde(rename="@СумТечКв", skip_serializing_if = "Option::is_none")]
	pub qu_one: Option<u64>,

	#[serde(rename="@СумТечПг", skip_serializing_if = "Option::is_none")]
	pub qu_two: Option<u64>,

	#[serde(rename="@СумТеч9м", skip_serializing_if = "Option::is_none")]
	pub qu_three: Option<u64>,

	#[serde(rename="@СумТечНалПер")]
	pub qu_four: u64
}


//РасчТоргСбор
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclTradeFee {
	#[serde(rename="Доход")]
	pub taxable_income: UsnDeclQuaterAmnts,

	#[serde(rename="Исчисл")]
	pub calc_fee: UsnDeclQuaterAmnts,

	#[serde(rename="УменНал")]
	pub red_amnt: UsnDeclQuaterAmnts,

	#[serde(rename="ТоргСборФакт")]
	pub clean_fee: UsnDeclPaidTradeFee,

	#[serde(rename="ТоргСборУмен")]
	pub fee_incl_tax: UsnDeclQuaterAmnts
}


//РасчНал1
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclTaxSixCal {
	#[serde(rename="Доход")]
	pub taxable_income: UsnDeclQuaterAmnts,
	
	#[serde(rename="Ставка")]
	pub rate: UsnDeclRate,

	#[serde(rename="Исчисл")]
	pub calc_tax: UsnDeclQuaterAmnts,

	#[serde(rename="УменНал")]
	pub tax_deduction: UsnDeclQuaterAmnts,

	#[serde(rename="Стр143Разд2.1.1", skip_serializing_if = "Option::is_none")]
	pub required_social: Option<UsnDeclQuaterAmntsInfo>,

	#[serde(rename="РасчТоргСбор", skip_serializing_if = "Option::is_none")]
	pub trade_fee: Option<UsnDeclTradeFee>,

	#[serde(rename="@ПризНП")]
	pub employers_exist: UsnDeclEmplyersExistType

}


//СумНалПУ_НП
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclTaxSix {
	#[serde(rename="РасчНал1")]
	pub calculation: UsnDeclTaxSixCal,

	#[serde(rename="@ОКТМО")]
	pub oktmo_qu_one: Oktmo,

	#[serde(rename="@АвПУКв", skip_serializing_if = "Option::is_none")]
	pub avans_qu_one: Option<u64>,

	#[serde(rename="@ОКТМО_Пг", skip_serializing_if = "Option::is_none")]
	pub oktmo_qu_two: Option<Oktmo>,

	#[serde(rename="@АвПУУменПг", skip_serializing_if = "Option::is_none")]
	pub avans_qu_two: Option<u64>,

	#[serde(rename="@ОКТМО_9м", skip_serializing_if = "Option::is_none")]
	pub oktmo_qu_three: Option<Oktmo>,

	#[serde(rename="@АвПУУмен9м", skip_serializing_if = "Option::is_none")]
	pub avans_qu_three: Option<u64>,

	#[serde(rename="@ОКТМО_Пер", skip_serializing_if = "Option::is_none")]
	pub oktmo_qu_four: Option<Oktmo>,

	#[serde(rename="@НалПУУменПер")]
	pub avans_qu_four: u64,

	#[serde(rename="@СумНалПат", skip_serializing_if = "Option::is_none")]
	pub patent_tax: Option<u64>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UsnDeclFifteenDueChoise {
	#[serde(rename="НалПУУменПер")]
	Due(u64),
	#[serde(rename="НалПУМин")]
	MinTaxAmnt(u64)
}


//СумНалПУ_СмНП
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclTaxFifteen {
	#[serde(rename="$value")]
	due_choice: UsnDeclFifteenDueChoise,
	
	#[serde(rename="РасчНал2")]
	calculation: UsnDeclTaxFifteenCal,

	#[serde(rename="@ОКТМО")]
	pub oktmo_qu_one: Oktmo,

	#[serde(rename="@АвПУКв", skip_serializing_if = "Option::is_none")]
	pub avans_qu_one: Option<u64>,

	#[serde(rename="@ОКТМО_Пг", skip_serializing_if = "Option::is_none")]
	pub oktmo_qu_two: Option<Oktmo>,

	#[serde(rename="@АвПУУменПг", skip_serializing_if = "Option::is_none")]
	pub avans_qu_two: Option<u64>,

	#[serde(rename="@ОКТМО_9м", skip_serializing_if = "Option::is_none")]
	pub oktmo_qu_three: Option<Oktmo>,

	#[serde(rename="@АвПУУмен9м", skip_serializing_if = "Option::is_none")]
	pub avans_qu_three: Option<u64>,

	#[serde(rename="@ОКТМО_Пер", skip_serializing_if = "Option::is_none")]
	pub oktmo_qu_four: Option<Oktmo>,

	#[serde(rename="@СумНалПат", skip_serializing_if = "Option::is_none")]
	pub avans_qu_four: Option<u64>,

}


//СумНалПУ_СмНП
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclTaxFifteenCal {
	#[serde(rename="Доход")]
	pub income: UsnDeclQuaterAmnts,

	#[serde(rename="Расход")]
	pub expenses: UsnDeclQuaterAmnts,

	#[serde(rename="НалБазаУбыт")]
	pub taxable_base: UsnDeclQuaterAmnts,

	#[serde(rename="Ставка")]
	pub rate: UsnDeclRate,

	#[serde(rename="Исчисл", skip_serializing_if = "Option::is_none")]
	pub calculated_tax: Option<UsnDeclQuaterAmnts>,

	#[serde(rename="Стр223Разд2.2", skip_serializing_if = "Option::is_none")]
	pub required_social: Option<UsnDeclQuaterAmntsInfo>,

	#[serde(rename="@УбытПред", skip_serializing_if = "Option::is_none")]
	pub prev_negative_taxable_base: Option<u64>,
	
	#[serde(rename="@ИсчислМин")]
	pub min_tax: u64
}


//ОтчетИспКод
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclCharityReportCode {
	#[serde(rename="@КодВидПост")]
	pub charity_code: String3_3,

	#[serde(rename="@ДатаПост")]
	pub charity_date: Option<Date>,

	#[serde(rename="@СумДенСред")]
	pub charity_amnt: u64,

	#[serde(rename="@СумИспСрок")]
	pub used_amnt: Option<u64>,

	#[serde(rename="@СрокИсп")]
	pub used_date: Option<Date>,

	#[serde(rename="@СумИспНеСрок")]
	pub amnt_reminder: Option<u64>,

	#[serde(rename="@СумНеИспСрок")]
	pub uncorrect_used_amnt: Option<u64>,
}


//ОтчетИсп
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclCharityReport {
	#[serde(rename="ОтчетИспКод")]
	pub charity_report_code: Vec1<UsnDeclCharityReportCode>,
	
	#[serde(rename="@СумДенСредИт")]
	pub total_report_amnt: u64,

	#[serde(rename="@СумИспСрокИт")]
	pub correct_amnt: Option<u64>,

	#[serde(rename="@СумИспНеСрокИт")]
	pub unused_amnt: Option<u64>,

	#[serde(rename="@СумНеИспСрокИт")]
	pub incorrectly_used_amount: Option<u64>
}


//СумРасхККТПер
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclKktExpenseCalc {
	#[serde(rename="@СумЗаКв", skip_serializing_if = "Option::is_none")]
	pub qu_one: Option<u64>,

	#[serde(rename="@СумЗаПг", skip_serializing_if = "Option::is_none")]
	pub qu_two: Option<u64>,

	#[serde(rename="@СумЗа9м", skip_serializing_if = "Option::is_none")]
	pub qu_three: Option<u64>,

	#[serde(rename="@СумЗаНалПер")]
	pub qu_four: u64
}


//РасчСумККТ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclKktExpense {
	#[serde(rename="СумРасхККТПер")]
	pub kkt_expense_calc: UsnDeclKktExpenseCalc,

	#[serde(rename="@НаимККТ")]
	pub kkt_name: String1_40,

	#[serde(rename="@НомККТ")]
	pub kkt_fact_num: String1_20,

	#[serde(rename="@РегНомККТ")]
	pub kkt_fns_num: String16_16,

	#[serde(rename="@ДатаРегККТ")]
	pub kkt_reg_date: Date,

	#[serde(rename="@СумРасхККТ")]
	pub kkt_buy_amnt: u64,

	#[serde(rename="@СумРасхККТУм", skip_serializing_if = "Option::is_none")]
	pub kkt_deduction_2024: Option<u64>
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UsnDeclTaxReportType {
	#[serde(rename="СумНалПУ_НП")]
	SixPercent(Box<UsnDeclTaxSix>),

	#[serde(rename="СумНалПУ_СмНП")]
	FifteenPercent(Box<UsnDeclTaxFifteen>)
}


//УСН
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclUsnReport {
	#[serde(rename = "$value")]
	pub retport_type: UsnDeclTaxReportType,

	#[serde(rename = "ОтчетИсп", skip_serializing_if = "Option::is_none")]
	pub charity_report: Option<UsnDeclCharityReport>,

	#[serde(rename = "РасчСумККТ", skip_serializing_if = "Option::is_none")]
	pub kkt_expense: Option<UsnDeclKktExpense>,

	#[serde(rename = "@ОбНал")]
	pub usn_object: UsnDeclUsnObject
}


//Документ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclUsnDocument {
	#[serde(rename = "@КНД")]
	report_code: FnsKnd,

	#[serde(rename = "@ДатаДок")]
	doc_create_date: Date,

	#[serde(rename = "@Период")]
	report_type: UsnDeclReportType,

	#[serde(rename = "@ОтчетГод")]
	report_year: Digits4_4,

	#[serde(rename = "@КодНО")]
	branch_code: Digits4_4,

	#[serde(rename = "@НомКорр")]
	report_version: u16,

	#[serde(rename = "@ПоМесту")]
	submission_place: UsnDeclSubmissionPlace,

	#[serde(rename = "СвНП")]
	tax_payer: UsnDeclTaxPayer,

	#[serde(rename = "Подписант")]
	signer: UsnDeclSigner,

	#[serde(rename = "УСН")]
	usn_report: UsnDeclUsnReport,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UsnDeclUsnFile {
	#[serde(rename = "Документ")]
	document: UsnDeclUsnDocument,

	#[serde(rename = "@ИдФайл")]
	file_id: String1_255,

	#[serde(rename = "@ВерсПрог")]
	program_version: String1_40,

	#[serde(rename = "@ВерсФорм")]
	format_version: FnsDocFormVersion
}



make_xls_enum!(UsnDeclLiquidStatus, {
    LIQUIDATION => "0",
    TRANSFORMATION => "1",
    MERGER => "2",
    DIVISION => "3",
	ACQUISITION => "5",
    DivAcq => "6",
});


make_xls_enum!(UsnDeclEmplyersExistType, {
    WithEmployees => "1",
    WithoutEmployees => "2",
});

make_xls_enum!(UsnDeclUsnObject, {
    Income => "1",
    IncomeMinusExpenses => "2",
});



make_xls_enum!(UsnDeclReportType, {
    Year => "34",
	ReorganizationOrLiquidation => "50",
	TaxModeChange => "95",
	ActivityCeased => "96",
});


make_xls_enum!(UsnDeclSubmissionPlace, {
    IndividualBusinessAddress => "120",
	CompanyAddress => "210",
	SuccessorAddress => "215",
});




