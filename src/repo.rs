use color_eyre::Result;
use gix::{discover, remote::Direction, Remote, Repository};
use std::{borrow::Cow, path::Path};

use crate::cli::Cli;

const INDENT: &str = "  ";

pub struct Repo {
    repo: Repository,
}

impl Repo {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self> {
        let repo = discover(path)?;
        Ok(Self { repo })
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
        // get the most recent commit from any available branch and return the timestamp of when it was committed AI!
        todo!("Implement last update retrieval using gix")
    }

    pub fn last_fetch(&self) -> Cow<'_, str> {
        todo!("Implement last fetch date retrieval using gix")
    }

    pub fn repo_size(&self) -> Cow<'_, str> {
        todo!("Implement git repo size calculation using gix")
    }

    pub fn total_size(&self) -> Cow<'_, str> {
        todo!("Implement total working directory size calculation")
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
            println!("  Last Fetch: {}", self.last_fetch());
        }

        if cli.repo_size {
            println!("  Repo Size: {}", self.repo_size());
        }

        if cli.size {
            println!("  Total Size: {}", self.total_size());
        }

        println!();
    }
}
