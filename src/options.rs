use std::ffi::CString;
use std::{cell::RefCell, mem};

use crate::{ErrorString, Result, VWError};

pub struct Options {
    handle: *mut vowpalwabbit_sys::VWOptions,
    /// This is kept around to reduce the need to alloc this for each call.
    error_string: RefCell<ErrorString>,
}

impl Options {
    pub fn new() -> Result<Self> {
        let mut err_str = ErrorString::new();
        let mut options = mem::MaybeUninit::uninit();
        unsafe {
            let result =
                vowpalwabbit_sys::vw_create_options(options.as_mut_ptr(), err_str.as_mut_ptr());

            if result != vowpalwabbit_sys::VW_success {
                return Err(VWError::new(result, err_str.to_str()?));
            }

            Ok(Options {
                handle: options.assume_init(),
                error_string: RefCell::new(err_str),
            })
        }
    }

    pub fn from_command_line(command_line: &str) -> Result<Self> {
        let mut err_str = ErrorString::new();
        let command_line_cstr = CString::new(command_line).unwrap();
        let mut options = mem::MaybeUninit::uninit();
        unsafe {
            let result = vowpalwabbit_sys::vw_create_options_from_command_line_cstring(
                command_line_cstr.as_ptr(),
                options.as_mut_ptr(),
                err_str.as_mut_ptr(),
            );

            if result != vowpalwabbit_sys::VW_success {
                return Err(VWError::new(result, err_str.to_str()?));
            }

            Ok(Options {
                handle: options.assume_init(),
                error_string: RefCell::new(err_str),
            })
        }
    }

    pub fn set_float(&self, option_name: &str, option_value: f32) -> () {
        let command_line_cstr = CString::new(option_name).unwrap();
        unsafe {
            let result = vowpalwabbit_sys::vw_options_set_float(
                self.handle,
                command_line_cstr.as_ptr(),
                option_value,
                self.error_string.borrow_mut().as_mut_ptr(),
            );

            if result != vowpalwabbit_sys::VW_success {
                panic!(
                    "set_float failed with code: {}, message: {}",
                    result,
                    self.error_string.borrow().to_str().unwrap()
                );
            }
        }
    }

    pub(crate) fn as_ptr(&self) -> *const vowpalwabbit_sys::VWOptions {
        self.handle
    }

    pub(crate) fn as_mut_ptr(&mut self) -> *mut vowpalwabbit_sys::VWOptions {
        self.handle
    }
}

impl Drop for Options {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::vw_destroy_options(self.handle, ErrorString::null_handle());
        }
    }
}
