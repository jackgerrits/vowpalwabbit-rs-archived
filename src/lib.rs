#[macro_use]
extern crate pest_derive;

pub mod hash;
pub mod parser;

use std::ffi::CStr;
use std::ffi::CString;
use std::option::Option;
use vowpalwabbit_sys;

pub struct All {
    handle: vowpalwabbit_sys::VW_HANDLE,
}

pub struct Example<'a> {
    handle: vowpalwabbit_sys::VW_EXAMPLE,
    all_handle: Option<&'a All>,
}

impl From<String> for All {
    fn from(command_line: String) -> Self {
        let command_line_cstr = CString::new(command_line).unwrap();
        let a;
        unsafe {
            a = vowpalwabbit_sys::VW_InitializeA(command_line_cstr.as_ptr());
        }

        All { handle: a }
    }
}

impl From<&str> for All {
    fn from(command_line: &str) -> Self {
        let command_line_cstr = CString::new(command_line).unwrap();
        let a;
        unsafe {
            a = vowpalwabbit_sys::VW_InitializeA(command_line_cstr.as_ptr());
        }

        All { handle: a }
    }
}

impl From<&CStr> for All {
    fn from(command_line: &CStr) -> Self {
        let a;
        unsafe {
            a = vowpalwabbit_sys::VW_InitializeA(command_line.as_ptr());
        }

        All { handle: a }
    }
}
trait ExampleNew<'a, T> {
    fn new(all: &'a All, line: T) -> Example<'a>;
}

impl<'a> ExampleNew<'a, &CStr> for Example<'a> {
    fn new(all: &'a All, line: &CStr) -> Example<'a> {
        let ex;
        unsafe {
            ex = vowpalwabbit_sys::VW_ReadExampleA(all.handle, line.as_ptr());
        }
        Example {
            handle: ex,
            all_handle: Some(all),
        }
    }
}

impl<'a> ExampleNew<'a, &str> for Example<'a> {
    fn new(all: &'a All, line: &str) -> Example<'a> {
        let ex_str = CString::new(line).unwrap();
        let ex;
        unsafe {
            ex = vowpalwabbit_sys::VW_ReadExampleA(all.handle, ex_str.as_ptr());
        }
        Example {
            handle: ex,
            all_handle: Some(all),
        }
    }
}

impl<'a> ExampleNew<'a, String> for Example<'a> {
    fn new(all: &'a All, line: String) -> Example<'a> {
        let ex_str = CString::new(line).unwrap();
        let ex;
        unsafe {
            ex = vowpalwabbit_sys::VW_ReadExampleA(all.handle, ex_str.as_ptr());
        }
        Example {
            handle: ex,
            all_handle: Some(all),
        }
    }
}

impl<'a> Example<'a> {
    pub fn finish(&mut self) {
        match self.all_handle {
            Some(all_handle) => unsafe {
                vowpalwabbit_sys::VW_FinishExample(all_handle.handle, self.handle);
                self.all_handle = Option::None;
            },
            None => (),
        }
    }
}

impl Drop for All {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::VW_Finish(self.handle);
        }
    }
}

impl<'a> Drop for Example<'a> {
    fn drop(&mut self) {
        self.finish();
    }
}

#[test]
fn test_basic_command_line() {
    let a: CString = CString::new("--quiet").unwrap();
    let all = All::from(a.as_c_str());
    let _ = Example::new(&all, "test");
}
