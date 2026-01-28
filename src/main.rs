use arboard::Clipboard;
use clap::Parser;
use color_eyre::Result;
use giti::{
    cli::{Cli, Commands, InfoArgs},
    repo::Repo,
};
use linkify::{LinkFinder, LinkKind};
use std::io::{self, BufRead, IsTerminal};
use std::path::PathBuf;
use std::process::Command;
use url::{ParseError, Url};
use yansi::Paint;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Clone { url }) => {
            if let Err(e) = handle_clone(url) {
                eprintln!("{}", format!("Clone failed: {e}").red());
                std::process::exit(1);
            }
        }
        Some(Commands::Info(mut args)) => {
            handle_info(&mut args);
        }
        None => {
            // Default behavior: treat as info command with no args
            let mut args = InfoArgs::default();
            handle_info(&mut args);
        }
    }
}

fn handle_info(args: &mut InfoArgs) {
    args.setup_defaults();

    // If no paths specified, use current directory
    let paths = if args.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        args.paths.clone()
    };

    let repos: Vec<Result<Repo>> = paths.iter().map(Repo::new).collect();

    for repo in repos {
        match repo {
            Ok(r) => {
                if args.tsv {
                    r.print_tsv_info(args);
                } else {
                    r.print_human_readable_info(args);
                }
            }
            Err(e) => eprintln!("{}", format!("{e}").red()),
        }
    }
}

fn handle_clone(url: Option<String>) -> Result<()> {
    let input_text = dbg!(get_input_text(url)?);
    let extracted_urls = extract_urls(&input_text);
    eprintln!("Found {} urls in input.", extracted_urls.len());
    for url in extracted_urls {
        match url {
            Ok(repo_url) => {
                println!("Cloning url: {}...", repo_url);
                clone_repo(repo_url)?
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}

fn clone_repo(repo_url: Url) -> Result<()> {
    // Validate URL
    if !is_valid_git_url(&repo_url.to_string()) {
        return Err(color_eyre::eyre::eyre!("Invalid git URL: {}", repo_url));
    }

    // Get $HOME/Repos directory
    let home = std::env::var("HOME")
        .map_err(|_| color_eyre::eyre::eyre!("HOME environment variable not set"))?;
    let repos_dir = PathBuf::from(home).join("Repos");

    // Create Repos directory if it doesn't exist
    if !repos_dir.exists() {
        std::fs::create_dir_all(&repos_dir)?;
        println!("Created directory: {}", repos_dir.display());
    }

    // Extract repository name from URL
    let repo_name = extract_repo_name(&repo_url.to_string())?;
    let dest_path = repos_dir.join(&repo_name);

    // Check if repository already exists
    if dest_path.exists() && dest_path.join(".git").exists() {
        println!(
            "Repository already exists at {}, pulling latest changes...",
            dest_path.display()
        );

        let status = Command::new("git")
            .arg("pull")
            .current_dir(&dest_path)
            .status()?;

        if status.success() {
            println!("{}", "Pull successful!".green());
            Ok(())
        } else {
            Err(color_eyre::eyre::eyre!(
                "git pull exited with status: {}",
                status
            ))
        }
    } else {
        println!("Cloning {} to {}", repo_url, repos_dir.display());

        // Run git clone
        let status = Command::new("git")
            .arg("clone")
            .arg(clean_git_url(&repo_url)?)
            .current_dir(&repos_dir)
            .status()?;

        if status.success() {
            println!("{}", "Clone successful!".green());
            Ok(())
        } else {
            Err(color_eyre::eyre::eyre!(
                "git clone exited with status: {}",
                status
            ))
        }
    }
}

fn extract_repo_name(url: &str) -> Result<String> {
    // Handle various URL formats:
    // https://github.com/user/repo.git
    // https://github.com/user/repo
    // git@github.com:user/repo.git
    // git@github.com:user/repo
    // ssh://git@github.com/user/repo.git

    let url = url.trim_end_matches('/');
    let url = url.strip_suffix(".git").unwrap_or(url);

    // Get the last path component
    let name = url
        .rsplit('/')
        .next()
        .or_else(|| url.rsplit(':').next())
        .ok_or_else(|| color_eyre::eyre::eyre!("Could not extract repository name from URL"))?;

    if name.is_empty() {
        return Err(color_eyre::eyre::eyre!(
            "Could not extract repository name from URL"
        ));
    }

    Ok(name.to_string())
}

fn get_input_text(url: Option<String>) -> Result<String> {
    // Priority 1: Check stdin (if not a terminal)
    if !io::stdin().is_terminal() {
        let stdin = io::stdin();
        if let Some(Ok(line)) = stdin.lock().lines().next() {
            let trimmed = line.trim().to_string();
            if !trimmed.is_empty() {
                println!("Using STDIN source");
                return Ok(trimmed);
            }
        }
    }

    // Priority 2: Use CLI argument if provided
    if let Some(u) = url {
        println!("Using Arg source");
        return Ok(u);
    }

    // Priority 3: Try clipboard
    let mut clipboard = Clipboard::new()?;
    let text = clipboard.get_text()?;
    let trimmed = text.trim().to_string();
    println!("Using Clipboard source");
    Ok(trimmed)

    // Err(color_eyre::eyre::eyre!(
    //     "No repository URL provided. Provide via stdin, argument, or copy a valid URL to clipboard."
    // ))
}

/// Extract URLs from a string
///
/// We'll return a Vec of Results so that we can potentially inform users of errors.
fn extract_urls(text: &str) -> Vec<Result<Url, ParseError>> {
    let mut finder = LinkFinder::new();
    finder.kinds(&[LinkKind::Url]);
    finder.links(text).map(|l| Url::parse(l.as_str())).collect()
}

fn is_valid_git_url(url: &str) -> bool {
    // Check for common git URL patterns
    url.starts_with("https://")
        || url.starts_with("http://")
        || url.starts_with("git://")
        || url.starts_with("git@")
        || url.starts_with("ssh://")
        || url.ends_with(".git")
}

/// Clean extra path path segments, anchors and other cruft
///
/// We need to clean these off so that Git will be able to clone from this url.
fn clean_git_url(url: &Url) -> Result<String> {
    let mut url = url.clone();

    // Retrieve all segments as a Vec of strings.
    let segments = url
        .path_segments()
        .ok_or_else(|| color_eyre::eyre::eyre!("Cannot be a base URL"))?
        .map(|s| s.to_owned());

    // Count the segments.
    let segments_count: usize = segments.count();

    if segments_count > 2 {
        // Get a mutable view of the path segments.
        let mut segs_mut = url
            .path_segments_mut()
            .map_err(|_| color_eyre::eyre::eyre!("Cannot be a base URL"))?;

        // Pop all segments except two
        for _ in 0..(segments_count - 2) {
            segs_mut.pop();
        }
    }

    // Remove the query string
    url.set_query(None);

    Ok(url.to_string())
}
