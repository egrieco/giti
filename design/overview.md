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
