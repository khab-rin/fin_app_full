
use serde::{Serialize, Deserialize};

use crate::primitives::frozen::text::{RubF, Date};

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum ReportStep {
	FnsReports { text: ReportInfo },

	HomeReports {text: ReportInfo },

	Loading { text: ReportInfo },

	SaveFiles { 
		text: ReportInfo, 
		xml_name: String,
		xml_file: Vec<u8>,
		pdf_name: String, 
		pdf_file: Vec<u8> },

	TryLater {text: ReportInfo },
}


#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum ReportInfo {
	#[serde(rename = "Критическая ошибка на серверной части приложения...")]
    BackApiError,

	#[serde(rename = "Критическая ошибка на устройстве...")]
    ClientServiceError,

	#[serde(rename = "Выберите тип управленческого отчета и период")]
	HomeReports,

	#[serde(rename = "Выберите тип отчета в налоговую и период")]
	FnsReports,

	#[serde(rename = "Выберите функционал для работы с отчетами")]
	Loading,

	#[serde(rename = "")]
	Notning,

	#[serde(rename = "Файлы успешно сгенерированы, сохраните их в формате xls и pdf")]
	SaveFiles,

	#[serde(rename = "Был введен некоректный год")]
	WrongYear
}


#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
#[ts(export)]
pub enum FnsReportType {
    #[serde(rename = "Декларация УСН (Доходы 6%)")]
    UsnDeclSix,

    #[serde(rename = "Декларация УСН (Доходы - Расходы 15%)")]
    UsnDeclFifteen,

    #[serde(rename = "Уведомление УСН (Доходы 6%)")]
    UsnNotifSix,

    #[serde(rename = "Уведомление УСН (Доходы - Расходы 15%)")]
    UsnNotifFifteen,
}

impl FnsReportType {
	pub fn get_all_fns_report_types() -> Vec<FnsReportType> {
		vec![
			Self::UsnDeclSix,
			Self::UsnDeclFifteen,
			Self::UsnNotifSix,
			Self::UsnNotifFifteen
		]
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuartDates {
	pub start: Date,
	pub q1: Date,
	pub q2: Date,
	pub q3: Date,
	pub q4: Date
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuartCummulAmnt {
	pub q1: RubF,
	pub q2: RubF,
	pub q3: RubF,
	pub q4: RubF
}


#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct ReportTSTS {
	report_step: ReportStep,
	report_info: ReportInfo,
	fns_report_type: FnsReportType
}
