pub mod err_models;
pub mod  primitives;
pub mod  static_data;
pub mod  parsers;
pub mod  service;
pub mod  sql_models;
pub mod alias_types;

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

        service::auth_service::client_state::NickData::export_all_to(output_dir)
            .expect("Не удалось экспортировать UserLogInfo");

        service::auth_service::implements::AuthStep::export_all_to(output_dir)
            .expect("Не удалось экспортировать AuthStep");

        service::auth_service::implements::ExternalDeviceData::export_all_to(output_dir)
            .expect("Не удалось экспортировать ExternalDeviceData");

        service::auth_service::implements::IngoingData::export_all_to(output_dir)
            .expect("Не удалось экспортировать IngoingData");

        service::auth_service::implements::InitFiles::export_all_to(output_dir)
            .expect("Не удалось экспортировать InitFiles");

        service::auth_service::implements::PasswordDataClientShort::export_all_to(output_dir)
            .expect("Не удалось экспортировать PasswordDataClientShort");

        service::auth_service::implements::RegistrationData::export_all_to(output_dir)
            .expect("Не удалось экспортировать RegistrationData");

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
