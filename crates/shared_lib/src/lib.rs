pub mod err_models;
pub mod  primitives;
pub mod  static_data;
pub mod  parsers;
pub mod  service;
pub mod  sql_models;
pub mod alias_types;

pub use crate::err_models::implements::Status;
use crate::service::auth_service::implements::AuthStep;
use crate::service::auth_service::client_state::UserLogInfo;

#[cfg(test)]
mod ts_tests {
    use crate::service::auth_service::client_state::NickData;

use super::*;
    use ts_rs::TS;

    #[test]
    fn generate_types_for_svelte() {

        let output_dir = "/home/khabipovrinat/dev/fin_app_full/crates/client_desktop/ui/src/lib/models";

        AuthStep::export_all_to(output_dir)
            .expect("Не удалось экспортировать AuthStep");
        NickData::export_all_to(output_dir)
            .expect("Не удалось экспортировать AuthStep");
    }
}

