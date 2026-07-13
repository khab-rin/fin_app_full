use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};

use shared_lib::Status;
use shared_lib::primitives::frozen::implements::CompInn;

use crate::config::BackApiState;


pub async fn get_json_data(
    state: &BackApiState,
    inn: &CompInn, 
) -> Result<String, Status> {

    let mut headers = HeaderMap::new();

    headers.insert(
        USER_AGENT, 
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"));
    
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/javascript, */*; q=0.01"));

    headers.insert(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8")
    );

    let form_body = format!("query={}", inn);


    let search_res = match state.config.get_std_client()
            .post("https://egrul.nalog.ru/")
            .headers(headers.clone())
            .body(form_body)
            .send()
            .await {
        
        Ok(r) => r,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?Status::QueryPostRequestErr,
                "FUN get_json_data FAILED BY POST QUERY"
            );
            return Err(Status::QueryPostRequestErr);
        }
    };

    let search_json: serde_json::Value = match search_res.json().await {
        Ok(s) => s,
        Err(err) => {
            tracing::error!(
                tech_err = ?err,
                local_err = ?&Status::MappingError,
                "FUN get_json_data FAILED BY POST QUERY"
            );
            return Err(Status::MappingError);
        }
    };

    let search_token = match search_json.get("t").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => {
            std::println!("Не удалось получить токен поиска для ИНН: {}", inn);
            return Ok(());
        }
    };

    // tokio::time::sleep(Duration::from_millis(600)).await;

    // // --- ШАГ 2: Получение токена конкретного субъекта ---
    // let result_res = client
    //     .get(format!("https://egrul.nalog.ru/search-result/{}", search_token))
    //     .headers(headers.clone())
    //     .send()
    //     .await?;

    // let result_json: serde_json::Value = result_res.json().await?;
    // let first_row = match result_json.get("rows").and_then(|r| r.as_array()).and_then(|a| a.first()) {
    //     Some(row) => row,
    //     None => {
    //         std::println!("Субъект с ИНН {} не найден в выдаче", inn);
    //         return Ok(());
    //     }
    // };

    // let company_token = match first_row.get("t").and_then(|v| v.as_str()) {
    //     Some(t) => t,
    //     None => return Ok(())
    // };

    // tokio::time::sleep(Duration::from_millis(500)).await;

    // // --- ШАГ 3: Получение полного сырого JSON карточки ---
    // let card_res = client
    //     .get(format!("https://egrul.nalog.ru/search-result/vyp-status/{}", company_token))
    //     .headers(headers)
    //     .send()
    //     .await?;

    // if card_res.status().is_success() {
    //     let card_json: serde_json::Value = card_res.json().await?;
        
    //     // Форматируем в красивый JSON со сдвигами
    //     let pretty_json = serde_json::to_string_pretty(&card_json)?;
        
    //     // Записываем в файл
    //     let mut file = File::create(filename)?;
    //     file.write_all(pretty_json.as_bytes())?;
    //     std::println!("Успешно сохранен дамп: {}", filename);
    // } else {
    //     std::println!("Ошибка запроса карточки для {}: Статус {}", inn, card_res.status());
    // }

    std::println!("search_json = {:?}", search_json);


    Err(Status::Unknown)
}

    