use crate::{primitives::frozen::text::{
    BoxUuid, CompInn, Date, DocNum, Kpp, PersInn, RubF, Snils,
}, sql_models::id_rules::IdRules::Device};

pub enum IdRules {
    Company { comp_inn: CompInn, kpp: Kpp },
    Person { pers_inn: PersInn, snils: Snils },
    User { comp_id: BoxUuid, pers_id: BoxUuid },
    Operation {
        doc_num: Option<DocNum>,
        oper_date: Option<Date>,
        amount: Option<RubF>,
        ctrpty_id: Option<BoxUuid>,
    },
    Contract {
        comp_id: BoxUuid,
        ctrpty_id: BoxUuid,
        contract_num: DocNum,
        contrac_date: Date,
    },
    Session { user_id: BoxUuid, device_id: BoxUuid },
	Device { app_str: String }
}

impl IdRules {
    pub fn make_stat_id(&self) -> BoxUuid {
        let t = match self {
            Self::Company { comp_inn, kpp } => {
                format!("{}-{}", comp_inn, kpp)
            }

            Self::Person { pers_inn, snils } => {
                format!("{}-{}", pers_inn, snils)
            }

            Self::User { comp_id, pers_id } => {
                format!("{}-{}", comp_id, pers_id)
            }

            Self::Operation {
                doc_num,
                oper_date,
                amount,
                ctrpty_id,
            } => {
                format!(
                    "{}-{}-{}-{}",
                    doc_num.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
                    oper_date.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
                    amount.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
                    ctrpty_id.as_ref().map_or_else(|| "-".to_string(), |v| v.to_string()),
                )
            }

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
            }

            Self::Session { user_id, device_id } => {
                format!("{}-{}", user_id, device_id)
            },
			
			Self::Device { app_str } => {
				app_str.to_string()
			}
        };

        BoxUuid::unchecked(
            uuid::Uuid::new_v5(&uuid::Uuid::NAMESPACE_OID, t.as_bytes())
        )
    }
}