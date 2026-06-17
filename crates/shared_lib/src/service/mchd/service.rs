use serde::{Serialize, Deserialize};

use crate::sql_models::person::implements::Person;

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum MchdStep {
    Loading {text: MchdInfo },
    TaxMchdMiss { pers: Person, text: MchdInfo },
    HomeMchdMiss { pers: Person, text: MchdInfo },
    TaxMchdFull { text: MchdInfo },
    HomeMchdFull { text: MchdInfo },
    TryLater {text: MchdInfo}
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum MchdInfo {
    #[serde(rename = "Выберите тип доверенности, который нужно создать")]
    Loading,

    #[serde(rename = "В системе имеется МЧД для работы с ФНС. Если хотите обновить МЧД для работы с ФНС, нажмите кнопку удалить МЧД для работы с ФНС, затем пройдите создание МЧД заново (Осторожно! При удалении МЧД она больше не будет доступна)")]
    TaxMchdFull,

    #[serde(rename = "В системе имеется МЧД для работы в сервисе. Если хотите обновить МЧД для работы в сервисе, нажмите кнопку удалить МЧД для работы в сервисе, затем пройдите создание МЧД заново (Осторожно! При удалении МЧД она больше не будет доступна)")]
    HomeMchdFull,

    #[serde(rename = "Заполните поэтапно поля для доверенности в ФНС в том же формате, в каком они указаны в документах. Предупреждение: Сервис ФНС РФ МЧД проверяет данные на соответствие, проверяйте корректность всех полей")]
    TaxMchdMiss,

    #[serde(rename = "Заполните поэтапно поля для доверенности в системе. Предупреждение: Сервис ФНС РФ МЧД проверяет данные на соответствие, проверяйте корректность всех полей")]
    HomeMchdMiss,

    #[serde(rename = "Произошла ошибка при загрузке данных, повторите попытку позже...")]
    Failed,

    #[serde(rename = "Критическая ошибка на устройстве...")]
    ClientServiceError,

    #[serde(rename = "Критическая ошибка на серверной части приложения...")]
    BackApiError,

    #[serde(rename = "")]
    Nothing,
}
