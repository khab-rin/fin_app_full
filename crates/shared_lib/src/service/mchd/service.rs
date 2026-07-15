use serde::{Serialize, Deserialize};

use crate::primitives::frozen::implements::*;
use crate::primitives::frozen::implements_base::*;
use crate::service::mchd::implements::*;
use crate::service::mchd::home_mchd_power::HomeMchdPower;
use crate::service::mchd::poa::PoaMchd;

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum MchdStep {
    BTBMchd { text: MchdInfo },
    FnsMchd { text: MchdInfo },
    HomeMchd { text: MchdInfo },
    LendMchd {text: MchdInfo },
    Loading {text: MchdInfo },
    SaveXmlDocFiles { 
        doc_name: String1_255,
        doc_file: Vec<u8>,
        xml_name: String1_255,
        xml_file: Vec<u8>,
        text: MchdInfo
    },
    ShowPowers {
        text: MchdInfo
    },
    
    SuccessRegisterMchd {
        guid: BoxUuid,
        text: TextInfo
    },
    TryLater {text: MchdInfo},
    WrongData {text: MchdInfo},
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum MchdInfo {
    #[serde(rename = "Вы на этапе создания доверенности для электронного документооборота с контрагентами, внимательно заполните поля аналогично полям в личных и учредительных документах")]
    BTBMchd,

    #[serde(rename = "Вы на этапе создания доверенности для сдачи отчетности в ФНС, внимательно заполните поля аналогично полям в личных и учредительных документах")]
    FnsMchd,

    #[serde(rename = "Вы на этапе создания доверенности для допуска к ветвям функционала данной системы, внимательно заполните поля аналогично полям в личных и учредительных документах")]
    HomeMchd,

    #[serde(rename = "Выберите ранее созданный xml файл доверенности, отсоединенный фалй подписи руководителя организации и отправьте доверенность для регистрации в сервисе МЧД")]
    LendMchd,

    #[serde(rename = "Выберите действие с машиночитаемыми доверенностями. Вы можете создать и зарегистрировать МЧД на любое физ. лицо от имени организации указанной при регистрации в приложении")]
    Loading,

    #[serde(rename = "Сохраните доверенность в формате doc и xml. Ознакомьтесь с текстом доверенности. Если все верно рукодитель организации должен подписать xml файл подписью руководителя отсоединенным файлом. Затем переходит в раздел отправки МЧД на регистрацию")]
    SaveXmlDocFiles,

    #[serde(rename = "Ознакомьтесь с вашими полномочиями")]
    ShowPowers,

    #[serde(rename = "Доверенность успешно зарегистрирована. Вы можете проверить её статус через сервис МЧД по указанному ниже номеру. Для этого сохраните этот номер. Для работы в системе данные номер сохранять нет необходимости, он подтягивается автоматически")]
    SuccessRegisterMchd,

    #[serde(rename = "Произошла ошибка при загрузке данных, повторите попытку позже...")]
    Failed,

    #[serde(rename = "Критическая ошибка на устройстве...")]
    ClientServiceError,

    #[serde(rename = "Критическая ошибка на серверной части приложения...")]
    BackApiError,

    #[serde(rename = "Введенные данные не соответствуют данным зарегистрированного пользователя")]
    WrongPerson,

    #[serde(rename = "Ошибка при запросе в интернет, проверьте подключение к сети")]
    ClientApiQueryError,

    #[serde(rename = "")]
    Nothing,
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewMchdData {
    pub poa_number: String1_50,
    pub poa_end_date: Date,
    pub tax_org_ident: Digits4_4,
 
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
    pub user_passport_issue_date: Date,
    pub user_passport_issueer: String1_4000,
    pub user_passport_ussuer_code: String7_7,
    pub user_is_citizen: IsCitizen,
    
    pub mchd_type: MchdType,
    pub powers: std::collections::HashSet<HomeMchdPower>

}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterMchdData {
    pub xml_file: Vec<u8>,
    pub sig_file: Vec<u8>,
    pub user_id: BoxUuid
}


#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub enum MchdType {
    BTBMchd,
    FnsMchd,
    HomeMchd
}

#[derive(Serialize, Deserialize)]
pub struct MchdStorage {
    pub storage: std::collections::HashMap<BoxUuid, PoaMchd>
}