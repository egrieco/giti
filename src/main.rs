use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "git-info")]
#[command(about = "A CLI tool to get git repository information")]
struct Cli {
    /// Print repo urls
    #[arg(short = 'r', long = "repo-urls")]
    repo_urls: bool,

    /// Print last update by git commit
    #[arg(short = 'u', long = "last-update")]
    last_update: bool,

    /// Print last fetch attempt date
    #[arg(short = 'f', long = "last-fetch")]
    last_fetch: bool,

    /// Print git repo size
    #[arg(short = 'g', long = "repo-size")]
    repo_size: bool,

    /// Print total working dir size
    #[arg(short = 's', long = "size")]
    size: bool,

    /// Print as TSV (Tab Separated Values)
    #[arg(short = 't', long = "tsv")]
    tsv: bool,

    /// Repository paths to analyze (defaults to current directory)
    paths: Vec<PathBuf>,
}

fn main() {
    let cli = Cli::parse();

    // If no paths specified, use current directory
    let paths = if cli.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cli.paths.clone()
    };

    // If no flags specified, enable all info flags
    let show_all = !cli.repo_urls && !cli.last_update && !cli.last_fetch && !cli.repo_size && !cli.size;

    for path in paths {
        process_repository(&path, &cli, show_all);
    }
}

fn process_repository(path: &PathBuf, cli: &Cli, show_all: bool) {
    if cli.tsv {
        print_tsv_info(path, cli, show_all);
    } else {
        print_human_readable_info(path, cli, show_all);
    }
}

fn print_tsv_info(path: &PathBuf, cli: &Cli, show_all: bool) {
    let mut values = Vec::new();
    
    values.push(path.display().to_string());
    
    if show_all || cli.repo_urls {
        values.push(get_repo_urls(path));
    }
    
    if show_all || cli.last_update {
        values.push(get_last_update(path));
    }
    
    if show_all || cli.last_fetch {
        values.push(get_last_fetch(path));
    }
    
    if show_all || cli.repo_size {
        values.push(get_repo_size(path));
    }
    
    if show_all || cli.size {
        values.push(get_total_size(path));
    }
    
    println!("{}", values.join("\t"));
}

fn print_human_readable_info(path: &PathBuf, cli: &Cli, show_all: bool) {
    println!("Repository: {}", path.display());
    
    if show_all || cli.repo_urls {
        println!("  Repo URLs: {}", get_repo_urls(path));
    }
    
    if show_all || cli.last_update {
        println!("  Last Update: {}", get_last_update(path));
    }
    
    if show_all || cli.last_fetch {
        println!("  Last Fetch: {}", get_last_fetch(path));
    }
    
    if show_all || cli.repo_size {
        println!("  Repo Size: {}", get_repo_size(path));
    }
    
    if show_all || cli.size {
        println!("  Total Size: {}", get_total_size(path));
    }
    
    println!();
}

fn get_repo_urls(_path: &PathBuf) -> String {
    todo!("Implement repo URLs retrieval using gix")
}

fn get_last_update(_path: &PathBuf) -> String {
    todo!("Implement last update retrieval using gix")
}

fn get_last_fetch(_path: &PathBuf) -> String {
    todo!("Implement last fetch date retrieval using gix")
}

fn get_repo_size(_path: &PathBuf) -> String {
    todo!("Implement git repo size calculation using gix")
}

fn get_total_size(_path: &PathBuf) -> String {
    todo!("Implement total working directory size calculation")
}
