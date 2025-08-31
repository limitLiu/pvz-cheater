use super::{KERN_SUCCESS, KernReturn, Result, boolean_t, err, kern_return_t, vm_prot_t};

pub type vm_map_t = u32;
pub type mach_vm_address_t = u64;
pub type mach_vm_size_t = u64;
pub type vm_offset_t = usize;
pub type mach_msg_type_number_t = u32;

pub const PROT_READ: vm_prot_t = 0x01; // read permission
pub const PROT_WRITE: vm_prot_t = 0x02; // write permission
pub const PROT_EXECUTE: vm_prot_t = 0x04; // execute permission

unsafe extern "C" {
  fn mach_vm_protect(
    task: vm_map_t,
    address: mach_vm_address_t,
    size: mach_vm_size_t,
    set_maximum: boolean_t,
    new_protection: vm_prot_t,
  ) -> kern_return_t;

  fn mach_vm_write(
    task: vm_map_t,
    address: mach_vm_address_t,
    data: vm_offset_t,
    dataCtn: mach_msg_type_number_t,
  ) -> kern_return_t;
}

pub fn protect(
  task: vm_map_t,
  address: mach_vm_address_t,
  size: mach_vm_size_t,
  set_maximum: boolean_t,
  new_protection: vm_prot_t,
) -> Result<()> {
  let ret = unsafe { mach_vm_protect(task, address, size, set_maximum, new_protection) };
  if ret == KERN_SUCCESS {
    Ok(())
  } else {
    Err(err::Error::Protect(KernReturn(ret).into()))
  }
}

pub fn write(task: vm_map_t, address: mach_vm_address_t, data: &[u8], data_ctn: mach_msg_type_number_t) -> Result<()> {
  let ret = unsafe { mach_vm_write(task, address, data.as_ptr() as usize, data_ctn) };
  if ret == KERN_SUCCESS {
    Ok(())
  } else {
    Err(err::Error::Write(KernReturn(ret).into()))
  }
}
