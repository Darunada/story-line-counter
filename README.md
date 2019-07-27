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

```

At this point it's ready to use.  Include the executable in your PATH and you can use it 
from within any git repository simply by running `story-line-counter`.
 
Usage
--------------------------------------
```
USAGE:
    story-line-counter [OPTIONS] [filepath]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --branch <branch>      The branch name to scan. Default: master
    -m, --matcher <matcher>    The story number regex to use. [v1, jira] Default: v1

ARGS:
    <filepath>    The path to the git repo to scan. Default: .
```
 
Roadmap
--------------------------------------
This is my first Rust project and I am enjoying it! Here is what I have in mind 
to add next, but I am not sure I will continue much beyond now.  Pull requests are welcome!

1. figure out how to make the -v verbose flag work
2. scan multiple repos 
3. or, the ability to sum a file? of DiffResults to DiffTotals in another operation.
4. make matcher regex able to be a parameter
5. ???