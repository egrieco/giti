# Design Overview

`giti` is intended to be a Rust crate and accompanying CLI program to query Git repos.

This is based off of a Zsh shell script that I created and used extensively. When moving from macOS to NixOS, the difference in date parsing broke the script. A Rust program shouldn't run into such issues and then we can call it from other Rust programs like the Symplasma project.

## CLI Options

The program should take an arbitrary number of args on the command line and return the requested info for each command. If no args are specified, it should default to searching for a git repo starting from the current working directory.

```man
  -r,--repo-urls    Print repo urls
  -u,--last-update  Print last update by git commit
  -f,--last-fetch   Print last fetch attempt date
  -g,--repo-size    Print git repo size
  -s,--size         Print total working dir size

  -t,--tsv          Print as TSV (Tab Separated Values)

  If no flag is specified print info for all flags (-rufgs). Multiple flags are allowed.

  -h,--help         Help (this text)
```

## Commands

Giti supports a number of subcommands. The default, when no arguments are provided, is `info`.

### Info

Print relevant info about the current repository or those specified via cli args.

Here is an example of the currnt output:

```text
Repository: /home/USER/Projects/giti
  Repo URLs:
    origin git@github.com:USER/giti.git
  Last Update: 2026-02-26 14:59:00 (a day ago)
  Last fetched: 2025-08-21 15:39:57 (6 months ago)
  Repo Size: 652.2 KB
  Total Size: 13.1 GB
```

### Clone

- Looks for URLs in the input text
- Cleans up URLs, converting them to standard repository URLs
- Clones the repositories to `~/Repos` or, if it finds an existing repo there, performs a pull on that repo

### Open

Opens URLs related to the repository in the browser. The following subcommands are available:

- `repo` (default): The main repo URL
- `code`: The same as above on GitHub
- `author`/`org`: The page for the entitiy that owns the repo
- `issues`: The issue/bug tracker
- `pulls`/`pull-requests`: Pull requests
- `wiki`: The wiki main page
- `commits`: The listing of commits for the `main` branch
- `branches`: Listing of branches
- `tags`: Listing of tags
- `releases`: Releases page

There are other pages that we may add later, but we want to keep this relatively simple for now.

This should also take a `--forge`/`--service`/`--site` arg in case the repo is available across multiple services.
