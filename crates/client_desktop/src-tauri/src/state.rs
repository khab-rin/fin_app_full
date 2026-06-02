use tauri::Manager;
use std::str::FromStr;
use std::sync::{OnceLock, Arc};

use serde::Deserialize;
use reqwest::header::{CONTENT_TYPE, ACCEPT, HeaderMap};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

use shared_lib::Status;
use shared_lib::make_header;
use shared_lib::service::auth_service::general::*;
use shared_lib::service::auth_service::client_state::*;
use shared_lib::service::auth_service::implements::SessionUserToken;




pub struct ClientState {
    pub config: &'static Config,
    pub app_handle: tauri::AppHandle,
    pub session: tokio::sync::Mutex<Option<Arc<ActiveSession>>>,
    pub temp_info: tokio::sync::Mutex<TempInfo>
}


#[derive(Deserialize, Debug)]
pub struct Config {
    back_api_url: String,
    pub network: NetWork,
    pub sqlite_options: SqliteOptions,
    #[serde(skip)]
    pub clients: Clients,
    #[serde(skip)]
    pub headers: Headers,
}



impl Config {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            dotenvy::dotenv().ok();
            let toml_str = include_str!("../config.toml");
            let mut config: Config = toml::from_str(toml_str)
                .expect("CONFIG MAPPING ERROR");

            let back_api_header = make_header!([
                CONTENT_TYPE => "application/json",
                ACCEPT => "application/json"
            ]);

            config.headers.back_api_header.set(back_api_header).expect("STATIC_MEMORY_ERROR!!!");

            let inst_client = make_client(
                config.network.inst_conn_timeout, 
                config.network.inst_tot_timeout, 
                config.network.inst_request_interval, 
                config.network.inst_retries);
            
            config.clients.instant.set(inst_client).expect("STATIC_MEMORY_ERROR!!!");
            
            let std_client = make_client(
                config.network.std_conn_timeout, 
                config.network.std_tot_timeout, 
                config.network.std_request_interval, 
                config.network.std_retries);
            
            config.clients.standard.set(std_client).expect("STATIC_MEMORY_ERROR!!!");
            
            let rel_client = make_client(
                config.network.rel_conn_timeout, 
                config.network.rel_tot_timeout, 
                config.network.rel_request_interval, 
                config.network.rel_retries);
            
            config.clients.relaxed.set(rel_client).expect("STATIC_MEMORY_ERROR!!!");

            config
        })
    }

    pub fn get_inst_client(&self) -> &'static reqwest_middleware::ClientWithMiddleware {
        Self::global().clients.instant.get().expect("STATIC_MEMORY_ERROR!!!")
    }

    pub fn get_std_client(&self) -> &'static reqwest_middleware::ClientWithMiddleware {
        Self::global().clients.standard.get().expect("STATIC_MEMORY_ERROR!!!")
    }

    pub fn get_rel_client(&self) -> &'static reqwest_middleware::ClientWithMiddleware {
        Self::global().clients.relaxed.get().expect("STATIC_MEMORY_ERROR!!!")
    }

    pub(crate) fn back_api_header(&self) -> &'static HeaderMap {
        Self::global().headers.back_api_header.get().expect("STATIC_MEMORY_ERROR!!!")
    }

    pub(crate) fn back_api_url(&self) -> &str {
        &Self::global().back_api_url
    }
}


pub(crate) async fn init_session(
    state: &ClientState,
    user_data: &SessionUserToken
) -> Result<Status, Status> {
    dotenvy::dotenv().ok();

    let pers_inn = &user_data.user.person.pers_inn;
    let comp_inn = &user_data.user.company.comp_inn;
    let kpp = &user_data.user.company.kpp;

    let app_handle = state.app_handle.clone();

    let app_path = match app_handle.path().app_data_dir() {
        Ok(b) => b,
        Err(err) => {
            log::error!(
                "FUN init_session FAILED BY tauri::AppHandle.path().app_data_dir(), tech_err = {}, local_err = {}",
                err, Status::SystemErr
            );
            return Err(Status::SystemErr);
        }
    };

    let user_path = app_path
        .join(pers_inn.to_string())
        .join(comp_inn.to_string())
        .join(kpp.to_string());

    match std::fs::create_dir_all(&user_path) {
        Ok(_) => {},
        Err(err) => {
            log::error!(
                "FAILED TO CREATE DIRECTORY: {}, tech_err = {}",
                user_path.display(), err
            );
            return Err(Status::SystemErr);
        }
    }
    
    let dp_path = user_path.join("database.db");

    let path_str = dp_path.to_string_lossy();

    let mut db_url = format!("sqlite://{}", path_str);

    if let Ok(env_db_url) = std::env::var("DATABASE_URL_TEMP") {
        db_url = env_db_url;
    }

    let connect_options = SqliteConnectOptions::
        from_str(&db_url)
        .inspect_err(|err| {
            log::error!(
                "LOCAL_DB_URL_ERROR: {}, {}",
                err, Status::SystemErr
            )
        }).map_err(|_| Status::SystemErr)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);

    let pool = match SqlitePoolOptions::new()
        .max_connections(Config::global().sqlite_options.max_connections)
        .acquire_timeout(Config::global().sqlite_options.duration)
        .connect_with(connect_options)
        .await {
            Ok(p) => p,
            Err(err) => {
                log::error!(
                    "INIT_POOL_ERROR: {}, {}",
                    err, Status::SystemErr
                );
                return Err(Status::SystemErr);
            }
        };
    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .inspect_err(|err| {
            log::error!(
                "SQLX_MIGRATE_ERROR: {}, {}",
                err, Status::SystemErr
            )
        }).map_err(|_| Status::SystemErr)?;

    let session = Arc::new(ActiveSession {
        user: user_data.user.clone(),
        local_db: pool,
        token: user_data.token.clone(),
    });

    let mut session_ref = state.session.lock().await;
    *session_ref = Some(session);

    Ok(Status::Success)

}


impl ClientState {
    pub async fn get_session(&self) -> Result<Arc<ActiveSession>, Status> {
        self.session.lock().await.clone().ok_or(Status::ClientSessionMissError)
    }
}