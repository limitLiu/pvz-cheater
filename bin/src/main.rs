use libprocess_sys::{Process, Result};
mod operation;

use operation::Operation;

fn main() -> Result<()> {
  let process_name = "Plants vs. Zombies";
  let processes = Process::processes()
    .into_iter()
    .filter(|x| x.name == process_name)
    .collect::<Vec<Process>>();
  if let Some(process) = processes.first() {
    let task = process.task()?;
    let sec_kill = Operation::SecKill(task);
    sec_kill.fire()?;

    let no_limit = Operation::NoLimit(task);
    no_limit.fire()?;

    let need_not_sun = Operation::NeedNotSun(task);
    need_not_sun.fire()?;

    let no_cd = Operation::NoCD(task);
    no_cd.fire()?;

    let sun_sub_zero = Operation::SunSubZero(task);
    sun_sub_zero.fire()?;
  }
  Ok(())
}
