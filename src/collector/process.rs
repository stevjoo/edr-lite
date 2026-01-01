use anyhow::Result;
use std::path::PathBuf;
use sysinfo::{System};

#[derive(Debug, Clone)]
pub struct ProcSnapshot {
    pub pid: u32,
    pub name: String,
    pub exe_path: Option<PathBuf>,
}

pub fn collect_process_snapshot() -> Result<Vec<ProcSnapshot>> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut out = Vec::new();

    for (pid, proc_) in sys.processes() {
        let pid_u32 = pid.to_string().parse::<u32>().unwrap_or(0);
        let name = proc_.name().to_string();
        let exe_path: Option<PathBuf> = proc_.exe().map(|p| p.to_path_buf());

        out.push(ProcSnapshot { pid: pid_u32, name, exe_path });
    }

    Ok(out)
}
