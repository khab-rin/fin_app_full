use std::sync::Arc;
use blake3;


use shared_lib::service::api_routes::implements::CryptoApiRoutes;
use shared_lib::Status;
use shared_lib::service::auth_service::implements::{
    RegisterResponse, 
    RegistrationRequestDto,
    RegistrationRequest
};

use crate::config::BackApiState;

pub(crate) async fn register_new_user(
    state: &Arc<BackApiState>,
    payload: RegistrationRequestDto
) -> Result<RegisterResponse, Status> {
    
    let crypto_url = format!(
        "{}/{}",
        state.config.crypto_servise.url.trim_end_matches('/'),
        CryptoApiRoutes::CryptoVerifyPerson.get_path().trim_start_matches('/')
    );

    let request:RegistrationRequest = payload.into();

    let check_hash = blake3::hash(&request.document).to_hex();

    if !check_hash.as_str().eq_ignore_ascii_case(&request.doc_hash) {
        tracing::warn!(
            "PERSON LEND ANOTHER FILE"
        );
        return Err(Status::Unknown);
    }

    


    Err(Status::Unknown)
}