use rff::scorer::score;
use std::f64::NEG_INFINITY;
use std::path::PathBuf;

use crate::utils::{chdir, get_base_path};

struct Candidate {
    score: f64,
    repository: String,
}

pub fn cd(path: String) {
    if path == "-" {
        return chdir(&path);
    }

    let repositories = list_all_repositories();
    let mut candidates: Vec<Candidate> = vec![];
    for repository in repositories {
        let score = score(path.as_str(), &repository);
        if score != NEG_INFINITY {
            candidates.push(Candidate { score, repository })
        }
    }

    if !candidates.is_empty() {
        candidates.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        return chdir(&candidates[0].repository);
    }
}

fn list_all_repositories() -> Vec<String> {
    let base_path = get_base_path().unwrap();
    let services = get_directories(&base_path).unwrap();
    let mut repositories: Vec<String> = vec![];
    for service in services.iter() {
        let owners = get_directories(&service).unwrap();
        for owner in owners.iter() {
            let repo = get_directories(owner)
                .unwrap()
                .iter()
                .map(|repo| repo.display().to_string())
                .collect::<Vec<String>>();

            repositories.extend(repo);
        }
    }

    repositories
}

fn get_directories(path: &PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {
    match path.read_dir() {
        Ok(entries) => {
            let directories = entries
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.is_dir())
                .collect::<Vec<PathBuf>>();
            Ok(directories)
        }
        Err(error) => Err(error),
    }
}
