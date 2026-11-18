use std::collections::{HashSet, HashMap};

use crate::Status;
use crate::primitives::frozen::text::*;
use crate::static_data::null_elems::NULLS;
use crate::primitives::frozen::text_base::{PayType, CompanyName, IdentStatus};
use crate::sql_models::operation::implements::DocType;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct BlockFields {
	pub doc_type: DocType,
    pub doc_num: DocNum,
    pub doc_date: Date,
    pub statement_amount: RubF,

    pub pay_acc: RasAcc,
    pub pay_date: Option<Date>,
    pub pay_name: CompanyName,
    pub pay_inn: CompInn,
    pub pay_kpp: Kpp,
    pub pay_ras_acc: RasAcc,
    pub pay_bic: Bic,
    pub pay_corr_acc: CorAcc,

    pub rec_acc: RasAcc,
    pub rec_date: Option<Date>,
    pub rec_name: CompanyName,
    pub rec_inn: CompInn,
    pub rec_kpp: Kpp,
    pub rec_ras_acc: RasAcc,
    pub rec_bic: Bic,
    pub rec_corr_acc: CorAcc,

    pub doc_pay_type: PayType,
    pub doc_comment: TextInfo,
    pub doc_maker_status: Option<IdentStatus>
}


impl<'a> TryFrom<&HashMap<&'a str, &'a str>> for BlockFields {
    type Error = Status;

    fn try_from(block: &HashMap<&'a str, &'a str>) -> Result<Self, Self::Error> {
		let pay_inn = block.get("ПлательщикИНН").unwrap_or(&"").parse::<CompInn>().unwrap_or(NULLS.p.comp_inn.clone());
		let pay_kpp = if pay_inn == NULLS.p.comp_inn {
			NULLS.p.kpp.clone()
		} else {
			block.get("ПлательщикКПП").unwrap_or(&"").parse::<Kpp>()?
		};

		let rec_inn = block.get("ПолучательИНН").unwrap_or(&"").parse::<CompInn>().unwrap_or(NULLS.p.comp_inn.clone());
		let rec_kpp = if rec_inn == NULLS.p.comp_inn {
			NULLS.p.kpp.clone()
		} else {
			block.get("ПолучательКПП").unwrap_or(&"").parse::<Kpp>()?
		}; 

        Ok(Self {
            doc_type: block.get("окумент").unwrap_or(&"").parse::<DocType>()?,
            doc_num: block.get("Номер").unwrap_or(&"").parse::<DocNum>()?,
            doc_date: block.get("Дата").unwrap_or(&"").parse::<Date>()?,
            statement_amount: block.get("Сумма").unwrap_or(&"").parse::<RubF>()?,

            pay_acc: block.get("ПлательщикСчет").unwrap_or(&"").parse::<RasAcc>()?,
            pay_date: block
                .get("ДатаСписано")
                .copied()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<Date>())
                .transpose()?,
            pay_name: block.get("Плательщик").unwrap_or(&"").parse::<CompanyName>()?,
            pay_inn, 
            pay_kpp,
            pay_ras_acc: block.get("ПлательщикРасчСчет").unwrap_or(&"").parse::<RasAcc>()?,
            pay_bic: block.get("ПлательщикБИК").unwrap_or(&"").parse::<Bic>()?,
            pay_corr_acc: block.get("ПлательщикКорсчет").unwrap_or(&"").parse::<CorAcc>()?,

            rec_acc: block.get("ПолучательСчет").unwrap_or(&"").parse::<RasAcc>()?,
            rec_date: block
                .get("ДатаПоступило")
                .copied()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<Date>())
                .transpose()?,
            rec_name: block.get("Получатель").unwrap_or(&"").parse::<CompanyName>()?,
            rec_inn,
			rec_kpp, 
            rec_ras_acc: block.get("ПолучательРасчСчет").unwrap_or(&"").parse::<RasAcc>()?,
            rec_bic: block.get("ПолучательБИК").unwrap_or(&"").parse::<Bic>()?,
            rec_corr_acc: block.get("ПолучательКорсчет").unwrap_or(&"").parse::<CorAcc>()?,

            doc_pay_type: block.get("ВидОплаты").unwrap_or(&"").parse::<PayType>()?,
            doc_comment: block.get("НазначениеПлатежа").unwrap_or(&"").parse::<TextInfo>()?,
            doc_maker_status: block
                .get("СтатусСоставителя")
                .copied()
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<IdentStatus>())
                .transpose()?,
        })
    }
}


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
    pub is_tax: bool,
    pub is_cred_return: bool,
    pub is_cred_loan: bool,
    pub dates: Vec<Date>,
    pub doc_num: HashSet<DocNum>,
    pub is_period: bool,
    pub is_contract: bool,
    pub is_komis: bool,
    pub is_salary: bool,
    pub is_invoice: bool,
    pub is_penalty: bool,
	pub is_depos_persent: bool,
	pub is_founder: bool,
    pub nds_amount: Option<RubF>,
    pub nds_rate: u8,
	pub is_person: bool
}

#[derive(Debug, Clone)]
pub struct ParsedBlock {
    pub block_fields: BlockFields,
    pub comment_data: BlockCommentData
}

