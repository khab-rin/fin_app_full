use serde::{Deserialize, Serialize};

use crate::{Status, make_enum_frozen};
use crate::sql_models::operation::macros::ParseFromStrMapValue;
use crate::primitives::frozen::text::{BoxUuid, Date, DocNum, RubF, TextInfo};
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

    pub contract_id: BoxUuid,

    pub debet: Account,
    pub credit: Account,
    pub amount: RubF,
    pub oper_date: Date,

    pub doc_type: DocType,
    pub doc_num: DocNum,
    pub doc_date: Date,

    pub is_storno: bool,
    pub is_del: bool,

    pub entr_date: Date,
}



make_enum_frozen! {
    DocType, {
        BankOrder, "банковский ордер", {
            "BANK_ORDER", "бо", "bo"
        },
        PaymentOrder, "платежное поручение", {
            "PAYMENT_ORDER", "пп", "po"
        },
        PaymentClaim, "платежное требование", {
            "PAYMENT_CLAIM", "пт", "pc"
        },
        CollectionOrder, "инкассовое поручение", {
            "COLLECTION_ORDER", "ип", "co"
        },
        BankStatement, "банковская выписка", {
            "BANK_STATEMENT", "бв", "bs"
        },
        CashReceipt, "приходный кассовый ордер", {
            "CASH_RECEIPT", "пко", "cr"
        },
        CashVoucher, "расходный кассовый ордер", {
            "CASH_VOUCHER", "рко", "cv"
        },
        CashCheck, "кассовый чек", {
            "CASH_CHECK", "кч", "cc"
        },
        WaybillTorg12, "товарная накладная", {
            "WAYBILL_TORG12", "тн", "wt"
        },
        Upd, "универсальный передаточный документ", {
            "UPD", "упд", "upd"
        },
        TransportWaybill, "транспортная накладная", {
            "TRANSPORT_WAYBILL", "ттр", "tw"
        },
        AcceptanceAct, "акт приема-передачи", {
            "ACCEPTANCE_ACT", "ап", "aa"
        },
        ServiceAct, "акт оказанных услуг", {
            "SERVICE_ACT", "ау", "sa"
        },
        VatInvoice, "счет-фактура", {
            "VAT_INVOICE", "сф", "vi"
        },
        PaymentInvoice, "счет на оплату", {
            "PAYMENT_INVOICE", "со", "pi"
        },
        ReconciliationAct, "акт сверки", {
            "RECONCILIATION_ACT", "ас", "ra"
        },
        AccountingNote, "бухгалтерская справка", {
            "ACCOUNTING_NOTE", "бс", "an"
        },
        WriteOffAct, "акт списания", {
            "WRITE_OFF_ACT", "асн", "wa"
        },
        CorrectionAct, "корректировочный акт", {
            "CORRECTION_ACT", "ка", "ca"
        },
        Other, "прочее", {
            "OTHER", "пр", "ot"
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

pub fn make_oper_id(
    doc_num: &Option<DocNum>, 
    oper_date: &Option<Date>, 
    amount: &Option<RubF>, 
    ctrpty_id: &Option<BoxUuid>
) -> BoxUuid {
    
    let num_str = doc_num.as_ref().map_or("".to_string(), |v| v.to_string());
    let date_str = oper_date.as_ref().map_or("".to_string(), |v| v.to_string());
    let amount_str = amount.as_ref().map_or("".to_string(), |v| {
        let s = v.to_string();
        if s.contains('.') {
            let trimmed = s.trim_end_matches('0');
            if trimmed.ends_with('.') {
                // Если после удаления нулей осталась только точка (например, 12.0 -> 12.)
                trimmed.trim_end_matches('.').to_string()
            } else {
                trimmed.to_string()
            }
        } else {
            s
        }
    });
    let ctrpty_str = ctrpty_id.as_ref().map_or("".to_string(), |v| v.to_string());

    // 2. Теперь макрос format! спокойно принимает эти строки, так как они "живы"
    let text_id = format!(
        "{}-{}-{}-{}",
        num_str,
        date_str,
        amount_str,
        ctrpty_str
    );

    BoxUuid::unchecked(uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, text_id.as_bytes()))

}


#[allow(dead_code)]
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