use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::convert::Infallible;

use crate::Status;
use crate::primitives::frozen::implements::{BoxUuid, RubF, DateTime, Date};
use crate::sql_models::operation::account::Account;
use crate::primitives::traits::ParseFromStrMapValue;



#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ts_rs::TS)]
pub struct Operation {
    oper_id: BoxUuid,
    user_id: BoxUuid,

    comp_id: BoxUuid,
    ctrpty_id: BoxUuid,
    contract_id: Option<BoxUuid>,

    debet: Account,
    credit: Account,
    amount: RubF,
    oper_date: Date,

    doc_type: DocType,
    doc_num: String,
    doc_date: Date,

    is_storno: bool,
    is_del: bool,

    entr_date: DateTime,

    external_id: String,

    is_sync: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize,  ts_rs::TS)]
#[serde(rename_all = "snake_case")]
pub enum DocType {
    // --- Банковские документы ---
    BankOrder,         // Банковский ордер
    PaymentOrder,      // Платежное поручение
    PaymentClaim,      // Платежное требование
    CollectionOrder,   // Инкассовое поручение
    BankStatement,     // Банковская выписка

    // --- Кассовые документы ---
    CashReceipt,       // Приходный кассовый ордер (ПКО)
    CashVoucher,       // Расходный кассовый ордер (РКО)
    CashCheck,         // Кассовый чек / БСО

    // --- Товарные документы ---
    WaybillTorg12,     // Товарная накладная (ТОРГ-12)
    Upd,               // Универсальный передаточный документ (УПД)
    TransportWaybill,  // Транспортная накладная (ТН / ТТН)
    AcceptanceAct,     // Акт приема-передачи (ОС, имущества)

    // --- Услуги и работы ---
    ServiceAct,        // Акт оказанных услуг / выполненных работ

    // --- Налоговые и расчетные ---
    VatInvoice,        // Счет-фактура (СФ)
    PaymentInvoice,    // Счет на оплату
    ReconciliationAct, // Акт сверки взаиморасчетов

    // --- Внутренние и корректировочные ---
    AccountingNote,    // Бухгалтерская справка
    WriteOffAct,       // Акт списания
    CorrectionAct,     // Корректировочный акт / КСФ

    // --- Прочее ---
    Other,             // Иной документ
}

impl DocType {
    /// Основная логика парсинга: принимает любую строку и всегда возвращает DocType
    pub fn parse_str(s: &str) -> Self {
        let clean_str = s.trim().to_lowercase();

        match clean_str.as_str() {
            // --- Банковские ---
            "bank_order" | "bankorder" | "банковский ордер" | "мемориальный ордер" => Self::BankOrder,
            "payment_order" | "paymentorder" | "платежное поручение" | "платежка" | "пп" => Self::PaymentOrder,
            "payment_claim" | "платежное требование" => Self::PaymentClaim,
            "collection_order" | "инкассовое поручение" => Self::CollectionOrder,
            "bank_statement" | "банковская выписка" | "выписка банка" | "выписка" => Self::BankStatement,

            // --- Кассовые ---
            "cash_receipt" | "приходный кассовый ордер" | "пко" => Self::CashReceipt,
            "cash_voucher" | "расходный кассовый ордер" | "рко" => Self::CashVoucher,
            "cash_check" | "кассовый чек" | "чек" | "бсо" => Self::CashCheck,

            // --- Товарные ---
            "torg12" | "waybill_torg12" | "товарная накладная" | "торг-12" | "торг 12" | "накладная" => Self::WaybillTorg12,
            "upd" | "универсальный передаточный документ" | "упд" => Self::Upd,
            "transport_waybill" | "транспортная накладная" | "тн" | "ттн" => Self::TransportWaybill,
            "acceptance_act" | "акт приема-передачи" | "акт приема передачи" => Self::AcceptanceAct,

            // --- Услуги ---
            "service_act" | "акт оказанных услуг" | "акт выполненных работ" | "акт" => Self::ServiceAct,

            // --- Расчетные и Налоговые ---
            "vat_invoice" | "счет-фактура" | "счет фактура" | "сф" => Self::VatInvoice,
            "payment_invoice" | "invoice" | "счет на оплату" | "счет" => Self::PaymentInvoice,
            "reconciliation_act" | "акт сверки" | "акт сверки взаиморасчетов" => Self::ReconciliationAct,

            // --- Внутренние ---
            "accounting_note" | "бухгалтерская справка" | "справка" => Self::AccountingNote,
            "write_off_act" | "акт списания" | "списание" => Self::WriteOffAct,
            "correction_act" | "корректировочный акт" | "ксф" | "корректировочная счет-фактура" => Self::CorrectionAct,

            // --- Fallback ---
            _ => Self::Other,
        }
    }

    /// Человекочитаемое наименование на русском языке (для UI и печатных форм)
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::BankOrder => "Банковский ордер",
            Self::PaymentOrder => "Платежное поручение",
            Self::PaymentClaim => "Платежное требование",
            Self::CollectionOrder => "Инкассовое поручение",
            Self::BankStatement => "Банковская выписка",
            
            Self::CashReceipt => "Приходный кассовый ордер",
            Self::CashVoucher => "Расходный кассовый ордер",
            Self::CashCheck => "Кассовый чек",
            
            Self::WaybillTorg12 => "Товарная накладная (ТОРГ-12)",
            Self::Upd => "Универсальный передаточный документ (УПД)",
            Self::TransportWaybill => "Транспортная накладная",
            Self::AcceptanceAct => "Акт приема-передачи",
            
            Self::ServiceAct => "Акт оказанных услуг",
            
            Self::VatInvoice => "Счет-фактура",
            Self::PaymentInvoice => "Счет на оплату",
            Self::ReconciliationAct => "Акт сверки",
            
            Self::AccountingNote => "Бухгалтерская справка",
            Self::WriteOffAct => "Акт списания",
            Self::CorrectionAct => "Корректировочный акт",
            
            Self::Other => "Прочее",
        }
    }
}

// 1. Реализация стандартного FromStr через parse_str
impl FromStr for DocType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_str(s))
    }
}

// 2. Реализация From<&str> для удобной конвертации через DocType::from("...")
impl From<&str> for DocType {
    fn from(s: &str) -> Self {
        Self::parse_str(s)
    }
}

// 3. Форматирование через Display (выводит красиво на русском)
impl fmt::Display for DocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ParseFromStrMapValue for DocType {
    fn parse_from_str_map_value(value: Option<&&str>) -> Result<Self, Status> {
        match value {
            Some(s) => Ok(DocType::parse_str(s)),
            // Если ключа в мапе вообще не было — отдаем дефолтный Other (или возвращаем ошибку Status, если поле обязательное)
            None => Ok(DocType::Other), 
        }
    }
}