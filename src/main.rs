use clap::Parser;
use color_eyre::Result;
use giti::{cli::Cli, repo::Repo};
use std::path::PathBuf;
use yansi::Paint;

fn main() {
    let mut cli = Cli::parse();

    cli.setup_defaults();

    // If no paths specified, use current directory
    let paths = if cli.paths.is_empty() {
        vec![PathBuf::from(".")]
    } else {
        cli.paths.clone()
    };

    let repos: Vec<Result<Repo>> = paths.iter().map(Repo::new).collect();

    for repo in repos {
        match repo {
            Ok(r) => {
                if cli.tsv {
                    r.print_tsv_info(&cli);
                } else {
                    r.print_human_readable_info(&cli);
                }
            }
            Err(e) => eprintln!("{}", format!("{e}").red()),
        }
    }
}
