pub mod err_models;
pub mod primitives;
pub mod static_data;
pub mod sql_models;
pub mod service;

#[cfg(any(feature = "lite_pc", feature = "lite_mobile"))]
pub mod client;

pub use crate::err_models::implements::Status;
pub use crate::err_models::api_status::ProcessError;
pub use crate::err_models::api_status::StatusLogLevel as LL;

#[cfg(any(feature = "lite_pc", feature = "lite_mobile"))]
pub use crate::service::auth_service::client_state::ClientState;


#[cfg(test)]
mod ts_tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn generate_types_for_svelte() {
		
		use ts_rs::TS;
		
        let output_dirs = [
            "../lite_mobile/ui/src/lib/models/rustModels",
            "../lite_pc/ui/src/lib/models/rustModels",
        ];

        for dir in &output_dirs {

            let path = Path::new(dir);

            primitives::svelte_validate::SvelteValidator::export_all_to(path)
                .expect("Не удалось экспортировать SvelteValidator");

            sql_models::operation::implements::OperationTSTS::export_all_to(path)
                .expect("Не удалось экспортировать OperationTSTS");

            service::auth_service::implements::AuthTSRS::export_all_to(path)
                .expect("Не удалось экспортировать AuthTSRS");

            service::mchd::implements::MchdTSRS::export_all_to(path)
                .expect("Не удалось экспортировать MchdTSRS");

            service::reports::service::ReportTSTS::export_all_to(path)
                .expect("Не удалось экспортировать ReportTSTS");
        }
    }
}