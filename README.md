# giti

A tool to easily query git repos for basic information like last update, last fetch, etc.

## Usage

### Info (default)

Query repository information:

### Clone

Clone a git repository to `$HOME/Repos`:

The clone command checks for the repository URL in the following order:

1. Standard input (stdin)
2. Command line argument
3. Clipboard (if it contains a valid git URL)

If the repository already exists in `$HOME/Repos`, the command will run `git pull` to update it instead of cloning.
