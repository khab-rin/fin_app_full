use serde::{Serialize, Deserialize};


use crate::Status;
use crate::sql_models::company::implements::Company;
use crate::sql_models::person::implements::Person;
use crate::sql_models::contracts::implements::Contract;
use crate::sql_models::operation::implements::Operation;
use crate::primitives::frozen::text::{BoxUuid};

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum DatabaseStep {
	Loading { text: DatabaseInfo },
	SyncCompanysToServer {companys: Vec<Company>},
	SyncCompanysToClient {del_companys: Vec<Company>, server_companys: Vec<Company>},
	SyncContracts {text: DatabaseInfo, contracts: Vec<Contract>, user_id: BoxUuid},
	SyncPersons {text: DatabaseInfo, persons: Vec<Person>, user_id: BoxUuid},
	SyncOperations {text: DatabaseInfo, operations: Vec<Operation>, user_id: BoxUuid},
	SyncSuccess {text: DatabaseInfo},
	TryLater {text: DatabaseInfo },	
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum DatabaseInfo {
	#[serde(rename = "Критическая ошибка на серверной части приложения...")]
    BackApiError,

	#[serde(rename = "Критическая ошибка на устройстве...")]
    ClientServiceError,

	#[serde(rename = "Проблема соединения, попробуйте выключить ВПН")]
	ConnectionError,

	#[serde(rename="База данных успешно синхронизирована")]
	DataBaseSyncSuccess,

	#[serde(rename = "Выберите функционал для работы с базой данных")]
	Loading,

	#[serde(rename = "Отсутствуют полномочия для выполнения этой операции")]
	MissPower,

	#[serde(rename = "Отсутствуют привязанная к пользователю организация")]
	MissUser,

	#[serde(rename = "")]
	Notning,

	#[serde(rename = "База данных успешно сихнронизирована")]
	SqlSyncSuccess,

	#[serde(rename = "Ошибка! Требуется синхронизация базы данных")]
	SqlNeedSync,
}

impl DatabaseStep {
	pub fn process_info(&mut self, err: &Status) {
		if let Self::TryLater { text } = self {
			match err {
				Status::ConnectProblems => *text = DatabaseInfo::ConnectionError,
				Status::SqlSyncError => *text = DatabaseInfo::SqlNeedSync,
				_ => {}
			}
		}
	}
}

impl DatabaseStep {
	pub fn from_err(err: &Status) -> Self {
		let info: DatabaseInfo = match err {
			Status::ConnectProblems => DatabaseInfo::ConnectionError,
			Status::SqlSyncError => DatabaseInfo::SqlNeedSync,
			Status::BackApiError => DatabaseInfo::BackApiError,
			_ => DatabaseInfo::ClientServiceError
		};
		Self::TryLater { text: info }
	}
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct DatabaseStepRSTS {
	database_step: DatabaseStep,
	database_info: DatabaseInfo
}