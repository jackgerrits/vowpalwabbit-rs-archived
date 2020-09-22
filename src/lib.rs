#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate lazy_static;

pub mod error;
pub mod example;
pub mod hash;
pub mod options;
pub mod parser;

use std::ffi::CStr;
use std::{cell::RefCell, mem};
use vowpalwabbit_sys;

use error::{Result, VWError};
use options::Options;

struct ErrorString {
    handle: *mut vowpalwabbit_sys::VWErrorInfo,
}

impl ErrorString {
    fn null_handle() -> *mut vowpalwabbit_sys::VWErrorInfo {
        0 as *mut vowpalwabbit_sys::VWErrorInfo
    }

    fn new() -> Self {
        unsafe {
            ErrorString {
                handle: vowpalwabbit_sys::vw_create_error_info(),
            }
        }
    }

    // TODO this should be const
    fn to_str(&self) -> Result<&str> {
        unsafe {
            let raw_string = vowpalwabbit_sys::vw_error_info_get_message(self.handle);
            let c_str: &CStr = CStr::from_ptr(raw_string);

            match c_str.to_str() {
                Ok(unwrapped_str) => Ok(unwrapped_str),
                Err(err) => Err(VWError::new(
                    vowpalwabbit_sys::VW_unknown,
                    format!("Failed to read error string: {}", err.to_string()),
                )),
            }
        }
    }

    fn as_ptr(&self) -> *const vowpalwabbit_sys::VWErrorInfo {
        self.handle
    }

    fn as_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWErrorInfo {
        self.handle
    }
}

impl Drop for ErrorString {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::vw_destroy_error_info(self.handle);
        }
    }
}

pub struct Workspace {
    handle: *mut vowpalwabbit_sys::VWWorkspace,
    /// This is kept around to reduce the need to alloc this for each call.
    error_string: RefCell<ErrorString>,
}

impl Drop for Workspace {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::vw_destroy_workspace(self.handle, ErrorString::null_handle());
        }
    }
}

impl Workspace {
    pub fn new(command_line: String) -> Result<Self> {
        unsafe {
            let mut options = Options::from_command_line(command_line.as_ref())?;

            let mut workspace_handle = mem::MaybeUninit::uninit();

            let ignored: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;

            let mut err_str = ErrorString::new();
            let result = vowpalwabbit_sys::vw_create_workspace(
                options.as_mut_ptr(),
                None,
                ignored,
                workspace_handle.as_mut_ptr(),
                err_str.as_mut_ptr(),
            );

            if result != vowpalwabbit_sys::VW_success {
                return Err(VWError::new(result, err_str.to_str()?));
            }

            let workspace_handle = workspace_handle.assume_init();
            return Ok(Workspace {
                handle: workspace_handle,
                error_string: RefCell::new(err_str),
            });
        }
    }
}

#[test]
fn test_basic_command_line() {
    let _workspace = Workspace::new("--quiet".to_string()).unwrap();
}
