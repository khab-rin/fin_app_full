
use serde::{Serialize, Deserialize};
use crate::Status;
use crate::primitives::frozen::formatters::*;
use crate::primitives::traits::ParseFromStrMapValue;
use crate::primitives::frozen::validator_base::*;


frozen_primitives!(
    CompanyName,
    init_str_1_1000,
    Box<str>,
    "Имя_контрагента",
    uppercase_fmt);

frozen_primitives!(
    IdentStatus,
    init_digits_2_2,
    Box<str>,
    "Идентификационный_статус",
    default_fmt);   


frozen_primitives!(
    PayType,
    init_digits_2_2,
    Box<str>,
    "Тип_платежа",
    default_fmt);


frozen_primitives!(
    String7_7,
    init_str_7_7,
    Box<str>,
    "Строка_7_7_символов",
    default_fmt);

frozen_primitives!(
    String3_8,
    init_str_3_8,
    Box<str>,
    "Строка_3_8_символов",
    default_fmt);

frozen_primitives!(
    String11_11,
    init_str_11_11,
    Box<str>,
    "Строка_11_символов",
    default_fmt);

frozen_primitives!(
    String3_13,
    init_str_3_13,
    Box<str>,
    "Строка_3_13_символов",
    default_fmt);

frozen_primitives!(
    String1_25,
    init_str_1_25,
    Box<str>,
    "Строка_1_25_символов",
    default_fmt);

frozen_primitives!(
    String1_28,
    init_str_1_28,
    Box<str>,
    "Строка_1_28_символов",
    default_fmt);

frozen_primitives!(
    String1_50,
    init_str_1_50,
    Box<str>,
    "Строка_1_50_символов",
    default_fmt);


frozen_primitives!( 
    String1_80,
    init_str_1_80,
    Box<str>,
    "Текст_1_80_символов",
    default_fmt);

frozen_primitives!( 
    String1_120,
    init_str_1_120,
    Box<str>,
    "Текст_1_120_символов",
    default_fmt);

frozen_primitives!( 
    String3_129,
    init_str_3_129,
    Box<str>,
    "Текст_3_129_символов",
    default_fmt);

frozen_primitives!( 
    String1_250,
    init_str_1_250,
    Box<str>,
    "Текст_1_250_символов",
    default_fmt);

frozen_primitives!( 
    String1_255,
    init_str_1_255,
    Box<str>,
    "Текст_1_255_символов",
    uppercase_fmt);

frozen_primitives!( 
    String6_255,
    init_str_6_255,
    Box<str>,
    "Текст_6_255_символов",
    default_fmt);


frozen_primitives!( 
    String1_1000,
    init_str_1_1000,
    Box<str>,
    "Текст_1_1000_символов",
    default_fmt);

frozen_primitives!( 
    String1_2500,
    init_str_1_2500,
    Box<str>,
    "Текст_1_2500_символов",
    default_fmt);

frozen_primitives!( 
    String1_4000,
    init_str_1_4000,
    Box<str>,
    "Текст_1_4000_символов",
    default_fmt);


frozen_primitives!( 
    String1_5000,
    init_str_1_5000,
    Box<str>,
    "Текст_1_5000_символов",
    default_fmt);


frozen_primitives!(
    String1_10000,
    init_str_1_10000,
    Box<str>,
    "Хэш_до_10000_символов",
    default_fmt);

frozen_primitives!(
    String1_16000,
    init_str_1_16000,
    Box<str>,
    "Хэш_до_16000_символов",
    default_fmt);


frozen_primitives!(
    Digits2_2,
    init_digits_2_2,
    Box<str>,
    "Строка_2_цифры",
    default_fmt);

frozen_primitives!(
    Digits3_3,
    init_digits_3_3,
    Box<str>,
    "Строка_3_цифры",
    default_fmt);

frozen_primitives!(
    Digits4_4,
    init_digits_4_4,
    Box<str>,
    "Строка_4_цифры",
    default_fmt);

frozen_primitives!(
    Digits7_7,
    init_digits_7_7,
    Box<str>,
    "Строка_7_цифр",
    default_fmt);

frozen_primitives!(
    Digits12_12,
    init_digits_12_12,
    Box<str>,
    "Строка_12_цифр",
    default_fmt);

frozen_primitives!(
    PasspRfNumber,
    init_digits_10_10,
    Box<str>,
    "Номер_пассморта_РФ",
    passport_fmt);





 





