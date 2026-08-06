use crate::Status;

#[derive(Debug)]
pub struct ApiStatus {
    pub local_err: Status,
    pub tech_err: Option<anyhow::Error>,
    pub context: String,
}

impl std::fmt::Display for ApiStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[tech_err = {:?}] local_err = {:?}", self.tech_err, self.local_err)
    }
}

impl std::error::Error for ApiStatus {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.tech_err.as_ref().map(|e| e.as_ref())
    }
}


#[cfg(feature = "server")]
impl axum::response::IntoResponse for ApiStatus {
    fn into_response(self) -> axum::response::Response {
        tracing::error!(err_info = %self);

        self.local_err.into_response()
    }
}


impl ApiStatus {
    pub fn log_err(&self) {
        #[cfg(feature = "server")]
        {
            tracing::error!(err_info = %self);
        }
        
        #[cfg(feature = "client")]
        {
            log::error!("err_info = {}", self);
        }
    }

    #[track_caller]
    pub fn process_err<E>(tech_err: E, local_err: Status) -> Self 
    where
        E: Into<anyhow::Error>,
    {
        let caller = std::panic::Location::caller();
        let context = format!("In {}:{}:{}", caller.file(), caller.line(), caller.column());

        let api_status = Self {
            local_err,
            tech_err: Some(tech_err.into()),
            context,
        };

        api_status.log_err();

        api_status
    }
}


pub trait IntoApiStatus {
    fn process_err(self, status: Status) -> Status;
}


impl<E> IntoApiStatus for E 
where
    E: Into<anyhow::Error>,
{
    #[track_caller]
    fn process_err(self, local_err: Status) -> Status {
        let caller = std::panic::Location::caller();
        let context = format!("In {}:{}:{}", caller.file(), caller.line(), caller.column());
        
        let api_status = ApiStatus {
            local_err,
            tech_err: Some(self.into()),
            context,
        };
        
        api_status.log_err();
        api_status.local_err
    }
}

impl IntoApiStatus for Status {
    #[track_caller]
    fn process_err(self, local_err: Status) -> Status {
        let caller = std::panic::Location::caller();
        let context = format!("In {}:{}:{}", caller.file(), caller.line(), caller.column());
        
        let api_status = ApiStatus {
            local_err,
            tech_err: Some(anyhow::Error::msg(self.to_string())),
            context,
        };
        
        api_status.log_err();
        api_status.local_err
    }
}


