use vowpalwabbit_sys;

use std::{cell::RefCell, sync::Mutex};

use crate::{ErrorString, Result, VWError};
use std::mem;

// This will break if the allocator is not threadsafe
struct Allocator {
    handle: *mut vowpalwabbit_sys::VWAllocator,
}

unsafe impl Sync for Allocator {}
unsafe impl Send for Allocator {}

lazy_static! {
  /// This is an example for using doc comment attributes
  static ref allocator: Allocator = unsafe { Allocator{ handle: vowpalwabbit_sys::vw_get_default_allocator()}};
}

struct Example {
    handle: *mut vowpalwabbit_sys::VWExample,
    /// This is kept around to reduce the need to alloc this for each call.
    error_string: RefCell<ErrorString>,
}

impl Drop for Example {
    fn drop(&mut self) {
        unsafe {
            vowpalwabbit_sys::vw_destroy_example(
                self.handle,
                allocator.handle,
                ErrorString::null_handle(),
            );
        }
    }
}

impl Example {
    pub fn new() -> Result<Self> {
        unsafe {
            let mut example_handle = mem::MaybeUninit::uninit();

            let mut err_str = ErrorString::new();
            let result = vowpalwabbit_sys::vw_create_example(
                example_handle.as_mut_ptr(),
                allocator.handle,
                err_str.as_mut_ptr(),
            );

            if result != vowpalwabbit_sys::VW_SUCCESS {
                return Err(VWError::new(result, err_str.to_str()?));
            }

            let example_handle = example_handle.assume_init();
            return Ok(Example {
                handle: example_handle,
                error_string: RefCell::new(err_str),
            });
        }
    }

    pub fn get_loss(&self) -> Result<f32> {
        unsafe {
            let mut item = mem::MaybeUninit::<f32>::uninit();
            let result = vowpalwabbit_sys::vw_example_get_loss(
                self.handle,
                item.as_mut_ptr(),
                self.error_string.borrow_mut().as_mut_ptr(),
            );
            if result != vowpalwabbit_sys::VW_SUCCESS {
                return Err(VWError::new(result, self.error_string.borrow().to_str()?));
            }
            Ok(item.assume_init())
        }
    }
}

#[test]
fn test_create_example() {
    let _example = Example::new().unwrap();
    assert_eq!(_example.get_loss().unwrap(), 0.);
}
