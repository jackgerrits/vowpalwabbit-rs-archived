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

impl All {
    pub fn new(command_line: &str) -> All {
        let command_line_cstr = CString::new(command_line).unwrap();
        let a;
        unsafe {
            a = vowpalwabbit_sys::VW_InitializeA(command_line_cstr.as_ptr());
        }

        All { handle: a }
    }
}

impl<'a> Example<'a> {
    pub fn new(command_line: &str) -> All {
        let command_line_cstr = CString::new(command_line).unwrap();
        let a;
        unsafe {
            a = vowpalwabbit_sys::VW_InitializeA(command_line_cstr.as_ptr());
        }

        All { handle: a }
    }

    pub fn finish(&mut self) {
        match self.all_handle {
            Some(all_handle) => unsafe {
                vowpalwabbit_sys::VW_FinishExample(all_handle.handle, self.handle);
                self.all_handle = Option::None
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
