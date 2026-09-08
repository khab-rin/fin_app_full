
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum ReportStep {
	FnsReports { text: ReportInfo },

	HomeReports {text: ReportInfo },

	Loading { text: ReportInfo },

	SaveFiles { text: ReportInfo, xls_file: Vec<u8>, pdf_file: Vec<u8> },

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
pub enum FnsReportType {
	#[serde(rename = "Отчет УСН => доходы 6%")]
	UsnSix,

	#[serde(rename = "Отчет УСН => доходы - расходы 15%")]
	UsnFifteen,
}

impl FnsReportType {
	pub fn get_all_fns_report_types() -> Vec<FnsReportType> {
		vec![
			Self::UsnSix,
			Self::UsnFifteen
		]
	}

}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct ReportTSTS {
	report_step: ReportStep,
	report_info: ReportInfo,
	fns_report_type: FnsReportType
}
