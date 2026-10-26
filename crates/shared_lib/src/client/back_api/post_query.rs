use serde::Serialize;
use reqwest_middleware::ClientWithMiddleware;
use reqwest::Response;

use crate::{Status, ClientState, LL};
use crate::service::api_routes::implements::ApiRoutes;

pub async fn post_query_back_api<T>(
    state: &ClientState,
    client: &ClientWithMiddleware,
    route: ApiRoutes,
    data: &T,
) -> Result<Response, Status>
where 
    T: Serialize + ?Sized,
{
    let back_api_url = format!("{}/{}",
        state.config.back_api_url().trim_end_matches('/'),
        route.get_path().trim_start_matches('/')
    );


    let response = match client
        .post(&back_api_url)
        .headers(state.config.back_api_header().clone())
        .json(data)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                Status::log(LL::Error,&format!(
					"FUN post_query_back_api FILED, tech_err = {}, url = {}",
					err, back_api_url
				));
                match err {
                    reqwest_middleware::Error::Reqwest(reqwest_err) => {
                        if reqwest_err.is_timeout() {
                            Status::log(LL::Error,&format!(
        						"Back API request timeout, local_err = {}, url = {}",
								Status::BackApiError, back_api_url
    						));
                            return Err(Status::BackApiError);
                        }
                        if reqwest_err.is_connect() {
                            Status::log(LL::Error, &format!("Back API connection failed (server is offline), url = {}", back_api_url));
                            return Err(Status::QueryConnectErr);
                        }
                        
                        return Err(Status::QueryPostRequestErr);
                    },

                    reqwest_middleware::Error::Middleware(_) => {
                        Status::log(LL::Error,&format!("Request blocked or failed inside middleware layer, url = {}", back_api_url),);
                        return Err(Status::SystemErr); // 400
                    }
                }
            }
        };
    
    if !response.status().is_success() {
        let status_code = response.status();
        let headers = response.headers().clone();
        
        let is_json = headers
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.contains("application/json"))
            .unwrap_or(false);

        // Читаем сырое тело ответа
        let raw_body = response.text().await.unwrap_or_else(|_| "<failed to read body>".to_string());

        if is_json {
            // Пробуем распарсить сырой текст в статус, чтобы при неудаче сохранить сырой JSON в логе
            match serde_json::from_str::<Status>(&raw_body) {
                Ok(back_err) => {
                    Status::log(LL::Error,&format!(
						"BACK API REJECTED REQUEST! url: {}, status: {}, backend_enum_err: {:?}, raw_body: {}",
						back_api_url, status_code, back_err, raw_body
					));
                }
                Err(parse_err) => {
                    Status::log(LL::Error,&format!(
						"BACK API RETURNED JSON BUT DESERIALIZE TO Status FAILED! url: {}, status: {}, parse_err: {}, raw_body: {}",
						back_api_url, status_code, parse_err, raw_body
					));
                }
            }
        } else {
            Status::log(LL::Error,&format!(
				"BACK API CRASHED OR REJECTED WITH NON-JSON! url: {}, status: {}, headers: {:?}, raw_body: {}",
				back_api_url, status_code, headers, raw_body
			));
        }

        return Err(Status::BackApiError);
    }


    Ok(response)
}