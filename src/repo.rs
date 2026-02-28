use color_eyre::Result;
use gix::date::Time;
use gix::{discover, remote::Direction, Remote, Repository};
use std::{borrow::Cow, fs, path::Path};
use yansi::Color::{self, *};
use yansi::Paint;

use crate::cli::{InfoArgs, OpenPage};
use crate::util::calculate_directory_size;
use crate::util::format_display_time;

const INDENT: &str = "  ";

/// Represents a parsed git remote URL with owner and repo name
#[derive(Debug, Clone)]
pub struct ParsedRemoteUrl {
    pub host: String,
    pub owner: String,
    pub repo: String,
}

impl ParsedRemoteUrl {
    /// Parse a git remote URL into its components
    ///
    /// Supports various formats:
    /// - https://github.com/owner/repo.git
    /// - https://github.com/owner/repo
    /// - git@github.com:owner/repo.git
    /// - git@github.com:owner/repo
    /// - ssh://git@github.com/owner/repo.git
    pub fn parse(url: &str) -> Result<Self> {
        let url = url.trim();

        // Handle SSH URLs like git@github.com:owner/repo.git
        if url.starts_with("git@") {
            return Self::parse_ssh_url(url);
        }

        // Handle ssh:// URLs like ssh://git@github.com/owner/repo.git
        if url.starts_with("ssh://") {
            return Self::parse_ssh_protocol_url(url);
        }

        // Handle HTTPS/HTTP URLs
        if url.starts_with("https://") || url.starts_with("http://") || url.starts_with("git://") {
            return Self::parse_https_url(url);
        }

        Err(color_eyre::eyre::eyre!("Unsupported URL format: {}", url))
    }

    fn parse_ssh_url(url: &str) -> Result<Self> {
        // git@github.com:owner/repo.git
        let without_prefix = url.strip_prefix("git@").ok_or_else(|| {
            color_eyre::eyre::eyre!("Invalid SSH URL format")
        })?;

        let (host, path) = without_prefix.split_once(':').ok_or_else(|| {
            color_eyre::eyre::eyre!("Invalid SSH URL format: missing ':'")
        })?;

        Self::parse_owner_repo(host, path)
    }

    fn parse_ssh_protocol_url(url: &str) -> Result<Self> {
        // ssh://git@github.com/owner/repo.git
        let without_prefix = url.strip_prefix("ssh://git@").ok_or_else(|| {
            color_eyre::eyre::eyre!("Invalid SSH protocol URL format")
        })?;

        let (host, path) = without_prefix.split_once('/').ok_or_else(|| {
            color_eyre::eyre::eyre!("Invalid SSH protocol URL format: missing '/'")
        })?;

        Self::parse_owner_repo(host, path)
    }

    fn parse_https_url(url: &str) -> Result<Self> {
        // https://github.com/owner/repo.git
        let parsed = url::Url::parse(url)?;

        let host = parsed.host_str().ok_or_else(|| {
            color_eyre::eyre::eyre!("URL has no host")
        })?.to_string();

        let path = parsed.path().trim_start_matches('/');

        Self::parse_owner_repo(&host, path)
    }

    fn parse_owner_repo(host: &str, path: &str) -> Result<Self> {
        let path = path.trim_end_matches('/');
        let path = path.strip_suffix(".git").unwrap_or(path);

        let parts: Vec<&str> = path.split('/').collect();

        if parts.len() < 2 {
            return Err(color_eyre::eyre::eyre!(
                "Could not extract owner/repo from path: {}",
                path
            ));
        }

        Ok(Self {
            host: host.to_string(),
            owner: parts[0].to_string(),
            repo: parts[1].to_string(),
        })
    }

    /// Get the base web URL for this repository
    pub fn base_url(&self) -> String {
        format!("https://{}/{}/{}", self.host, self.owner, self.repo)
    }

    /// Get the URL for a specific page on the forge
    pub fn page_url(&self, page: &OpenPage, forge_override: Option<&str>) -> String {
        let host = forge_override.map_or_else(
            || self.host.as_str(),
            |f| match f.to_lowercase().as_str() {
                "github" | "gh" => "github.com",
                "gitlab" | "gl" => "gitlab.com",
                "bitbucket" | "bb" => "bitbucket.org",
                "codeberg" | "cb" => "codeberg.org",
                "sourcehut" | "sr" | "srht" => "sr.ht",
                other => other,
            },
        );

        let base = format!("https://{}/{}/{}", host, self.owner, self.repo);

        // Determine the forge type for URL construction
        let forge_type = self.detect_forge_type(host);

        match page {
            OpenPage::Repo | OpenPage::Code => base,
            OpenPage::Author => format!("https://{}/{}", host, self.owner),
            OpenPage::Issues => match forge_type {
                ForgeType::Bitbucket => format!("{}/issues", base),
                ForgeType::SourceHut => format!("https://todo.sr.ht/~{}/{}", self.owner, self.repo),
                _ => format!("{}/issues", base),
            },
            OpenPage::Pulls => match forge_type {
                ForgeType::GitLab => format!("{}/-/merge_requests", base),
                ForgeType::Bitbucket => format!("{}/pull-requests", base),
                ForgeType::SourceHut => format!("https://lists.sr.ht/~{}/{}", self.owner, self.repo),
                _ => format!("{}/pulls", base),
            },
            OpenPage::Wiki => match forge_type {
                ForgeType::GitLab => format!("{}/-/wikis/home", base),
                ForgeType::Bitbucket => format!("{}/wiki", base),
                ForgeType::SourceHut => base, // SourceHut doesn't have wikis
                _ => format!("{}/wiki", base),
            },
            OpenPage::Commits => match forge_type {
                ForgeType::GitLab => format!("{}/-/commits/main", base),
                ForgeType::Bitbucket => format!("{}/commits", base),
                ForgeType::SourceHut => format!("{}/log", base),
                _ => format!("{}/commits/main", base),
            },
            OpenPage::Branches => match forge_type {
                ForgeType::GitLab => format!("{}/-/branches", base),
                ForgeType::Bitbucket => format!("{}/branches", base),
                ForgeType::SourceHut => format!("{}/refs", base),
                _ => format!("{}/branches", base),
            },
            OpenPage::Tags => match forge_type {
                ForgeType::GitLab => format!("{}/-/tags", base),
                ForgeType::Bitbucket => format!("{}/downloads/?tab=tags", base),
                ForgeType::SourceHut => format!("{}/refs", base),
                _ => format!("{}/tags", base),
            },
            OpenPage::Releases => match forge_type {
                ForgeType::GitLab => format!("{}/-/releases", base),
                ForgeType::Bitbucket => format!("{}/downloads", base),
                ForgeType::SourceHut => format!("{}/refs", base),
                _ => format!("{}/releases", base),
            },
        }
    }

    fn detect_forge_type(&self, host: &str) -> ForgeType {
        let host_lower = host.to_lowercase();
        if host_lower.contains("github") {
            ForgeType::GitHub
        } else if host_lower.contains("gitlab") {
            ForgeType::GitLab
        } else if host_lower.contains("bitbucket") {
            ForgeType::Bitbucket
        } else if host_lower.contains("codeberg") {
            ForgeType::Codeberg
        } else if host_lower.contains("sr.ht") || host_lower.contains("sourcehut") {
            ForgeType::SourceHut
        } else {
            // Default to GitHub-style URLs for unknown forges
            ForgeType::GitHub
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ForgeType {
    GitHub,
    GitLab,
    Bitbucket,
    Codeberg,
    SourceHut,
}

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
        self.repo.workdir()
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

    pub fn remotes(&'_ self) -> Vec<Remote<'_>> {
        let mut remotes = Vec::new();

        for remote_name in self.repo.remote_names() {
            if let Ok(remote) = self.repo.find_remote(&*remote_name) {
                remotes.push(remote);
            }
        }

        remotes
    }

    /// Get the URL for a specific remote
    pub fn get_remote_url(&self, remote_name: &str) -> Result<String> {
        let remote = self.repo.find_remote(remote_name).map_err(|_| {
            color_eyre::eyre::eyre!("Remote '{}' not found", remote_name)
        })?;

        let url = remote.url(Direction::Fetch).ok_or_else(|| {
            color_eyre::eyre::eyre!("Remote '{}' has no fetch URL", remote_name)
        })?;

        Ok(url.to_string())
    }

    /// Get the web URL for a specific page
    pub fn get_web_url(
        &self,
        remote_name: &str,
        page: &OpenPage,
        forge_override: Option<&str>,
    ) -> Result<String> {
        let remote_url = self.get_remote_url(remote_name)?;
        let parsed = ParsedRemoteUrl::parse(&remote_url)?;
        Ok(parsed.page_url(page, forge_override))
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
        match calculate_directory_size(self.repo.git_dir()) {
            Ok(size) => format!("{INDENT}Repo Size: {}", self.format_size(size)).into(),
            Err(_) => "Unknown".into(),
        }
    }

    pub fn total_size(&self) -> Cow<'_, str> {
        if let Some(work_dir) = self.work_dir() {
            match calculate_directory_size(work_dir) {
                Ok(size) => format!("{INDENT}Total Size: {}", self.format_size(size)).into(),
                Err(_) => "Unknown".into(),
            }
        } else {
            // For bare repositories, use the repository's git directory
            match calculate_directory_size(self.repo.git_dir()) {
                Ok(size) => format!("{INDENT}Total Size: {}", self.format_size(size)).into(),
                Err(_) => "Unknown".into(),
            }
        }
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

    pub fn print_tsv_info(&self, args: &InfoArgs) {
        let mut values = Vec::new();

        values.push(
            self.work_dir()
                .map(|r| r.to_string_lossy())
                .unwrap_or_default(),
        );

        if args.repo_urls {
            values.push(self.repo_urls());
        }

        if args.last_update {
            values.push(self.last_update());
        }

        if args.last_fetch {
            values.push(self.last_fetch());
        }

        if args.repo_size {
            values.push(self.repo_size());
        }

        if args.size {
            values.push(self.total_size());
        }

        println!("{}", values.join("\t"));
    }

    pub fn print_human_readable_info(&self,args: &InfoArgs) {
        println!("Repository: {}", self.work_path());

        if args.repo_urls {
            println!("{}", self.repo_urls());
        }

        if args.last_update {
            println!("  Last Update: {}", self.last_update());
        }

        if args.last_fetch {
            println!("{}", self.last_fetch());
        }

        if args.repo_size {
            println!("{}", self.repo_size());
        }

        if args.size {
            println!("{}", self.total_size());
        }

        println!();
    }
}
