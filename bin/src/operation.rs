use libprocess_sys::{vm, Result, FALSE};

#[derive(Debug)]
pub enum Operation {
  /// address: 0x000e5b6c
  /// data: [0x29, 0xd2, 0x90]
  /// size: 3
  SecKill(u32),
  /// address: 0x0001c6da
  /// data: [0x90, 0x90]
  /// size: 2
  NoLimit(u32),
  /// address: 0x0001c71c
  /// data: [0x90, 0x90]
  /// size: 2
  NeedNotSun(u32),
  /// address: 0x000f8334
  /// data: [0x90, 0x90]
  /// size: 2
  NoCD(u32),
  /// address: 0x0001c729
  /// data: [0x90, 0x90, 0x90]
  /// size: 3
  SunSubZero(u32),
}

impl Operation {
  pub fn fire(self) -> Result<()> {
    match self {
      Operation::SecKill(task) => Operation::write(task, 0x000e5b6c, &[0x29, 0xd2, 0x90], 3),
      Operation::NoLimit(task) => Operation::write(task, 0x0001c6da, &[0x90, 0x90], 2),
      Operation::NoCD(task) => Operation::write(task, 0x0001c71c, &[0x90, 0x90], 2),
      Operation::NeedNotSun(task) => Operation::write(task, 0x000f8334, &[0x90, 0x90], 2),
      Operation::SunSubZero(task) => Operation::write(task, 0x0001c729, &[0x90, 0x90, 0x90], 3),
    }
  }

  fn write(task: u32, address: u64, data: &[u8], size: u32) -> Result<()> {
    vm::protect(task, address, size as u64, FALSE, vm::PROT_READ | vm::PROT_WRITE | vm::PROT_EXECUTE)?;
    vm::write(task, address, data, size)
  }
}
