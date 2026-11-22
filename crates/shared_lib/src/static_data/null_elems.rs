use serde::{Serialize, Deserialize};

use std::collections::HashSet;
use std::sync::LazyLock;
use chrono::{NaiveDate};
use rust_decimal::Decimal;


use crate::sql_models::id_rules::IdRules;
use crate::sql_models::person::implements::Person;
use crate::sql_models::company::implements::Company;
use crate::sql_models::user::implements::{UserSetData};
use crate::sql_models::contracts::implements::Contract;
use crate::sql_models::operation::implements::Operation;
use crate::sql_models::operation::account::{Account};
use crate::sql_models::operation::implements::DocType;
use crate::service::parsers::dadata::implements::CtrprtyMetadata;
use crate::sql_models::person::implements::PersonMetadata;
use crate::primitives::composite::implements::Fio;
use crate::service::parsers::dadata::implements::DadaCompName;
use crate::primitives::frozen::text_base::String1_1000;
use crate::primitives::frozen::text::{BoxUuid, CompInn, CompStatus, CompType, Currency, Date, DateTime, DocNum, Email, FirstName, Integ, Kpp, PersInn, Phone, RubF, Snils, SurName};


#[derive(Clone, ts_rs::TS, Serialize, Deserialize)]
pub struct NullPrimitives {
    pub pers_inn: PersInn,
    pub comp_inn: CompInn,
    pub kpp: Kpp,
    pub date: Date,
    pub snils: Snils,
    pub amount: RubF,
    pub doc_num: DocNum,
    pub string: String,
	pub contr_tittle: String,
    pub currency: Currency,
    pub integer: Integ,
    pub acc: Account,
    pub doc_type: DocType,
    pub phone: Phone,
    pub email: Email,
	pub last_update: DateTime,
	pub ip_kpp: Kpp
}

#[derive(Clone, ts_rs::TS, Serialize, Deserialize)]
pub struct NullIds {
    pub pers_id: BoxUuid,
    pub comp_id: BoxUuid,
    pub user_id: BoxUuid,
    pub contract_id: BoxUuid,
    pub operation_id: BoxUuid,
}

#[derive(Clone, ts_rs::TS, Serialize, Deserialize)]
pub struct NullModels {
    pub person: Person,
    pub company: Company,
    pub user_data: UserSetData,
    pub contract: Contract,
    pub operation: Operation,
}


#[derive(Clone, ts_rs::TS, Serialize, Deserialize)]
pub struct NullRegistry {
    pub p: NullPrimitives,
    pub id: NullIds,
    pub model: NullModels,
}

pub static NULLS: LazyLock<NullRegistry> = LazyLock::new(|| {
    let p = NullPrimitives {
        pers_inn: PersInn::unchecked("202000000000".to_string()),
        comp_inn: CompInn::unchecked("0010000000".to_string()),
        kpp: Kpp::unchecked("000000000".to_string()),
        date: Date::unchecked(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()),
        snils: Snils::unchecked("00000000000".to_string()),
        amount: RubF::unchecked(Decimal::ZERO),
        doc_num: DocNum::unchecked("0".to_string()),
        string: "0".to_string(),
		contr_tittle: "б/д".to_string(),
        currency: Currency::RUB,
        integer: Integ::unchecked(0),
        acc: Account::BankAcc,
        doc_type: DocType::Other,
        phone: Phone::unchecked("70000000000".to_string()),
        email: Email::unchecked("null@null.nul".to_string()),
		last_update: DateTime::unchecked(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc(),),
		ip_kpp: Kpp::unchecked("0".to_string()),
    };

    let pers_id = IdRules::Person {
        pers_inn: p.pers_inn.clone(),
        snils: p.snils.clone(),
    }.make_stat_id();

    let comp_id = IdRules::Company {
        comp_inn: p.comp_inn.clone(),
        kpp: p.kpp.clone(),
    }.make_stat_id();

    let user_id = IdRules::User {
        comp_id: comp_id.clone(),
        pers_id: pers_id.clone(),
    }.make_stat_id();

    let contract_id = IdRules::Contract {
        comp_id: comp_id.clone(),
        ctrpty_id: comp_id.clone(),
        contract_num: p.doc_num.clone(),
        contrac_date: p.date.clone(),
    }.make_stat_id();



    let operation_id = IdRules::Operation {
		comp_id: Some(comp_id.clone()),
		ctrpty_id: Some(comp_id.clone()),
		debet: Some(p.acc.clone()),
		credit: Some(p.acc.clone()),
        doc_num: Some(p.doc_num.clone()),
        oper_date: Some(p.date.clone()),
        amount: Some(p.amount.clone()),
        
    }.make_stat_id();

    let id = NullIds {
        pers_id,
        comp_id,
        user_id,
        contract_id,
        operation_id,
    };

    let person = Person {
        pers_id: id.pers_id.clone(),
        pers_inn: p.pers_inn.clone(),
        metadata: PersonMetadata {
            snils: p.snils.clone(),
            fio: Fio {
                sur_name: SurName::unchecked("nullname".to_string()),
                first_name: FirstName::unchecked("nullfirstname".to_string()),
                mid_name: None,
            },
            passport: None,
            address: None,
            gender: None,
            birth_day: None,
            phone: HashSet::new(),
            email: HashSet::new(),
        },
        last_update: p.last_update.clone(),
    };

    let company = Company {
        comp_id: id.comp_id.clone(),
        comp_inn: p.comp_inn.clone(),
        kpp: p.kpp.clone(),
        comp_type: CompType::ComEnt,
        comp_status: CompStatus::Liquidated,
        metadata: CtrprtyMetadata {
            kpp: None,
            main_manager: None,
            branch_type: None,
            kind: None,
            is_active: None,
            opf: None,
            comp_name: Some(DadaCompName{
				full_egrul_name: Some(String1_1000::unchecked("Контрагент отстуствует")),
				short_egrul_name: Some(String1_1000::unchecked("Контрагент отстуствует"))
			}),
            inn: None,
            ogrn: None,
            oktmo_company: None,
            okogu: None,
            okfs: None,
            okved: None,
            ogrn_date_dadata: None,
            address: None,
            e_mail: None,
            phone: None,
            bank_acc: vec![],
            ogrn_date_date: None,
            okpo: None,
        },
        last_update: p.last_update.clone(),
    };

    let user_data = UserSetData {
        user_id: id.user_id.clone(),
        pers_id: id.pers_id.clone(),
        comp_id: id.comp_id.clone(),
        phone: p.phone.clone(),
        password_hash: "".to_string(),
        email: p.email.clone(),
        guids: HashSet::new(),
    };

    let contract = Contract {
        contract_id: id.contract_id.clone(),
        comp_id: id.comp_id.clone(),
        user_id: id.user_id.clone(),
        ctrpty_id: id.comp_id.clone(),
        contract_num: p.doc_num.clone(),
        contract_date: p.date.clone(),
        title: p.contr_tittle.clone(),
        st_date: p.date.clone(),
        end_date: p.date.clone(),
        currency: p.currency.clone(),
        total_amount: p.amount.clone(),
        payment_deferral_days: p.integer.clone(),
        is_active: false,
        descrip: p.contr_tittle.clone(),
        entr_date: p.date.clone(),
        is_del: true,
    };

    let operation = Operation {
        oper_id: id.operation_id.clone(),
        user_id: id.user_id.clone(),
        comp_id: id.comp_id.clone(),
        ctrpty_id: id.comp_id.clone(),
        contract_id: id.contract_id.clone(),
        debet: p.acc.clone(),
        credit: p.acc.clone(),
        amount: p.amount.clone(),
        oper_date: p.date.clone(),
        doc_type: p.doc_type.clone(),
        doc_num: p.doc_num.clone(),
        doc_date: p.date.clone(),
        is_storno: false,
        is_del: true,
        entr_date: p.date.clone(),
    };

    let model = NullModels {
        person,
        company,
        user_data,
        contract,
        operation,
    };

    NullRegistry { p, id, model }
});
