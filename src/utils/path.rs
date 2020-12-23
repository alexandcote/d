use shellexpand::{full, LookupError};
use std::env::VarError;
use std::fs::File;
use std::io::Write;
use std::os::unix::io::FromRawFd;
use std::path::PathBuf;

pub fn chdir(path: &str) {
    let mut f = unsafe { File::from_raw_fd(9) };
    if let Err(e) = f.write_fmt(format_args!("cd:{}\n", path)) {
        panic!(e)
    }
}

const D_PATH: &str = "~/src";
pub fn get_base_path() -> Result<PathBuf, LookupError<VarError>> {
    match full(D_PATH) {
        Ok(value) => Ok(PathBuf::from(value.to_string())),
        Err(e) => Err(e),
    }
}
