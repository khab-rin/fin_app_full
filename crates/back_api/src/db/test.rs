use std::sync::Arc;
use axum::extract::State;
use axum::Json;

use crate::handlers::service::mchd::handler::get_ifns_handler;
use crate::config::BackApiState;

use crate::config


use shared_lib::primitives::frozen::implements::CompInn;



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ifns_handler() {

        let base_url = &config::Config::global().data_base.database_url;

        let pool_fast = config::create_pool(
            base_url, 
            &config::Config::global().postgresql_options.fast_max_conn, 
            &config::Config::global().postgresql_options.fast_timeout
        ).await;

        let pool_long = config::create_pool(
            base_url, 
            &config::Config::global().postgresql_options.long_max_conn, 
            &config::Config::global().postgresql_options.long_timeout
        ).await;


        let state = Arc::new(config::BackApiState {
            pool_fast,
            pool_long,
            config: config::Config::global()
        });


        let state_arc = Arc::new(base_state);

        // 2. Создаем входные данные (payload)
        let payload = CompInn {
            inn: CompInn::unchecked("161101510882"), // Твой ИНН ИП для проверки

        };

        // 3. Вызываем хендлер напрямую, заворачивая аргументы в экстракторы Axum
        let result = get_ifns_handler(
            State(state_arc), 
            Json(payload)
        ).await;

        // 4. Проверяем результат
        match result {
            Ok(Json(json_string)) => {
                std::println!("\n=== ОТВЕТ ХЕНДЛЕРА ===");
                std::println!("{}", json_string);
                std::println!("======================\n");
            }
            Err(status) => {
                std::eprintln!("Хендлер вернул ошибку Status: {:?}", status);
                panic!("Тест провален");
            }
        }
    }
}