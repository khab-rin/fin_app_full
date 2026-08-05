use serde::{Deserialize, Serialize};

use crate::primitives::frozen::text::{CompInn, Kpp, PersInn, Snils};

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckSignDocData {
    pub init_file: Vec<u8>,  
    pub sign_file: Vec<u8>, 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonSignCheckResult {
    pub is_signed: bool,
    pub text: String
}

pub struct CryptoSignFields{
    pub comp_inn: Option<CompInn>,
    pub kpp: Kpp,
    pub is_manager: bool,
    pub pers_inn: PersInn,
    pub snils: Snils
}