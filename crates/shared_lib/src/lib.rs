pub mod err_models;
pub mod primitives;
pub mod static_data;
pub mod sql_models;
pub mod service;

#[cfg(feature = "client")]
pub mod client;

pub use crate::err_models::implements::Status;
pub use crate::err_models::api_status::ProcessError;

#[cfg(feature = "client")]
pub use crate::service::auth_service::client_state::ClientState;

#[cfg(test)]
mod ts_tests {

use super::*;
    use ts_rs::TS;

    #[test]
    fn generate_types_for_svelte() {

        let output_dir = "../lite_mobile/ui/src/lib/models/rustModels";

        primitives::svelte_validate::SvelteValidator::export_all_to(output_dir)
            .expect("Не удалось экспортировать SvelteValidator");

        sql_models::operation::implements::OperationTSTS::export_all_to(output_dir)
            .expect("Не удалось экспортировать OperationTSTS");

        service::auth_service::implements::AuthTSRS::export_all_to(output_dir)
         .expect("Не удалось экспортировать AuthTSRS");

        service::mchd::implements::MchdTSRS::export_all_to(output_dir)
            .expect("Не удалось экспортировать MchdTSRS");

		service::reports::service::ReportTSTS::export_all_to(output_dir)
			.expect("Не удалось экспортировать ReportTSTS");

    }
}

 


