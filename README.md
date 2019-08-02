# Story Line Counter #

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
 
Roadmap
--------------------------------------
This is my first Rust project and I am enjoying it! Here is what I have in mind 
to add next.  Pull requests are welcome!

Todo for  1.0.0 release:
1. points file can be loaded to add points to any step

Bugs:
1. traversing whole branch trees instead of just the branch's commits.  Works fine when commits are squashed, does 
    not work fine when commits are merged.
1. the matcher regex could be made more robust, it matches a lot of non-stories currently. 
