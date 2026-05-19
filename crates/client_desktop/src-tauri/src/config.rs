use shared_lib::Status;
use shared_lib::service::auth_service::implements::SessionUserToken;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};

use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;
use dotenvy;
use reqwest::Client;
use reqwest::header::{HeaderMap, CONTENT_TYPE, ACCEPT};
use serde::{Deserialize, Deserializer};
use directories::ProjectDirs;

use shared_lib::primitives::frozen::implements::{Inn, Bic, RasAcc};
use shared_lib::service::auth_service::client_state::{ClientState, ActiveSession};

macro_rules! make_header {
    ( [ $($key:expr => $val:expr),* $(,)? ] ) => {
        {
            let mut new_header = reqwest::header::HeaderMap::new();
            $(
                new_header.insert($key,
                    reqwest::header::HeaderValue::from_str(&$val.to_string())
                        .expect("HEADER_VALUE_ERROR!!!")
                );
            )*
            new_header
        }       
    };
}


#[derive(Deserialize, Debug)]
pub(crate) struct Config {
    pub(crate) own_company: OwnCompany,
    pub(crate) network: NetWork,
    #[serde(skip)]
    pub(crate) client: OnceLock<Client>,
    #[serde(skip)]
    pub(crate) headers: Headers,
    #[serde(skip)]
    pub(crate) database: String,
}


#[derive(Default, Debug)]
pub(crate) struct Headers {
    pub(crate) back_api_header: HeaderMap
}


#[derive(Deserialize, Debug)]
pub(crate) struct OwnCompany {
    pub(crate) own_ras_acc: RasAcc,
    pub(crate) own_bic: Bic,
    pub(crate) own_inn: Inn,
    pub(crate) own_comp_id: String,
    pub(crate) own_user_id: String,
}




#[derive(Deserialize, Debug)]
pub(crate) struct NetWork {
    pub(crate) api_base_url: String,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) req_conn_timeout: Duration,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) req_tot_timeout: Duration
}

impl Config {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<Config> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            dotenvy::dotenv().ok();
            let toml_str = include_str!("../config.toml"); 

            let mut config: Config = toml::from_str(toml_str)
                .expect("CONFIG_TOML_PARSE_ERROR!!!");
            
            let back_api_header = make_header!([
                CONTENT_TYPE => "application/json",
                ACCEPT => "application/json"
            ]);

            config.headers = Headers { back_api_header };

            let client = Client::builder()
                .connect_timeout(config.network.req_conn_timeout)
                .timeout(config.network.req_tot_timeout)
                .build()
                .expect("FATAL: Failed to build reqwest::Client");
            

            config.client.set(client).expect("STATIC_MEMORY_ERROR!!!");

            
            config.database = std::env::var("DATABASE_URL_TEMP")
                .unwrap_or_else(|_| Self::make_base_url(&config.own_company.own_inn));

            config
        })
    }


    pub fn make_base_url(inn: &Inn) -> String {
        
        let proj_dirs = ProjectDirs::
                from("com",  "XPinAT", inn.as_ref())
                .expect("FILE_SYSTEM_ACCESS_ERROR!!!");

        let local_db_dir = proj_dirs.data_dir();

        std::fs::create_dir_all(local_db_dir)
            .expect("FILE_SYSTEM_ACCESS_ERROR!!!");

        let local_db_path = local_db_dir
            .join("local_db.db");

        format!("sqlite:{}?mode=rwc", local_db_path
            .to_str()
            .expect("WRONG_UTF_BYTES!"))

    }

    pub(crate) fn get_client() -> &'static Client {
        Self::global().client.get().expect("STATIC_MEMORY_ERROR!!!")
    }
}


pub(crate) mod time_parser {
    use super::*;
    pub(crate) fn duration_from_u64<'de, D>(func: D) -> Result<Duration, D::Error> 
    where D: Deserializer<'de>  {
        let secs = u64::deserialize(func)?;
        Ok(Duration::from_secs(secs))
    }  
}

#[derive(Deserialize, Debug)]
pub(crate) struct TomlConfig {
    network: TomlNetwork,
    sqlite_options: SqliteOptions,
    own_company: TomlOwnCompany
}

#[derive(Deserialize, Debug)]
pub(crate) struct TomlNetwork {
    api_base_url: String,
    req_conn_timeout: u32,
    req_tot_timeout: u32,
}

#[derive(Deserialize, Debug)]
pub(crate) struct TomlOwnCompany {
    own_ras_acc: String,
    own_inn: String,
    own_bic: String,
    own_comp_id: String,
    own_user_id: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SqliteOptions {
    pub(crate) max_connections: u32,
    #[serde(deserialize_with = "time_parser::duration_from_u64")]
    pub(crate) duration: Duration
}


pub(crate) async fn init_session(
    state: &ClientState,
    user_data: SessionUserToken
) -> Result<Status, Status> {
    dotenvy::dotenv().ok();
    let toml_str = include_str!("../config.toml");
    let toml_config: TomlConfig = toml::from_str(toml_str)
        .expect("CONFIG_TOML_WRONG_MAPPING!!!");
    
    let pers_inn = &user_data.user.person.inn;
    let comp_inn = &user_data.user.company.inn;
    let comp_kpp = &user_data.user.company.kpp;

    let db_path = std::path::PathBuf::from(&state.app_path)
        .join(pers_inn.to_string())
        .join(comp_inn.to_string())
        .join(format!("{}.db", comp_kpp));

    let mut db_url = format!("sqlite:{}", db_path.to_string_lossy());

    if let Ok(env_db_url) = std::env::var("DATABASE_URL_TEMP") {
        db_url = env_db_url;
    }

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
        .inspect_err(|err| {
            log::error!(
                "MAKE_LOCAL_DB_DIR_ERROR, {}, {}",
                err, Status::ClientInitSessionGetPathParrent
            )
        }).map_err(|_| Status::ClientInitSessionGetPathParrent)?
    }


    let connect_options = SqliteConnectOptions::
        from_str(&db_url)
        .inspect_err(|err| {
            log::error!(
                "LOCAL_DB_URL_ERROR: {}, {}",
                err, Status::ClientInitSessionInitSqlOptions
            )
        }).map_err(|_| Status::ClientInitSessionInitSqlOptions)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .foreign_keys(true);


    let pool = SqlitePoolOptions::
        new()
        .max_connections(toml_config.sqlite_options.max_connections)
        .acquire_timeout(toml_config.sqlite_options.duration)
        .connect_with(connect_options)
        .await
        .inspect_err(|err| {
            log::error!(
                "INIT_POOL_ERROR: {}, {}",
                err, Status::ClientInitSessionInitPool
            )
        }).map_err(|_| Status::ClientInitSessionInitPool)?;

    
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .inspect_err(|err| {
            log::error!(
                "SQLX_MIGRATE_ERROR: {}, {}",
                err, Status::ClientInitSqlxMigrate
            )
        }).map_err(|_| Status::ClientInitSqlxMigrate)?;
    
    
    let session = ActiveSession {
        user: user_data.user,
        local_db: pool,
        token: user_data.token
    };

    let mut session_ref = state.session.lock().await;

    *session_ref = Some(session);


    Ok(Status::Success)
}
