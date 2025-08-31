use std::ffi::{CStr, c_char};

use super::KernReturn;

pub type mach_error_t = i32;

unsafe extern "C" {
  fn mach_error_string(error_value: mach_error_t) -> *mut c_char;
}

#[derive(Debug)]
pub enum Error {
  TaskErr(String),
  Protect(String),
  Write(String),
}

impl From<KernReturn> for String {
  fn from(value: KernReturn) -> String {
    let c_str = unsafe { CStr::from_ptr(mach_error_string(value.0)) };
    c_str.to_string_lossy().to_string()
  }
}
