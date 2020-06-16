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

pub trait VWErrorNew<T> {
    fn new(code: VWStatus, msg: T) -> VWError;
}

impl VWErrorNew<&str> for VWError {
    fn new(code: VWStatus, msg: &str) -> VWError {
        VWError {
            code,
            message: msg.to_string(),
        }
    }
}

impl VWErrorNew<String> for VWError {
    fn new(code: VWStatus, msg: String) -> VWError {
        VWError { code, message: msg }
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
