use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::convert::Infallible;

use crate::{Status, make_enum_frozen};
use crate::sql_models::operation::macros::ParseFromStrMapValue;
use crate::primitives::frozen::text::{BoxUuid, Date, DateTime, DocNum, RubF, TextInfo};
use crate::sql_models::operation::account::Account;
use crate::sql_models::company::implements::Company;
use crate::sql_models::contracts::implements::{Contract, NewContrData};
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



make_enum_frozen! {
    DocType, {
        BankOrder, "банковский ордер", {
            "BANK_ORDER", "bank_order", "bankorder", "мемориальный ордер"
        },
        PaymentOrder, "платежное поручение", {
            "PAYMENT_ORDER", "payment_order", "paymentorder", "платежка", "пп"
        },
        PaymentClaim, "платежное требование", {
            "PAYMENT_CLAIM", "payment_claim"
        },
        CollectionOrder, "инкассовое поручение", {
            "COLLECTION_ORDER", "collection_order"
        },
        BankStatement, "банковская выписка", {
            "BANK_STATEMENT", "bank_statement", "выписка банка", "выписка"
        },
        CashReceipt, "приходный кассовый ордер", {
            "CASH_RECEIPT", "cash_receipt", "пко"
        },
        CashVoucher, "расходный кассовый ордер", {
            "CASH_VOUCHER", "cash_voucher", "рко"
        },
        CashCheck, "кассовый чек", {
            "CASH_CHECK", "cash_check", "чек", "бсо"
        },
        WaybillTorg12, "товарная накладная", {
            "WAYBILL_TORG12", "torg12", "waybill_torg12", "торг-12", "торг 12", "накладная"
        },
        Upd, "универсальный передаточный документ", {
            "UPD", "upd", "упд"
        },
        TransportWaybill, "транспортная накладная", {
            "TRANSPORT_WAYBILL", "transport_waybill", "тн", "ттн"
        },
        AcceptanceAct, "акт приема-передачи", {
            "ACCEPTANCE_ACT", "acceptance_act", "акт приема передачи"
        },
        ServiceAct, "акт оказанных услуг", {
            "SERVICE_ACT", "service_act", "акт выполненных работ", "акт"
        },
        VatInvoice, "счет-фактура", {
            "VAT_INVOICE", "vat_invoice", "счет фактура", "сф"
        },
        PaymentInvoice, "счет на оплату", {
            "PAYMENT_INVOICE", "payment_invoice", "invoice", "счет"
        },
        ReconciliationAct, "акт сверки", {
            "RECONCILIATION_ACT", "reconciliation_act", "акт сверки взаиморасчетов"
        },
        AccountingNote, "бухгалтерская справка", {
            "ACCOUNTING_NOTE", "accounting_note", "справка"
        },
        WriteOffAct, "акт sписания", {
            "WRITE_OFF_ACT", "write_off_act", "списание"
        },
        CorrectionAct, "корректировочный акт", {
            "CORRECTION_ACT", "correction_act", "ксф", "корректировочная счет-фактура"
        },
        Other, "прочее", {
            "OTHER", "other"
        },
    }
}




impl ParseFromStrMapValue for DocType {
    fn parse_from_str_map_value(value: Option<&&str>) -> Result<Self, Status> {
        match value {
            Some(s) => {
                let clean = s.trim().to_lowercase();
                Ok(clean.parse::<DocType>().unwrap_or(DocType::Other))
            },
            None => Ok(DocType::Other)
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
    oper_inf: OperationInfo,
    new_contract_data: NewContrData
}