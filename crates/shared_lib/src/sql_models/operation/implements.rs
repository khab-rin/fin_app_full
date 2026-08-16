use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::convert::Infallible;

use crate::Status;
use crate::sql_models::operation::macros::ParseFromStrMapValue;
use crate::primitives::frozen::text::{BoxUuid, Date, DateTime, DocNum, RubF, TextInfo};
use crate::sql_models::operation::account::Account;
use crate::sql_models::company::implements::Company;
use crate::sql_models::contracts::implements::Contract;
use crate::sql_models::operation::service::{
    OperationInfo, OperationStep
};


#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
pub struct OperationRaw {
    pub oper_id: BoxUuid,
    pub user_id: BoxUuid,
    pub comp_id: BoxUuid,

    pub ctrpty: Option<Company>,

    pub contract: ContractOption,

    pub debet: Account,
    pub credit: Account,
    pub amount: RubF,
    pub oper_date: Option<Date>,

    pub doc_type: DocType,
    pub doc_num: DocNum,
    pub doc_date: Date,

    pub is_storno: bool,
    pub is_del: bool,

    pub entr_date: Date,

    pub is_sync: Option<bool>,

    pub comment: TextInfo,

    pub is_duplicate: bool
}

#[derive(Debug, Clone, Serialize, Deserialize, ts_rs::TS)]
pub struct ContractOption {
    pub current: Option<Contract>,
    pub contracts: Vec<Contract>
}


#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ts_rs::TS)]
pub struct Operation {
    pub oper_id: BoxUuid,
    pub user_id: BoxUuid,

    pub comp_id: BoxUuid,
    pub ctrpty_id: BoxUuid,
    pub contract_id: Option<BoxUuid>,

    pub debet: Account,
    pub credit: Account,
    pub amount: RubF,
    pub oper_date: Date,

    pub doc_type: DocType,
    pub doc_num: DocNum,
    pub doc_date: Date,

    pub is_storno: bool,
    pub is_del: bool,

    pub entr_date: DateTime,
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
            "bank_order" | "bankorder" | "банковский ордер" | "мемориальный ордер" => Self::BankOrder,
            "payment_order" | "paymentorder" | "платежное поручение" | "платежка" | "пп" => Self::PaymentOrder,
            "payment_claim" | "платежное требование" => Self::PaymentClaim,
            "collection_order" | "инкассовое поручение" => Self::CollectionOrder,
            "bank_statement" | "банковская выписка" | "выписка банка" | "выписка" => Self::BankStatement,

            "cash_receipt" | "приходный кассовый ордер" | "пко" => Self::CashReceipt,
            "cash_voucher" | "расходный кассовый ордер" | "рко" => Self::CashVoucher,
            "cash_check" | "кассовый чек" | "чек" | "бсо" => Self::CashCheck,

            "torg12" | "waybill_torg12" | "товарная накладная" | "торг-12" | "торг 12" | "накладная" => Self::WaybillTorg12,
            "upd" | "универсальный передаточный документ" | "упд" => Self::Upd,
            "transport_waybill" | "транспортная накладная" | "тн" | "ттн" => Self::TransportWaybill,
            "acceptance_act" | "акт приема-передачи" | "акт приема передачи" => Self::AcceptanceAct,

            "service_act" | "акт оказанных услуг" | "акт выполненных работ" | "акт" => Self::ServiceAct,

            "vat_invoice" | "счет-фактура" | "счет фактура" | "сф" => Self::VatInvoice,
            "payment_invoice" | "invoice" | "счет на оплату" | "счет" => Self::PaymentInvoice,
            "reconciliation_act" | "акт сверки" | "акт сверки взаиморасчетов" => Self::ReconciliationAct,


            "accounting_note" | "бухгалтерская справка" | "справка" => Self::AccountingNote,
            "write_off_act" | "акт списания" | "списание" => Self::WriteOffAct,
            "correction_act" | "корректировочный акт" | "ксф" | "корректировочная счет-фактура" => Self::CorrectionAct,

            _ => Self::Other,
        }
    }

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

impl FromStr for DocType {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::parse_str(s))
    }
}

impl From<&str> for DocType {
    fn from(s: &str) -> Self {
        Self::parse_str(s)
    }
}

impl fmt::Display for DocType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ParseFromStrMapValue for DocType {
    fn parse_from_str_map_value(value: Option<&&str>) -> Result<Self, Status> {
        match value {
            Some(s) => Ok(DocType::parse_str(s)),
            None => Ok(DocType::Other), 
        }
    }
}

#[derive(Serialize, Deserialize, Debug, ts_rs::TS)]
pub struct OperationDocument {
    pub doc_type: DocType,
    pub doc_num: DocNum,
    pub doc_data: Date
}

pub fn make_oper_id(doc_num: &DocNum, doc_date: &Date, amount: &RubF, ctrpty: &Option<Company>) -> BoxUuid {
    let text_id = if let Some(comp) = ctrpty {
        format!("{}-{}-{}-{}", doc_num.as_ref(), doc_date.as_ref(), amount.as_ref(), comp.comp_id.as_ref())
    } else {
        format!("{}-{}-{}", doc_num.as_ref(), doc_date.as_ref(), amount.as_ref())
    };

    BoxUuid::unchecked(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, text_id.as_bytes()))
}


#[derive(ts_rs::TS)]
pub struct OperationTSTS {
    operation_raw: OperationRaw,
    operation: Operation,
    contract_option: ContractOption,
    doc_type: DocType,
    account: Account,
    oper_step: OperationStep,
    oper_inf: OperationInfo
}