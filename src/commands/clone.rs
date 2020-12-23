use regex::Regex;
use std::process::Command;

use crate::utils::{chdir, get_base_path};

pub fn clone(repository: String) {
    let target = get_target(&repository).unwrap();
    Command::new("git")
        .arg("clone")
        .arg(&repository)
        .arg(&target)
        .output()
        .expect("failed to execute process");

    chdir(&target);
}

fn get_target(repository: &str) -> Result<String, &str> {
    let base_path = get_base_path().unwrap();
    let re = Regex::new(r"[/@]([A-z]+.*)").unwrap();
    let path = re.captures(&repository).unwrap();
    let repository_path = path[1].replace(".git", "").replace(":", "/");
    if path.len() > 1 {
        let value = format!("{}/{}", base_path.display(), repository_path);
        return Ok(value);
    }

    Err("Unsupported path")
}
