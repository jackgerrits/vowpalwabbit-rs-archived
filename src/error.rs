use std::error::Error;
use std::fmt;

use vowpalwabbit_sys;
use vowpalwabbit_sys::VWStatus;

pub type Result<T> = std::result::Result<T, VWError>;

#[derive(Debug)]
pub struct VWError {
    code: VWStatus,
    message: String,
}

impl VWError {
    pub fn new<S: Into<String>>(code: VWStatus, msg: S) -> VWError {
        VWError {
            code,
            message: msg.into(),
        }
    }
}

impl fmt::Display for VWError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for VWError {
    fn description(&self) -> &str {
        &self.message
    }
}
