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
        let caller = std::panic::Location::caller();
        let tech_err = anyhow::Error::msg(self.to_string());

        #[cfg(feature = "server")]
        {   
            tracing::error!(
                tech_err = ?tech_err,
                local_err = ?local_err,
                ext_info = ext_info,
            );
        }
        
        #[cfg(feature = "client")]
        {
            log::error!(
                "tech_err = {:?}, local_err = {:?}, ext_info = {}", tech_err, local_err, ext_info
            );
        }
        
        local_err
    
    }
}

