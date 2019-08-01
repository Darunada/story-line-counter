
#[macro_use]
extern crate clap;

use clap::{App, ArgMatches};
use crate::repo::parse_repo;

use repo::diff::DiffTotalCollection;
use core::fmt;
use std::error::Error;
use crate::args_parser::{parse_total_args, CollectArgs, TotalArgs, parse_collect_args};
use crate::repo::diff::DiffCollection;

mod repo;
mod args_parser;


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

    match matches.subcommand_name() {
        Some("collect") => {
            collect_command(&parse_collect_args(&matches))
        },
        Some("total") => {
            total_command(&parse_total_args(&matches))
        },
        Some("run") | None | _ => {
            run_command(&parse_collect_args(&matches))
        }
    }.unwrap();
}


fn total_command(args: &TotalArgs) -> Result<(), CliError> {
    let TotalArgs { path, points_path } = args;

    // TODO: get file streams from DiffCollections paths/s.

    // TODO: total all the diff collections into a DiffTotalCollection
    let diff_total_collection = total()?;

    // TODO: set points values with with points file

    Ok(())
}

fn collect_command(args: &CollectArgs) -> Result<(), CliError> {

    let CollectArgs { branch, matcher, path, points_path } = args;
    let diff_collection = collect(path, branch, matcher);

    // TODO: set points values with with points file

    match diff_collection {
        Ok(diff_collection) => {
            print!("{}", diff_collection.to_string());
            Ok(())
        },
        Err(error) => Err(error)
    }
}


fn run_command(args: &CollectArgs) -> Result<(), CliError> {

    let CollectArgs { branch, matcher, path, points_path } = args;
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


fn total() -> Result<DiffTotalCollection, CliError> {
    unimplemented!();
}

fn collect(path: &str, branch: &str, matcher: &str) -> Result<DiffCollection, CliError> {
    unimplemented!();
}

fn run(path: &str, branch: &str, matcher: &str) -> Result<DiffTotalCollection, CliError> {
    // TODO: use collect + total functions
    parse_repo(path, branch, matcher).map_err(CliError::Git)
}
