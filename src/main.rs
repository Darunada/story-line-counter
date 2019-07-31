
#[macro_use]
extern crate clap;

use clap::App;
use crate::repo::parse_repo;

use repo::diff::DiffTotalCollection;
use core::fmt;
use std::error::Error;

mod repo;


#[derive(Debug)]
enum CliError {
    Git(git2::Error),
}

impl From<git2::Error> for CliError {
    fn from(err: git2::Error) -> CliError {
        CliError::Git(err)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Git(ref err) => err.fmt(f),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Git(ref err) => err.description(),
        }
    }
}

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let mut branch = "master";
    if let Some(requested_branch) = matches.value_of("branch") {
        branch = requested_branch;
    }
    //println!("Using branch '{}'", branch);

    let mut matcher = "v1";
    if let Some(requested_matcher) = matches.value_of("matcher") {
        matcher = requested_matcher;
    }
    //println!("Using pattern matcher '{}'", matcher);

    let mut path = ".";
    if let Some(requested_path) = matches.value_of("filepath") {
        path = requested_path;
    }
    //println!("searching path '{}'", path);

    match run(path, branch, matcher) {
        Ok(diff_total_collection) => {
            print!("{}", diff_total_collection.to_string());
        },
        Err(error) => {
            eprintln!("{}", error.to_string());
            panic!();
        }
    };
}

fn run(path: &str, branch: &str, matcher: &str) -> Result<DiffTotalCollection, CliError> {
    parse_repo(path, branch, matcher).map_err(CliError::Git)
}
