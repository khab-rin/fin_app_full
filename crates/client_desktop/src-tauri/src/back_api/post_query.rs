use serde::Serialize;
use reqwest_middleware::ClientWithMiddleware;
use reqwest::Response;

use shared_lib::Status;
use shared_lib::service::api_routes::implements::ApiRoutes;

use crate::state::ClientState;

pub(crate) async fn post_query_back_api<T>(
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
        route.get_path()
    );

    let response = match client
        .post(&back_api_url)
        .headers(state.config.back_api_header().clone())
        .json(data)
        .send()
        .await {
            Ok(r) => r,
            Err(err) => {
                log::error!(
                    "FUN post_query_back_api FILED, tech_err = {}", err
                );
                match err {
                    reqwest_middleware::Error::Reqwest(reqwest_err) => {
                        if reqwest_err.is_timeout() {
                            log::error!("Back API request timeout, local_err = {}", Status::BackApiError);
                            return Err(Status::BackApiError);
                        }
                        if reqwest_err.is_connect() {
                            log::error!("Back API connection failed (server is offline)");
                            return Err(Status::QueryConnectErr);
                        }
                        
                        return Err(Status::QueryPostRequestErr);
                    },

                    reqwest_middleware::Error::Middleware(_) => {
                        log::error!("Request blocked or failed inside middleware layer");
                        return Err(Status::SystemErr); // 400
                    }
                }
            }
        };
    
    if !response.status().is_success() {
        let back_err = response
            .json::<Status>()
            .await
            .unwrap_or(Status::Unknown);
        log::error!(
            "FUN post_query_back_api FAILED, BACK API GIVE WRONG STATUS. Backend error code: {}, local_err = {:?}",
            back_err, Status::BackApiError
        );
        return Err(Status::BackApiError);
    }


    Ok(response)
}