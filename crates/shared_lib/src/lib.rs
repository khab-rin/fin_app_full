pub mod alias_types;
pub mod err_models;
pub mod parsers;
pub mod primitives;
pub mod service;
pub mod static_data;
pub mod sql_models;


pub use crate::err_models::implements::Status;

#[cfg(test)]
mod ts_tests {

use super::*;
    use ts_rs::TS;

    #[test]
    fn generate_types_for_svelte() {

        let output_dir = "../client_desktop/ui/src/lib/models/rustModels";

        primitives::svelte_validate::SvelteValidator::export_all_to(output_dir)
            .expect("Не удалось экспортировать SvelteValidator");

        service::auth_service::implements::AuthTs::export_all_to(output_dir)
            .expect("Не удалось экспортировать AuthStep");

        service::mchd::home_mchd_power::HomeMchdPower::export_all_to(output_dir)
            .expect("Не удалось экспортировать HomeMchdPower");
        
        service::mchd::home_mchd_power::HomePowerInfo::export_all_to(output_dir)
            .expect("Не удалось экспортировать MchdPowerInfo");

        service::mchd::poa::PoaMchd::export_all_to(output_dir)
            .expect("Не удалось экспортировать PoaMchd");

        service::mchd::service::MchdStep::export_all_to(output_dir)
            .expect("Не удалось экспортировать MchdStep");

        service::mchd::service::MchdType::export_all_to(output_dir)
            .expect("Не удалось экспортировать MchdType");
    }
}
