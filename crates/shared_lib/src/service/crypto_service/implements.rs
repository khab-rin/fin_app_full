use serde::{Deserialize, Serialize};

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