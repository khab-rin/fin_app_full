use chrono::{NaiveDate};
use rust_decimal::Decimal;
use serde::{Serialize, Deserialize};
use sqlx;

use crate::primitives::frozen::validators::*;
use crate::primitives::frozen::formatters::*;
use crate::Status;
use crate::sql_models::operation::macros::ParseFromStrMapValue;

frozen_primitives!(
    PersInn,
    init_pers_inn_from_str,
    String,
    "ИНН_физлица",
    default_fmt);

frozen_primitives!(
    CompInn,
    init_comp_inn_from_str,
    String,
    "ИНН_юлица",
    default_fmt);

frozen_primitives!(
    Kpp,
    init_kpp_from_str,
    String,
    "КПП",
    default_fmt);

impl_is_zero!(Kpp);


frozen_primitives!(
    CorAcc,
    init_cor_ras_acc_from_str,
    String,
    "КорСчет",
    default_fmt);

frozen_primitives!(
    RasAcc,
    init_cor_ras_acc_from_str,
    String,
    "РасСчет",
    default_fmt);

frozen_primitives!(
    Bic,
    init_bic_from_str,
    String,
    "БИК",
    default_fmt);

frozen_primitives!(
    Ogrn,
    init_ogrn_from_str,
    String,
    "ОГРН",
    default_fmt);

frozen_primitives!(
    Date,
    str_to_date,
    NaiveDate,
    "Дата",
    default_fmt);

frozen_primitives!(
    RubF,
    init_rubf_from_str,
    Decimal,
    "Руб.",
    default_fmt);

frozen_primitives!(
    DocNum,
    init_doc_num_from_str,
    String,
    "Номер_документа",
    default_fmt);

frozen_primitives!(
    TextInfo,
    init_text_info_from_str,
    String,
    "Текстовая_информация",
    default_fmt);

frozen_primitives!(
    BranchType,
    init_branch_type_from_str,
    String,
    "Статус_филиала",
    default_fmt);

frozen_primitives!(
    Okpo,
    init_okpo_from_str,
    String,
    "ОКПО",
    default_fmt);

frozen_primitives!(
    Oktmo,
    init_oktmo_from_str,
    String,
    "ОКТМО",
    default_fmt);

frozen_primitives!(
    Okogu,
    init_okogu_from_str,
    String,
    "ОКОГУ",
    default_fmt);

frozen_primitives!(
    Okfs,
    init_okfs_from_str,
    String,
    "ОКФС",
    default_fmt);

frozen_primitives!(
    Okved,
    init_okved_from_str,
    String,
    "ОКВЭД",
    default_fmt);

frozen_primitives!(
    Phone, 
    init_phone_from_str,
    String,
    "Телефон",
    default_fmt);

frozen_primitives!(
    OpfCode, 
    init_opf_code_from_str,
    String,
    "ОКОПФ",
    default_fmt);

frozen_primitives!(
    SurName, 
    init_fio,
    String,
    "ФамилияФЛ",
    uppercase_fmt);

frozen_primitives!(
    FirstName, 
    init_fio,
    String,
    "ИмяФЛ",
    uppercase_fmt);

frozen_primitives!(
    MidName, 
    init_fio,
    String,
    "ОтчествоФЛ",
    uppercase_fmt);


frozen_primitives!(
    Region,
    init_region,
    String,
    "Код_региона",
    default_fmt);

frozen_primitives!(
    Snils,
    init_snils_from_str,
    String,
    "СНИЛС",
    snils_fmt);

frozen_primitives!(
    BoxUuid,
    init_boxuuid,
    uuid::Uuid,
    "Фиас_код_адреса",
    default_fmt);

frozen_primitives!(
    DateTime,
    init_date_time_from_str,
    chrono::DateTime<chrono::Utc>,
    "Дата_Время",
    default_fmt);

frozen_primitives!(
    Email,
    init_email_from_str,
    String,
    "Email",
    default_fmt);

frozen_primitives!(
    Password,
    init_password_from_str,
    String,
    "Поле_пароля",
    default_fmt);


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

make_enum_frozen! {
    Currency, {
        RUB, "РУБ", {"RUB", "rub", "руб"},
        EUR, "ЕВРО", {"EUR", "eur", "евро"},
        USD, "ДОЛЛ", {"USD", "usd", "долл"}
    }
}