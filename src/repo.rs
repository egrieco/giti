use color_eyre::Result;
use gix::{discover, Repository};
use std::{borrow::Cow, path::Path};

use crate::cli::Cli;

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

    pub fn repo_urls(&self) -> Vec<String> {
        // pull the fetch urls from the remotes via the gix repo AI!
        todo!("Implement repo URLs retrieval using gix")
    }

    pub fn last_update(&self) -> Cow<'_, str> {
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
        println!(
            "Repository: {}",
            self.work_dir()
                .map(|r| r.to_string_lossy())
                .unwrap_or_default()
        );

        if cli.repo_urls {
            println!("  Repo URLs: {}", self.repo_urls());
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
