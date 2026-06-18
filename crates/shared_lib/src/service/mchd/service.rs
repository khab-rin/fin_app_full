use serde::{Serialize, Deserialize};

use crate::sql_models::person::implements::Person;
use crate::primitives::frozen::implements::*;
use crate::primitives::frozen::implements_base::*;
use crate::parsers::mchd::implements::*;

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum MchdStep {
    Loading {text: MchdInfo },
    TaxMchdMiss { pers: Person, text: MchdInfo },
    HomeMchdMiss { pers: Person, text: MchdInfo },
    TaxMchdFull { text: MchdInfo },
    HomeMchdFull { text: MchdInfo },
    TryLater {text: MchdInfo},
    Success {text: MchdInfo},
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

    #[serde(rename = "Доверенность успешно создана и проходит регистрацию в сервисе МЧД ФНС РФ")]
    Success,

    #[serde(rename = "")]
    Nothing,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewMchdData {
    pub poa_number: String1_50,
    pub poa_end_date: Date,

    pub manager_tittle: String1_255,
    pub manager_sur_name: SurName,
    pub manager_first_name: FirstName,
    pub manager_mid_name: MidName,
    pub manager_birth_day: Date,
    pub manager_snils: Snils,
    pub manager_inn: PersInn,
    pub manager_is_citizen: IsCitizen,

    pub user_sur_name: SurName,
    pub user_first_name: FirstName,
    pub user_mid_name: MidName,
    pub user_birth_day: Date,
    pub user_gender: Gender,
    pub user_snils: Snils,
    pub user_inn: PersInn,
    pub user_passport_number: PasspRfNumber,
    pub user_passport_issueer: String1_1000,
    pub user_passport_ussuer_code: String7_7,
    pub user_is_citizen: IsCitizen,
    pub powers: std::collections::HashSet<MchdPower>
}
