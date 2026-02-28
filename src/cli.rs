use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "giti")]
#[command(
    author,
    version,
    about = "A CLI tool to get git repository information"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Clone a git repository to $HOME/Repos
    Clone {
        /// Repository URL to clone (can also be provided via stdin or clipboard)
        url: Option<String>,
    },
    /// Show repository information (default behavior)
    Info(InfoArgs),
    /// Open repository URLs in the browser
    Open(OpenArgs),
}

#[derive(Parser, Default)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "Many bools make sense for CLI flags"
)]
pub struct InfoArgs {
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

impl InfoArgs {
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

#[derive(Parser)]
pub struct OpenArgs {
    /// The page to open
    #[arg(value_enum, default_value = "repo")]
    pub page: OpenPage,

    /// Override the forge/service to use (e.g., github, gitlab, bitbucket)
    #[arg(long = "forge", visible_alias = "service", visible_alias = "site")]
    pub forge: Option<String>,

    /// Remote name to use (defaults to "origin")
    #[arg(short = 'r', long = "remote", default_value = "origin")]
    pub remote: String,

    /// Repository path (defaults to current directory)
    pub path: Option<PathBuf>,
}

#[derive(ValueEnum, Clone, Default, Debug)]
pub enum OpenPage {
    /// The main repository page
    #[default]
    Repo,
    /// The code/source page (same as repo on most forges)
    Code,
    /// The author/organization page
    #[value(alias = "org")]
    Author,
    /// The issues/bug tracker page
    Issues,
    /// The pull requests page
    #[value(alias = "pull-requests", alias = "prs", alias = "mrs")]
    Pulls,
    /// The wiki page
    Wiki,
    /// The commits listing
    Commits,
    /// The branches listing
    Branches,
    /// The tags listing
    Tags,
    /// The releases page
    Releases,
}
