use serde::{Serialize, Deserialize};

use crate::Status;
use crate::primitives::frozen::implements::{Bic, RasAcc, CorAcc, SurName, MidName, FirstName};
use crate::primitives::composite::validator_rules::*;


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, sqlx::Type)]
pub struct Fio {
    #[serde(rename = "@Фамилия")]
    pub sur_name: SurName,
    #[serde(rename = "@Имя")]
    pub first_name: FirstName,
    #[serde(rename = "@Отчество")]
    pub mid_name: Option<MidName>,
}



#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct RasBicAcc {
    bic: Bic,
    ras_acc: RasAcc,
}

impl RasBicAcc {
    const LABEL: &'static str = "РасчСчетБик";

    pub fn new<B, A>(bic: B, ras_acc: A) -> Result<Self, Status> 
        where 
            B: TryInto<Bic>,
            A: TryInto<RasAcc>
            {
                let bic:Bic = bic.try_into().map_err(|_| Status::ValidWrongBicValue)?;
                let ras_acc:RasAcc = ras_acc.try_into().map_err(|_| Status::ValidWrongBanAccValue)?;
                validate_ras_bic_acc(&bic, &ras_acc)
                    .then_some(Self{ bic, ras_acc })
                    .ok_or(Status::ValidWrongRasBicAccPair)
            }
    
    pub fn label() -> &'static str {
        Self::LABEL
    }
}


#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(type_name = "jsonb")]
pub struct CorBicAcc {
    bic: Bic,
    cor_acc: CorAcc
}

impl CorBicAcc {
    const LABEL: &'static str = "КоррСчетБик";

    pub fn label() -> &'static str {
        Self::LABEL
    }

    pub fn new<B, C>(bic: B, cor_acc: C) -> Result<Self, Status> 
    where
        B: TryInto<Bic>,
        C: TryInto<CorAcc>
        {
            let bic:Bic = bic.try_into().map_err(|_| Status::ValidWrongBicValue)?;
            let cor_acc:CorAcc = cor_acc.try_into().map_err(|_| Status::ValidWrongBanAccValue)?;
            validate_cor_bic_acc(&bic, &cor_acc)
                .then_some(Self { bic, cor_acc})
                .ok_or(Status::ValidWrongCorBiccAccPair)
        }
}


