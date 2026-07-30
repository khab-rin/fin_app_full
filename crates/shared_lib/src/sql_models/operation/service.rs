use serde::{Serialize, Deserialize};

use crate::sql_models::operation::implements::{OperationRaw, Operation};

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum OperationStep {
    AccessDenied { text: OperationInfo },

    Loading { text: OperationInfo },

    SuccessRaw {
        text: OperationInfo,
        operations: Vec<OperationRaw>
    },

    Success {
        operations: Vec<Operation>
    },

    TryLater { text: OperationInfo },

}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum OperationInfo {
    #[serde(rename = "У вас недостаточно прав для доступа к этому разделу")]
    AccessDenied,

    #[serde(rename = "Ошибка в работе серверной части приложения, попробуйте авторизоваться позже, либо сделайте запрос в техподдержку")]
    BackApiError,
    
    #[serde(rename = "Выберите банковский счет и загрузите файл выписки по данному выбранному счету")]
    BankParser,

    #[serde(rename = "Критическая ошибка в работе программы на устройстве пользователя, попробуйте обновить или перезагрузить приложение")]
    ClientApiSystemError,

    #[serde(rename = "Выберите способ создания проводок")]
    InitInfo,

    #[serde(rename = "Страница загружается, подождите пожалуйста. В случае зависания попробуйте обновить или перезагрузить приложение")]
    LoadingInfo,

    #[serde(rename = "При необходимости отредактируйте проводки")]
    SuccessRaw,

    #[serde(rename = "Указанный банковский счет и банковский счет выписки не совпадают")]
    WrongBankAcc,

    #[serde(rename = "Возможно загруженный файл не является банковской выпиской установленного формата")]
    WrongFile,
    
    #[serde(rename = "")]
    Nothing,
}