use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "giti")]
#[command(
    author,
    version,
    about = "A CLI tool to get git repository information"
)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Many bools make sense for CLI flags"
)]
pub struct Cli {
    /// Print repo urls
    #[arg(short = 'r', long = "repo-urls")]
    pub repo_urls: bool,

    /// Print last update by git commit
    #[arg(short = 'u', long = "last-update")]
    pub last_update: bool,

    /// Print last fetch attempt date
    #[arg(short = 'f', long = "last-fetch")]
    pub last_fetch: bool,

    /// Print git repo size
    #[arg(short = 'g', long = "repo-size")]
    pub repo_size: bool,

    /// Print total working dir size
    #[arg(short = 's', long = "size")]
    pub size: bool,

    /// Print as TSV (Tab Separated Values)
    #[arg(short = 't', long = "tsv")]
    pub tsv: bool,

    /// Repository paths to analyze (defaults to current directory)
    pub paths: Vec<PathBuf>,
}

impl Cli {
    pub fn setup_defaults(&mut self) {
        // If no flags specified, enable all info flags
        if !self.repo_urls && !self.last_update && !self.last_fetch && !self.repo_size && !self.size
        {
            self.repo_urls = true;
            self.last_update = true;
            self.last_fetch = true;
            self.repo_size = true;
            self.size = true;
        }
    }
}
