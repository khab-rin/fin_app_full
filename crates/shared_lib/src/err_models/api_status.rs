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
        if let Some(ref tech_err) = self.tech_err {
            tracing::error!(status = ?self.local_err, context = %self.context, tech_err = ?tech_err, "Application Error");
        } else {
            tracing::error!(status = ?self.local_err, context = %self.context, "Application Error");
        }

        self.local_err.into_response()
    }
}


#[cfg(feature = "client")]
impl ApiStatus {
    pub fn log_client(&self) {
        if let Some(ref tech_err) = self.tech_err {
            log::error!(
                "Application Error: local_err = {:?}, context = {}, tech_err = {:?}", 
                self.local_err, self.context, tech_err
            );
        } else {
            log::error!(
                "Application Error: local_err = {:?}, context = {}", 
                self.local_err, self.context
            );
        }
    }
}


pub trait OptionApiStatusExt<T> {
    fn status(self, local_err: Status, context: impl Into<String>) -> Result<T, ApiStatus>;
}

impl<T> OptionApiStatusExt<T> for Option<T> {
    fn status(self, local_err: Status, context: impl Into<String>) -> Result<T, ApiStatus> {
        self.ok_or_else(|| ApiStatus {
            local_err,
            tech_err: None,
            context: context.into(),
        })
    }
}

// Трейт для быстрого превращения Любой Ошибки в ApiStatus
pub trait ResultApiStatusExt<T, E> {
    fn status(self, local_err: Status, context: impl Into<String>) -> Result<T, ApiStatus>;
}

impl<T, E: std::error::Error + Send + Sync + 'static> ResultApiStatusExt<T, E> for Result<T, E> {
    fn status(self, local_err: Status, context: impl Into<String>) -> Result<T, ApiStatus> {
        self.map_err(|err| ApiStatus {
            local_err,
            tech_err: Some(anyhow::Error::new(err)),
            context: context.into(),
        })
    }
}