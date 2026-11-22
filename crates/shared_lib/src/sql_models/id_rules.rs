use crate::primitives::frozen::text::{
    BoxUuid, CompInn, Date, DocNum, Kpp, PersInn, RubF, Snils,
};
use crate::sql_models::operation::account::Account;

pub enum IdRules {
    Company { comp_inn: CompInn, kpp: Kpp },
	Contract {
        comp_id: BoxUuid,
        ctrpty_id: BoxUuid,
        contract_num: DocNum,
        contrac_date: Date,
    },
	Device { app_str: String },
	Operation {
		comp_id: Option<BoxUuid>,
		ctrpty_id: Option<BoxUuid>,
		debet: Option<Account>,
		credit: Option<Account>,
        oper_date: Option<Date>,
        amount: Option<RubF>,
		doc_num: Option<DocNum>,
    },
    Person { pers_inn: PersInn, snils: Snils },

    User { comp_id: BoxUuid, pers_id: BoxUuid },
    
    #[cfg(feature = "server")]
    Session { user_id: BoxUuid, device_id: BoxUuid },
	
}

impl IdRules {
    pub fn make_stat_id(&self) -> BoxUuid {
        let t = match self {
            Self::Company { comp_inn, kpp } => {
                format!("{}-{}", comp_inn, kpp)
            },

			Self::Contract {
                comp_id,
                ctrpty_id,
                contract_num,
                contrac_date,
            } => {
                format!(
                    "{}-{}-{}-{}",
                    comp_id,
                    ctrpty_id,
                    contract_num,
                    contrac_date,
                )
            },

			Self::Device { app_str } => {
				app_str.to_string()
			},

			Self::Operation {
				comp_id,
				ctrpty_id,
				debet,
				credit,
				oper_date,
				amount,
				doc_num,
            } => {
                format!(
                    "{}-{}-{}-{}-{}-{}-{}",
					comp_id.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
					ctrpty_id.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
					debet.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
					credit.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
					oper_date.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
                    amount.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
                    doc_num.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),        
                )
            },

            Self::Person { pers_inn, snils } => {
                format!("{}-{}", pers_inn, snils)
            },

            Self::User { comp_id, pers_id } => {
                format!("{}-{}-{}", comp_id, pers_id, "user-pepper")
            },

			#[cfg(feature="server")]
            Self::Session { user_id, device_id } => {
                format!("{}-{}-{}", user_id, device_id, "session-pepper")
            },
        };

        BoxUuid::unchecked(
            uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, t.as_bytes())
        )
    }
}