use chrono::{NaiveDate};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use sqlx;

use crate::primitives::frozen::validator_rules::*;
use crate::Status;
use crate::primitives::traits::ParseFromStrMapValue;

frozen_primitives!(
    PersInn,
    init_pers_inn_from_str,
    Box<str>,
    "ИНН_физлица");

frozen_primitives!(
    CompInn,
    init_comp_inn_from_str,
    Box<str>,
    "ИНН_юлица");

frozen_primitives!(
    Kpp,
    init_kpp_from_str,
    Box<str>,
    "КПП");

impl_is_zero!(Kpp);


frozen_primitives!(
    CorAcc,
    init_cor_ras_acc_from_str,
    Box<str>,
    "КорСчет");

frozen_primitives!(
    RasAcc,
    init_cor_ras_acc_from_str,
    Box<str>,
    "РасСчет");

frozen_primitives!(
    Bic,
    init_bic_from_str,
    Box<str>,
    "БИК");

frozen_primitives!(
    Ogrn,
    init_ogrn_from_str,
    Box<str>,
    "ОГРН");

frozen_primitives!(
    Date,
    str_to_date,
    NaiveDate,
    "Дата");

frozen_primitives!(
    RubF,
    init_rubf_from_str,
    Decimal,
    "Руб.");

frozen_primitives!(
    DocNum,
    init_doc_num_from_str,
    Box<str>,
    "Номер_документа");

frozen_primitives!(
    TextInfo,
    init_text_info_from_str,
    Box<str>,
    "Текстовая_информация");

frozen_primitives!(
    BranchType,
    init_branch_type_from_str,
    Box<str>,
    "Статус_филиала");

frozen_primitives!(
    Okpo,
    init_okpo_from_str,
    Box<str>,
    "ОКПО");

frozen_primitives!(
    Oktmo,
    init_oktmo_from_str,
    Box<str>,
    "ОКТМО");

frozen_primitives!(
    Okogu,
    init_okogu_from_str,
    Box<str>,
    "ОКОГУ");

frozen_primitives!(
    Okfs,
    init_okfs_from_str,
    Box<str>,
    "ОКФС");

frozen_primitives!(
    Okved,
    init_okved_from_str,
    Box<str>,
    "ОКВЭД");

frozen_primitives!(
    Phone, 
    init_phone_from_str,
    Box<str>,
    "Телефон");

frozen_primitives!(
    OpfCode, 
    init_opf_code_from_str,
    Box<str>,
    "ОКОПФ");

frozen_primitives!(
    SurName, 
    init_fio,
    Box<str>,
    "ФамилияФЛ");

frozen_primitives!(
    FirstName, 
    init_fio,
    Box<str>,
    "ИмяФЛ");

frozen_primitives!(
    MidName, 
    init_fio,
    Box<str>,
    "ОтчествоФЛ");


frozen_primitives!(
    Region,
    init_region,
    Box<str>,
    "Код_региона"
);

frozen_primitives!(
    Snils,
    init_snils_from_str,
    Box<str>,
    "СНИЛС"
);

frozen_primitives!(
    BoxUuid,
    init_uuid_from_str,
    uuid::Uuid,
    "Фиас_код_адреса"
);

frozen_primitives!(
    DateTime,
    init_date_time_from_str,
    chrono::DateTime<chrono::Utc>,
    "Дата_Время"
);

frozen_primitives!(
    Email,
    init_email_from_str,
    Box<str>,
    "Email"
);

make_enum_frozen! {
    CompStatus, {
        Active, "ACTIVE", {"120"},
        Liquidating, "LIQUIDATING", {"121"},
        Liquidated, "LIQUIDATED", {"122"},
        Reorganizing, "REORGANIZING", {"123"},
        Bankrupt, "BANKRUPT", {"124"}
    }
}

make_enum_frozen! {
    CompType, {
        ComEnt, "COM_ENT", {"LEGAL"},
        Ip, "IP", {"INDIVIDUAL"},
        Gov, "GOV", {},
        Bank, "BANK", {}
    }
}

frozen_primitives!(
    ParticipantStatus,
    init_part_status,
    Box<str>,
    "3_значный_код_статуса"
);

frozen_primitives!(
    PoaReqElemsFlag,
    init_flag_str,
    Box<str>,
    "Флаги_обязательности_элементов"
);

frozen_primitives!(
    Password,
    init_password_from_str,
    Box<str>,
    "Поле_пароля"
);