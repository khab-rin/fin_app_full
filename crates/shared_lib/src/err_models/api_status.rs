use crate::Status;

#[derive(Debug)]
pub struct ApiStatus {
    pub local_err: Status,
    pub tech_err: Option<anyhow::Error>,
    pub context: String,
}

impl std::error::Error for Status{}

pub trait ProcessError {
    #[track_caller]
    fn process_err(self, local_err: Status, ext_info: &str) -> Status;
}


impl<E> ProcessError for E
where 
    E: std::fmt::Display
{
    #[track_caller]
    fn process_err(self, local_err: Status, ext_info: &str) -> Status {
        
        let tech_err = anyhow::Error::msg(self.to_string());
        
        #[cfg(feature = "server")]
        {   
            let caller = std::panic::Location::caller();
            let location = format!("api_error_at::{}:{}:{}", caller.file().replace(".rs", ""), caller.line(), caller.column());

            tracing::error!(
                location = &location,
                tech_err = ?tech_err,
                local_err = ?local_err,
                ext_info = ext_info,
            );
        }
        
        #[cfg(any(feature = "lite_pc", feature = "lite_mobile"))]
        {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, ext_info = {}", tech_err, local_err, ext_info
            );
        }
        
        local_err
    
    }
}


#[derive(Debug, Clone, Copy)]
pub enum StatusLogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl Status {
	#[track_caller]
	pub fn log(level: StatusLogLevel, message: &str) {
		#[cfg(feature = "server")]
		{
			let caller = std::panic::Location::caller();
			let location = format!(
				"api_log_at::{}:{}:{}",
				caller.file().replace(".rs", ""),
				caller.line(),
				caller.column()
			);

			match level {
				StatusLogLevel::Error => tracing::error!(location = %location, message = %message),
				StatusLogLevel::Warn  => tracing::warn!(location = %location, message = %message),
				StatusLogLevel::Info  => tracing::info!(location = %location, message = %message),
				StatusLogLevel::Debug => tracing::debug!(location = %location, message = %message),
				StatusLogLevel::Trace => tracing::trace!(location = %location, message = %message),
			}
		}

		#[cfg(any(feature = "lite_pc", feature = "lite_mobile"))]
		{
			match level {
				StatusLogLevel::Error => log::error!("{}", message),
				StatusLogLevel::Warn  => log::warn!("{}", message),
				StatusLogLevel::Info  => log::info!("{}", message),
				StatusLogLevel::Debug => log::debug!("{}", message),
				StatusLogLevel::Trace => log::trace!("{}", message),
			}
		}
		
	}
}

