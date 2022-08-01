use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ServiceError {
    pub status_code: u64,
    pub message: String
}


impl ServiceError {
    pub fn new(status_code: u64, message: String) -> Self {
        ServiceError {
            status_code,
            message
        }
    }
}


impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - {}", self.status_code, self.message)
    }
}

