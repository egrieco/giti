use color_eyre::Result;
use gix::date::Time;
use gix::{discover, remote::Direction, Remote, Repository};
use std::{borrow::Cow, fs, path::Path};
use yansi::Color::{self, *};
use yansi::Paint;

use crate::cli::Cli;
use crate::util::format_display_time;

const INDENT: &str = "  ";

pub struct Repo {
    repo: Repository,
}

impl Repo {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self> {
        let repo = discover(path)?;
        Ok(Self { repo })
    }

    pub fn git_dir(&self) -> &Path {
        self.repo.git_dir()
    }

    pub fn work_dir(&self) -> Option<&Path> {
        self.repo.work_dir()
    }

    pub fn work_path(&self) -> String {
        match self.work_dir() {
            Some(dir) => match dir.canonicalize() {
                // return the canonicalized path
                Ok(path) => path.to_string_lossy().to_string(),
                // return the original path
                Err(_) => dir.to_string_lossy().to_string(),
            },
            // this must be a bare repository
            None => "NO WORK DIR".to_owned(),
        }
    }

    pub fn remotes(&self) -> Vec<Remote> {
        let mut remotes = Vec::new();

        for remote_name in self.repo.remote_names() {
            if let Ok(remote) = self.repo.find_remote(&*remote_name) {
                remotes.push(remote);
            }
        }

        remotes
    }

    pub fn repo_urls(&self) -> Cow<'_, str> {
        let mut urls: Vec<String> = Vec::default();

        let remotes = self.remotes();
        if remotes.is_empty() {
            urls.push(format!("{INDENT}No Remotes"));
        } else {
            urls.push(format!("{INDENT}Repo URLs:"));
            for remote in remotes {
                if let Some(url) = remote.url(Direction::Fetch) {
                    urls.push(format!(
                        "{}{} {}",
                        INDENT.repeat(2),
                        remote
                            .name()
                            .map(|n| n.as_symbol().unwrap_or_default())
                            .unwrap_or_default(),
                        url
                    ));
                }
            }
        }

        urls.join("\n").into()
    }

    pub fn last_update(&self) -> Cow<'_, str> {
        match self.get_most_recent_commit_time() {
            Ok(time) => time.into(),
            Err(_) => "Unknown".into(),
        }
    }

    fn get_most_recent_commit_time(&self) -> Result<String> {
        let mut times: Vec<Time> = Vec::default();

        // Iterate through all references to find the most recent commit
        for reference_result in self.repo.references()?.all()? {
            let reference = match reference_result {
                Ok(r) => r,
                Err(_) => continue,
            };

            let target = reference.target();
            if let Some(oid) = target.try_id() {
                if let Ok(commit) = self.repo.find_object(oid)?.try_into_commit() {
                    let commit_time = commit.time()?;
                    times.push(commit_time);
                }
            }
        }

        // times are stored as seconds since the epoch, the largest should thus be the most recent
        match times.iter().max() {
            Some(time) => {
                // TODO Claude used this method, it works but there should be a cleaner way
                let datetime =
                    std::time::UNIX_EPOCH + std::time::Duration::from_secs(time.seconds as u64);
                Ok(format_display_time(datetime))
            }
            None => Ok("NO REFERENCES FOUND".to_owned()),
        }
    }

    pub fn last_fetch(&self) -> Cow<'_, str> {
        if let Some(work_dir) = self.work_dir() {
            let git_dir = work_dir.join(".git");

            // Check for FETCH_HEAD first
            let fetch_head_path = git_dir.join("FETCH_HEAD");
            if let Ok(metadata) = fs::metadata(&fetch_head_path) {
                if let Ok(modified) = metadata.modified() {
                    return format!("{INDENT}Last fetched: {}", format_display_time(modified))
                        .into();
                }
            }

            // If FETCH_HEAD not found, check for HEAD
            let head_path = git_dir.join("HEAD");
            if let Ok(metadata) = fs::metadata(&head_path) {
                if let Ok(modified) = metadata.modified() {
                    return format!("{INDENT}Last cloned: {}", format_display_time(modified))
                        .into();
                }
            }
        }

        "Unknown".into()
    }

    pub fn repo_size(&self) -> Cow<'_, str> {
        // For bare repositories, use the repository's git directory
        match self.calculate_directory_size(self.repo.git_dir()) {
            Ok(size) => format!("{INDENT}Repo Size: {}", self.format_size(size)).into(),
            Err(_) => "Unknown".into(),
        }
    }

    pub fn total_size(&self) -> Cow<'_, str> {
        if let Some(work_dir) = self.work_dir() {
            match self.calculate_directory_size(work_dir) {
                Ok(size) => format!("{INDENT}Total Size: {}", self.format_size(size)).into(),
                Err(_) => "Unknown".into(),
            }
        } else {
            // For bare repositories, use the repository's git directory
            match self.calculate_directory_size(self.repo.git_dir()) {
                Ok(size) => format!("{INDENT}Total Size: {}", self.format_size(size)).into(),
                Err(_) => "Unknown".into(),
            }
        }
    }

    fn calculate_directory_size(&self, dir: &Path) -> Result<u64> {
        let mut total_size = 0;

        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_dir() {
                    total_size += self.calculate_directory_size(&path)?;
                } else {
                    total_size += entry.metadata()?.len();
                }
            }
        }

        Ok(total_size)
    }

    fn format_size(&self, size: u64) -> String {
        const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
        const COLOR: &[Color] = &[Blue, Green, Green, Yellow, Red];
        let mut size = size as f64;
        let mut unit_index = 0;

        while size >= 1024.0 && unit_index < UNITS.len() - 1 {
            size /= 1024.0;
            unit_index += 1;
        }

        if unit_index == 0 {
            format!("{} {}", size as u64, UNITS[unit_index])
        } else {
            format!("{:.1} {}", size, UNITS[unit_index])
        }
        .paint(COLOR[unit_index])
        .to_string()
    }

    pub fn print_tsv_info(&self, cli: &Cli) {
        let mut values = Vec::new();

        values.push(
            self.work_dir()
                .map(|r| r.to_string_lossy())
                .unwrap_or_default(),
        );

        if cli.repo_urls {
            values.push(self.repo_urls());
        }

        if cli.last_update {
            values.push(self.last_update());
        }

        if cli.last_fetch {
            values.push(self.last_fetch());
        }

        if cli.repo_size {
            values.push(self.repo_size());
        }

        if cli.size {
            values.push(self.total_size());
        }

        println!("{}", values.join("\t"));
    }

    pub fn print_human_readable_info(&self, cli: &Cli) {
        println!("Repository: {}", self.work_path());

        if cli.repo_urls {
            println!("{}", self.repo_urls());
        }

        if cli.last_update {
            println!("  Last Update: {}", self.last_update());
        }

        if cli.last_fetch {
            println!("{}", self.last_fetch());
        }

        if cli.repo_size {
            println!("{}", self.repo_size());
        }

        if cli.size {
            println!("{}", self.total_size());
        }

        println!();
    }
}
