# Finding and Validating Git URLs

There are many different git forges. Most have relatively similar URL formats with a few outliers.

[How to validate git repository url | LabEx](https://labex.io/tutorials/git-how-to-validate-git-repository-url-434201)

## Rust Crates

The [git-url-parse](https://lib.rs/crates/git-url-parse) is a parser for urls used by git. It seems like the most obvious place to start.

### Writing a custom parser

If the above doesn't work we mighht have to write our own parser. Here are some parsing crates that we might want to consider.

It's unlikely that we'll benefit from a "hand rolled" parser. Instead we should use a [#parser-combinator](https://lib.rs/keywords/parser-combinator) if we can't find an existing crate.

#### Winnow

- [winnow](https://crates.io/crates/winnow): A byte-oriented, zero-copy, parser combinators library
- [why winnow?](https://docs.rs/winnow/latest/winnow/_topic/why/index.html)

#### Chumsky

- [chumsky](https://lib.rs/crates/chumsky): A parser library for humans with powerful error recovery
- [chumsky guide](https://docs.rs/chumsky/latest/chumsky/guide/)
