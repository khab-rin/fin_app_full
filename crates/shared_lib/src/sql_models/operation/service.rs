use serde::{Serialize, Deserialize};

use crate::sql_models::operation::implements::OperationRaw;

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum OperationStep {
    AccInput { text: OperationInfo },

    Loading { text: OperationInfo },

    ManualInput { text: OperationInfo },

	ProcessSuccess {text: OperationInfo, count: i32 },

    StatementLoader { text: OperationInfo },

    StatementSuccess {
        text: OperationInfo,
        operations: Vec<OperationRaw>
    },

    TryLater { text: OperationInfo },

}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum OperationInfo {
    #[serde(rename = "Введите БИК банка и номер расчетного счета")]
    AccInput,

    #[serde(rename = "У вас недостаточно прав для доступа к этому разделу")]
    AccessDenied,

    #[serde(rename = "Ошибка в работе серверной части приложения, попробуйте авторизоваться позже, либо сделайте запрос в техподдержку")]
    BackApiError,

    #[serde(rename = "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение")]
    ClientApiSystemError,

    #[serde(rename = "Выберите функционал работы с проводками")]
    InitInfo,

    #[serde(rename = "Страница загружается, подождите пожалуйста. В случае зависания попробуйте обновить или перезагрузить приложение")]
    LoadingInfo,

    #[serde(rename = "Введите данные для проводок вручную")]
    ManualInput,

    #[serde(rename = "При необходимости отредактируйте или удалите проводки")]
    SuccessRaw,

    #[serde(rename = "Выберите расчетный счет и загрузите банковскую выписку")]
    StatementParser,

	#[serde(rename = "Выписка обработана, добавлено операций - ")]
	StatementSuccess,

    #[serde(rename = "Указанный банковский счет и банковский счет выписки не совпадают")]
    WrongBankAcc,

    #[serde(rename = "Возможно загруженный файл не является банковской выпиской установленного формата")]
    WrongFile,
    
    #[serde(rename = "")]
    Nothing,
}