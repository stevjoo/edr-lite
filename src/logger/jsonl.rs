use anyhow::Result;
use std::{
    fs::{create_dir_all, OpenOptions},
    io::Write,
    path::Path,
};

pub fn append_jsonl<P: AsRef<Path>>(file_path: P, line: &str) -> Result<()> {
    if let Some(parent) = file_path.as_ref().parent() {
        create_dir_all(parent)?;
    }

    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    f.write_all(line.as_bytes())?;
    f.write_all(b"\n")?;
    Ok(())
}
