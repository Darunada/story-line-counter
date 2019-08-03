# Story Line Counter #

[![Latest Version](https://img.shields.io/crates/v/story-line-counter.svg)](https://crates.io/crates/story-line-counter)
[![Rustc Version 1.36+](https://img.shields.io/badge/rustc-1.36+-lightgray.svg)](https://blog.rust-lang.org/2017/02/02/Rust-1.15.html)

Description
--------------------------------------
This utility analyzes a git repo's commit messages for Version1 or Jira story numbers 
and gets the total number of changed lines associated with a story.

Getting Started
--------------------------------------
This utility is built in Rust.  Is this your first time?  It is mine @_@

- You have Rust installed, preferably via `rustup`
- You have `~/.cargo/bin` added to your PATH variable
- Build and run in one step with `cargo run --`, or `cargo run -- --help`
- or, Build the executable with 

```
$ cargo build --release
  ... builds in a few minutes ...
$ ./target/release/story-line-counter
$ ./target/release/story-line-counter --help
$ ./target/release/story-line-counter run --help
```

At this point it's ready to use, but only the run operation is implemented.  Include the executable in your PATH and you can use it
from within any git repository simply by running `story-line-counter`.
 
Usage
--------------------------------------

```
$ story-line-counter --help

story-line-counter 1.0.0
Lea Fairbanks <lea.rae.fairbanks@gmail.com>

USAGE:
    story-line-counter [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    collect    Parses a git repo and outputs DiffCollection objects.
    help       Prints this message or the help of the given subcommand(s)
    run        Opens a repo and outputs a DiffTotalCollection.
    total      Loads a DiffCollection file and outputs a DiffTotalCollection.
```

You may collect and total a repository in one step.

```
$ story-line-counter run -p /path/to/points.json /path/to/repo
```

Or you may split it into two steps, to total multiple repos together.

```
$ story-line-counter collect /path/to/repo1 > repo1.json
$ story-line-counter collect /path/to/repo2 > repo2.json
$ story-line-counter total -p /path/to/points.json repo1.json repo2.json
```

Including a points file is optional, but you must generate your own to match story numbers with point 
values. An example points.json file is included in this repo [here](points.json). It may be pretty-printed or not.
