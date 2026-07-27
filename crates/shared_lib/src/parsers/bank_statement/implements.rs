use std::collections::{HashSet, HashMap};

use crate::Status;
use crate::primitives::frozen::implements::*;
use crate::primitives::frozen::implements_base::{PayType, CompanyName, IdentStatus};
use crate::primitives::composite::implements::RasBicAcc;
use crate::sql_models::operation::implements::{DocType, Operation};

make_struct!(pub BlockFields, [
    (doc_type, DocType, "окумент"),
    (doc_num, DocNum, "Номер"),
    (doc_date, Date, "Дата"),
    (statement_amount, RubF, "Сумма"),

    (pay_acc, RasAcc, "ПлательщикСчет"),
    (pay_date, Option<Date>, "ДатаСписано"),
    (pay_name, CompanyName, "Плательщик"),
    (pay_inn, CompInn, "ПлательщикИНН"),
    (pay_kpp, Kpp, "ПлательщикКПП"),
    (pay_ras_acc, RasAcc, "ПлательщикРасчСчет"),
    (pay_bic, Bic, "ПлательщикБИК"),
    (pay_corr_acc, CorAcc, "ПлательщикКорсчет"),

    (rec_acc, RasAcc, "ПолучательСчет"),
    (rec_date, Option<Date>, "ДатаПоступило"),
    (rec_name, CompanyName, "Получатель"),
    (rec_inn, CompInn, "ПолучательИНН"),
    (rec_kpp, Kpp, "ПолучательКПП"),
    (rec_ras_acc, RasAcc, "ПолучательРасчСчет"),
    (rec_bic, Bic, "ПолучательБИК"),
    (rec_corr_acc, CorAcc, "ПолучательКорсчет"),

    (doc_pay_type, PayType, "ВидОплаты"),
    (doc_comment, TextInfo, "НазначениеПлатежа"),
    (doc_maker_status, Option<IdentStatus>, "СтатусСоставителя")
]);

make_struct!(pub StatementHead,[
    (start_date, Date, "ДатаНачала"),
    (end_date, Date, "ДатаКонца"),
    (head_acc, RasAcc, "РасчСчет"),
    (start_amount, Option<RubF>, "НачальныйОстаток"),
    (total_in, Option<RubF>, "ВсегоПоступило"),
    (total_out, Option<RubF>, "ВсегоСписано"),
    (end_amount, Option<RubF>, "КонечныйОстаток"),
]);

#[derive(Debug, Clone, Default)]
pub struct BlockCommentData {
    pub is_own_operation: bool,
    pub is_tax: bool,
    pub dates: Vec<Date>,
    pub doc_num: Option<DocNum>,
    pub is_period: bool,
    pub is_contract: bool,
    pub is_komis: bool,
    pub is_salary: bool,
    pub is_invoice: bool,
    pub is_credit: bool,
    pub is_penalty: bool,
    pub nds_amount: Option<RubF>,
    pub nds_rate: u8,
    pub errors: HashSet<Status>
}

#[derive(Debug, Clone)]
pub struct ParsedBlock {
    pub block_fields: BlockFields,
    pub comment_data: BlockCommentData
}

pub struct StatementParseResult {
    pub text: String,
    pub operations: Vec<Operation>
}


pub type InnKppMapAcc = HashMap<(CompInn, Kpp), HashSet<RasBicAcc>>;
