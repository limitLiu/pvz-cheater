#![allow(non_camel_case_types)]

pub mod err;
pub mod vm;

use std::{
  ffi::CStr,
  ops::{Deref, Drop},
  os::raw::{c_char, c_int},
  slice,
};

pub type Result<T> = std::result::Result<T, err::Error>;

const KERN_SUCCESS: kern_return_t = 0;

pub type mach_port_name_t = u32;
pub type kern_return_t = i32;
pub type boolean_t = i32;
pub type vm_prot_t = i32;

pub struct KernReturn(pub kern_return_t);

pub const FALSE: boolean_t = 0;

unsafe extern "C" {
  fn get_processes(len: *mut c_int) -> *const ProcessInfo;
  fn free_process_infos(process_infos: *const ProcessInfo);
  fn get_mach_task_self() -> mach_port_name_t;
  fn task_for_pid(target_tport: mach_port_name_t, pid: i32, t: *const mach_port_name_t) -> kern_return_t;
}

#[repr(C)]
struct ProcessInfo {
  pid: i32,
  name: [c_char; 256],
}

struct ProcessInfoWrapper {
  ptr: *const ProcessInfo,
  len: usize,
}

impl Deref for ProcessInfoWrapper {
  type Target = [ProcessInfo];

  fn deref(&self) -> &[ProcessInfo] {
    unsafe { slice::from_raw_parts(self.ptr, self.len) }
  }
}

impl Drop for ProcessInfoWrapper {
  fn drop(&mut self) {
    unsafe {
      free_process_infos(self.ptr);
    }
  }
}

#[derive(Debug, Default)]
pub struct Process {
  pub pid: i32,
  pub name: String,
}

impl Process {
  pub fn new(name: String, pid: i32) -> Process {
    Process { name, pid }
  }

  pub fn processes() -> Vec<Process> {
    let mut len: c_int = 0;
    let mut v = Vec::new();
    unsafe {
      let pointer = get_processes(&mut len);
      if !pointer.is_null() && len > 0 {
        let process_infos = ProcessInfoWrapper {
          ptr: pointer,
          len: len as usize,
        };
        for info in process_infos.iter() {
          let name = CStr::from_ptr(info.name.as_ptr()).to_string_lossy().into_owned();
          v.push(Process::new(name, info.pid));
        }
      }
    }
    v
  }

  pub fn task(&self) -> Result<mach_port_name_t> {
    let task = 0;
    let ret = unsafe { task_for_pid(mach_task_self(), self.pid, &task) };
    if ret == KERN_SUCCESS {
      Ok(task)
    } else {
      Err(err::Error::TaskErr(KernReturn(ret).into()))
    }
  }
}

pub fn mach_task_self() -> mach_port_name_t {
  unsafe { get_mach_task_self() }
}
