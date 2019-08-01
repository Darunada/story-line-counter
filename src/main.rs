
#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};
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

struct RunArgs<'a> {
    branch: &'a str,
    matcher: &'a str,
    path: &'a str,
    points_path: Option<&'a str>
}

struct TotalArgs<'a> {
    path: Option<&'a str>,
    points_path: Option<&'a str>
}


fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    match matches.subcommand_name() {
        Some("collect") => unimplemented!(),
        Some("total") => {
            total_command(&parse_total_args(&matches))
        },
        Some("run") | None | _ => {
            run_command(&parse_run_args(&matches))
        }
    }.unwrap();
}

fn parse_run_args<'a>(matches: &'a ArgMatches) -> RunArgs<'a> {
    let mut branch = "master";
    if let Some(requested_branch) = matches.value_of("branch") {
        branch = requested_branch;
    }

    let mut matcher = "v1";
    if let Some(requested_matcher) = matches.value_of("matcher") {
        matcher = requested_matcher;
    }

    let mut path = ".";
    if let Some(requested_path) = matches.value_of("filepath") {
        path = requested_path;
    }

    let points_path = matches.value_of("points");

    RunArgs {
        branch,
        matcher,
        path,
        points_path
    }
}

fn run_command(args: &RunArgs) -> Result<(), CliError> {

    let RunArgs { branch, matcher, path, points_path } = args;
    let diff_total_collection = run(path, branch, matcher);

    // TODO: set points values with with points file

    match diff_total_collection {
        Ok(diff_total_collection) => {
            print!("{}", diff_total_collection.to_string());
            Ok(())
        },
        Err(error) => Err(error)
    }
}

fn run(path: &str, branch: &str, matcher: &str) -> Result<DiffTotalCollection, CliError> {
    parse_repo(path, branch, matcher).map_err(CliError::Git)
}


fn parse_total_args<'a>(matches: &'a ArgMatches) -> TotalArgs<'a> {
    let path = matches.value_of("in");
    let points_path = matches.value_of("points");

    TotalArgs {
        path,
        points_path
    }
}

fn total_command(args: &TotalArgs) -> Result<(), CliError> {
    let TotalArgs { path, points_path } = args;

    // TODO: get file streams from DiffCollections paths/s.

    // TODO: total all the diff collections into a DiffTotalCollection
    let diff_total_collection = total()?;

    // TODO: set points values with with points file

    Ok(())
}

fn total() -> Result<DiffTotalCollection, CliError> {
    unimplemented!();
}