use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use vec1::Vec1;

use crate::make_xls_enum;
use crate::primitives::frozen::text_base::*;
use crate::primitives::frozen::text::*;
use crate::primitives::composite::implements::Fio;


//СумТип
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsQuaterAmnts {
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
pub struct FnsQuaterAmntsInfo {
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
pub struct FnsTrnasformationInfo {
	#[serde(rename="@ФормРеорг")]
	pub liq_status: FnsLiquidStatus,

	#[serde(rename="@ИННЮЛ", skip_serializing_if = "Option::is_none")]
	pub comp_inn: Option<CompInn>,

	#[serde(rename="@КПП", skip_serializing_if = "Option::is_none")]
	pub kpp: Option<Kpp>,
}

//НПЮЛ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsTaxPayerCompany {
	#[serde(rename="СвРеоргЮЛ", skip_serializing_if = "Option::is_none")]
	pub transformation_info: Option<FnsTrnasformationInfo>,

	#[serde(rename="@НаимОрг")]
	pub comp_name: String1_1000,

	#[serde(rename="@ИННЮЛ")]
	pub comp_inn: CompInn,

	#[serde(rename="@КПП")]
	pub kpp: Kpp,

}

//НПФЛ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsTaxPayerPerson {
	#[serde(rename="ФИО")]
	pub fio: Fio,

	#[serde(rename="@ИННФЛ")]
	pub pers_inn: PersInn,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum FnsTaxPayerType {
	#[serde(rename = "НПЮЛ")]
	Company(FnsTaxPayerCompany),
	#[serde(rename = "НПФЛ")]
	Person(FnsTaxPayerPerson)
}





//СвНП
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsTaxPayer {
	#[serde(rename = "$value")]
	pub tax_payer: FnsTaxPayerType,
	#[serde(rename="@Тлф", skip_serializing_if = "Option::is_none")]
	pub tel: Option<Phone>
}


//СвПред
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsDelegateInfo {
	#[serde(rename = "@НаимДок")]
	pub doc_name: String1_120
}

//Подписант
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsSignature {
	#[serde(rename="ФИО", skip_serializing_if = "Option::is_none")]
	pub fio: Option<Fio>,

	#[serde(rename="СвПред", skip_serializing_if = "Option::is_none")]
	pub delegate_info: Option<FnsDelegateInfo>,

	#[serde(rename="@ПрПодп")]
	pub signature: FnsSignatureType
}

//Ставка
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsRate {
	#[serde(rename="@СтавкаКв", skip_serializing_if = "Option::is_none")]
	pub qu_one: Option<Decimal>,

	#[serde(rename="@СтавкаПг", skip_serializing_if = "Option::is_none")]
	pub qu_two: Option<Decimal>,

	#[serde(rename="@Ставка9м", skip_serializing_if = "Option::is_none")]
	pub qu_three: Option<Decimal>,

	#[serde(rename="@СтавкаНалПер")]
	pub qu_four: Decimal,

	#[serde(rename="@КодЛьгот")]
	pub rate_reason: String
}


//ТоргСборФакт
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsPaidTradeFee {
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
pub struct FnsTradeFee {
	#[serde(rename="Доход")]
	pub taxable_income: FnsQuaterAmnts,

	#[serde(rename="Исчисл")]
	pub calc_fee: FnsQuaterAmnts,

	#[serde(rename="УменНал")]
	pub red_amnt: FnsQuaterAmnts,

	#[serde(rename="ТоргСборФакт")]
	pub clean_fee: FnsPaidTradeFee,

	#[serde(rename="ТоргСборУмен")]
	pub fee_incl_tax: FnsQuaterAmnts

}


//РасчНал1
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsTaxSixCal {
	#[serde(rename="Доход")]
	pub taxable_income: FnsQuaterAmnts,
	
	#[serde(rename="Ставка")]
	pub rate: FnsRate,

	#[serde(rename="Исчисл")]
	pub calc_tax: FnsQuaterAmnts,

	#[serde(rename="УменНал")]
	pub tax_deduction: FnsQuaterAmnts,

	#[serde(rename="Стр143Разд2.1.1", skip_serializing_if = "Option::is_none")]
	pub required_social: Option<FnsQuaterAmntsInfo>,

	#[serde(rename="РасчТоргСбор", skip_serializing_if = "Option::is_none")]
	pub trade_fee: Option<FnsTradeFee>,

	#[serde(rename="@ПризНП")]
	pub employers_exist: FnsEmplyersExistType

}


//СумНалПУ_НП
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsTaxSix {
	#[serde(rename="РасчНал1")]
	pub calculation: FnsTaxSixCal,

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
pub enum FnsFifteenDueChoise {
	#[serde(rename="НалПУУменПер")]
	Due(u64),
	#[serde(rename="НалПУМин")]
	MinTaxAmnt(u64)
}

//СумНалПУ_СмНП
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsTaxFifteen {
	#[serde(rename="$value")]
	due_choice: FnsFifteenDueChoise,
	
	#[serde(rename="РасчНал2")]
	calculation: FnsTaxFifteenCal,

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

}


//СумНалПУ_СмНП
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsTaxFifteenCal {
	#[serde(rename="Доход")]
	pub income: FnsQuaterAmnts,

	#[serde(rename="Расход")]
	pub expenses: FnsQuaterAmnts,

	#[serde(rename="НалБазаУбыт")]
	pub taxable_base: FnsQuaterAmnts,

	#[serde(rename="Ставка")]
	pub rate: FnsRate,

	#[serde(rename="Исчисл", skip_serializing_if = "Option::is_none")]
	pub calculated_tax: Option<FnsQuaterAmnts>,

	#[serde(rename="Стр223Разд2.2", skip_serializing_if = "Option::is_none")]
	pub required_social: Option<FnsQuaterAmntsInfo>,

	#[serde(rename="@УбытПред", skip_serializing_if = "Option::is_none")]
	pub prev_negative_taxable_base: Option<u64>,
	
	#[serde(rename="@ИсчислМин")]
	pub min_tax: u64
}

//ОтчетИспКод
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsCharityReportCode {
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
pub struct FnsCharityReport {
	#[serde(rename="ОтчетИспКод")]
	pub charity_report_code: Vec1<FnsCharityReportCode>,
	
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
pub struct FnsKktExpenseCalc {
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
pub struct FnsKktExpense {
	#[serde(rename="СумРасхККТПер")]
	pub kkt_expense_calc: FnsKktExpenseCalc,

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
pub enum FnsTaxReportType {
	#[serde(rename="СумНалПУ_НП")]
	SixPercent(Box<FnsTaxSix>),

	#[serde(rename="СумНалПУ_СмНП")]
	FifteenPercent(Box<FnsTaxFifteen>)
}

//УСН
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsUsnReport {
	#[serde(rename = "$value")]
	pub retport_type: FnsTaxReportType,

	#[serde(rename = "ОтчетИсп", skip_serializing_if = "Option::is_none")]
	pub charity_report: Option<FnsCharityReport>,

	#[serde(rename = "РасчСумККТ", skip_serializing_if = "Option::is_none")]
	pub kkt_expense: Option<FnsKktExpense>,

	#[serde(rename = "@ОбНал")]
	pub usn_object: FnsUsnObject
}

//Документ
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsUsnDocument {
	#[serde(rename = "СвНП")]
	tax_payer: FnsTaxPayer,

	#[serde(rename = "Подписант")]
	signature: FnsSignature,

	#[serde(rename = "УСН")]
	usn_report: FnsUsnReport,

	#[serde(rename = "@КНД")]
	report_code: FnsUsnKnd,

	#[serde(rename = "@ДатаДок")]
	doc_create_date: Date,

	#[serde(rename = "@Период")]
	report_type: FnsReportType,

	#[serde(rename = "@ОтчетГод")]
	report_year: Digits4_4,

	#[serde(rename = "@КодНО")]
	fns_branch_code: Digits4_4,

	#[serde(rename = "@НомКорр")]
	report_version: u16,

	#[serde(rename = "@ПоМесту")]
	submission_place: FnsSubmissionPlace
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FnsUsnFile {
	#[serde(rename = "Документ")]
	document: FnsUsnDocument,

	#[serde(rename = "@ИдФайл")]
	file_id: String1_255,

	#[serde(rename = "@ВерсПрог")]
	program_version: String1_40,

	#[serde(rename = "@ВерсФорм")]
	format_version: FnsUsnFromtat
}




make_xls_enum!(FnsLiquidStatus, {
    LIQUIDATION => "0",
    TRANSFORMATION => "1",
    MERGER => "2",
    DIVISION => "3",
	ACQUISITION => "5",
    DivAcq => "6",
});


make_xls_enum!(FnsSignatureType, {
    TAXPAYER => "1",
    DELEGATE => "2",
});


make_xls_enum!(FnsEmplyersExistType, {
    WithEmployees => "1",
    WithoutEmployees => "2",
});

make_xls_enum!(FnsUsnObject, {
    Income => "1",
    IncomeMinusExpenses => "2",
});


make_xls_enum!(FnsUsnKnd, {
    Value => "1152017",
});


make_xls_enum!(FnsReportType, {
    Year => "34",
	ReorganizationOrLiquidation => "50",
	TaxModeChange => "95",
	ActivityCeased => "96",
});


make_xls_enum!(FnsSubmissionPlace, {
    IndividualBusinessAddress => "120",
	CompanyAddress => "210",
	SuccessorAddress => "215",
});

make_xls_enum!(FnsUsnFromtat, {
    Value => "5.09",
});



