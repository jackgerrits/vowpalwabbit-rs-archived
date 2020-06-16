#[macro_use]
extern crate pest_derive;

pub mod error;
pub mod hash;
pub mod parser;

use std::ffi::CStr;
use std::ffi::CString;
use std::mem;
use vowpalwabbit_sys;

use error::{Result, VWError, VWErrorNew};

struct ErrorString {
    handle: *mut vowpalwabbit_sys::VWErrorString,
}

impl ErrorString {
    fn null_handle() -> *mut vowpalwabbit_sys::VWErrorString {
        0 as *mut vowpalwabbit_sys::VWErrorString
    }

    fn new() -> Self {
        unsafe {
            ErrorString {
                handle: vowpalwabbit_sys::VWCreateErrorString(),
            }
        }
    }

    // TODO this should be const
    fn to_str(&mut self) -> Result<&str> {
        unsafe {
            let raw_string = vowpalwabbit_sys::VWErrorStringToCString(self.handle);
            let c_str: &CStr = CStr::from_ptr(raw_string);

            match c_str.to_str() {
                Ok(unwrapped_str) => Ok(unwrapped_str),
                Err(err) => Err(VWError::new(
                    vowpalwabbit_sys::VW_FAIL,
                    format!("Failed to read error string: {}", err.to_string()),
                )),
            }
        }
    }

    fn as_ptr(&self) -> *const vowpalwabbit_sys::VWErrorString {
        self.handle
    }

    fn as_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWErrorString {
        self.handle
    }
}

impl Drop for ErrorString {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWDestroyErrorString(self.handle);
        }
    }
}

pub struct Workspace {
    pub handle: *mut vowpalwabbit_sys::VWWorkspace,
}

impl Drop for Workspace {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VWDestroyWorkspace(self.handle, ErrorString::null_handle());
        }
    }
}

impl Workspace {
    pub fn new(command_line: String) -> Result<Self> {
        unsafe {
            let mut err_str = ErrorString::new();
            let command_line_cstr = CString::new(command_line).unwrap();
            let mut options = mem::MaybeUninit::uninit();
            let result = vowpalwabbit_sys::VWCreateOptionsFromCommandLineCString(
                command_line_cstr.as_ptr(),
                options.as_mut_ptr(),
                err_str.as_mut_ptr(),
            );

            if result != vowpalwabbit_sys::VW_SUCCESS {
                return Err(VWError::new(result, err_str.to_str()?));
            }
            let options = options.assume_init();

            let mut workspace_handle = mem::MaybeUninit::uninit();

            let ignored: *mut std::ffi::c_void = 0 as *mut std::ffi::c_void;

            let result = vowpalwabbit_sys::VWCreateWorkspace(
                options,
                false,
                None,
                ignored,
                workspace_handle.as_mut_ptr(),
                err_str.as_mut_ptr(),
            );

            if result != vowpalwabbit_sys::VW_SUCCESS {
                return Err(VWError::new(result, err_str.to_str()?));
            }

            let workspace_handle = workspace_handle.assume_init();
            vowpalwabbit_sys::VWDestroyOptions(options, err_str.as_mut_ptr());
            return Ok(Workspace {
                handle: workspace_handle,
            });
        }
    }
}

#[test]
fn test_basic_command_line() {
    let _workspace = Workspace::new("--quiet".to_string()).unwrap();
}
