
#[macro_use]
extern crate clap;

use clap::{App, ArgMatches, Arg, SubCommand};
use crate::repo::{collect_repo, calculate_diff_totals};

use repo::diff::DiffTotalCollection;
use core::fmt;
use crate::args_parser::{parse_total_args, CollectArgs, TotalArgs, parse_collect_args};
use crate::repo::diff::DiffCollection;
use std::error::Error;
use std::path::Path;
use std::{error, io};
use std::io::LineWriter;

mod repo;
mod args_parser;


trait New {
    fn new(description: &'static str) -> Self;
}

#[derive(Debug, Clone)]
struct InputError {
    description: String
}

impl New for InputError {
    fn new(description: &'static str) -> Self {
        InputError {
            description: description.to_string()
        }
    }
}


impl Error for InputError {
    fn description(&self) -> &str {
        self.description()
    }
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)?;
        Ok(())
    }
}


#[derive(Debug)]
enum CliError {
    Git(git2::Error),
    IO(std::io::Error),
    Input(InputError),
}

impl From<git2::Error> for CliError {
    fn from(err: git2::Error) -> CliError {
        CliError::Git(err)
    }
}


impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> CliError {
        CliError::IO(err)
    }
}


impl From<InputError> for CliError {
    fn from(err: InputError) -> CliError {
        CliError::Input(err)
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Git(ref err) => err.fmt(f),
            CliError::IO(ref err) => err.fmt(f),
            CliError::Input(ref err) => err.fmt(f),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Git(ref err) => err.description(),
            CliError::IO(ref err) => err.description(),
            CliError::Input(ref err) => err.description(),
        }
    }
}


fn main() {
    let collect_args = [
        Arg::with_name("branch")
            .help("The branch name to scan.")
            .default_value("master")
            .long("branch")
            .short("b"),

        Arg::with_name("matcher")
            .help("The issue tracker counting schema to use.")
            .possible_values(&["v1", "jira"])
            .default_value("v1")
            .long("matcher")
            .short("m"),

        Arg::with_name("points")
            .help("The path to a points file to merge.")
            .takes_value(true)
            .required(false)
            .long("points")
            .short("p"),

        Arg::with_name("filepath")
            .help("The path to the git repo to scan.")
            .required(false)
            .default_value(".")
            .index(1)
    ];

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .subcommands(vec![
            SubCommand::with_name("collect")
                .version(crate_version!())
                .author(crate_authors!())
                .about("Parses a git repo and outputs DiffCollection objects.")
                .args(&collect_args),

            SubCommand::with_name("run")
                .version(crate_version!())
                .author(crate_authors!())
                .about("Opens a repo and outputs a DiffTotalCollection.")
                .args(&collect_args),

            SubCommand::with_name("total")
                .version(crate_version!())
                .author(crate_authors!())
                .about("Loads a DiffCollection file and outputs a DiffTotalCollection.")
                .args(&[
                    Arg::with_name("in")
                        .help("The input file to use.")
                        .takes_value(true)
                        .required(false)
                        .multiple(true),

                    Arg::with_name("points")
                        .help("The path to a points file to merge.")
                        .takes_value(true)
                        .required(false)
                        .long("points")
                        .short("p"),
                ])
    ]).get_matches();

    let program_result = match matches.subcommand() {
        ("collect", Some(collect_args)) => {
            collect_command(&parse_collect_args(collect_args))
        },
        ("total", Some(total_args)) => {
            total_command(&parse_total_args(total_args))
        },
        (_, collect_args) => {
            if collect_args.is_some() {
                run_command(&parse_collect_args(collect_args.unwrap()))
            } else {
                let default = CollectArgs {
                    branch: "master".to_string(),
                    matcher: "v1".to_string(),
                    path: ".".to_string(),
                    points_path: None
                };
                run_command(&default)
            }
        },
    };

    match program_result {
        Err(error) => {
            eprintln!("An error occurred: {}", error.to_string());
        },
        _ => ()
    }
}


fn total_command(args: &TotalArgs) -> Result<(), CliError> {
    let TotalArgs { paths, points_path } = args;

    let file_paths = match paths {
        Some(paths) => {
            Ok(paths.into_iter()
                .map(|string| Path::new(string) )
                .collect::<Vec<&Path>>())
        },
        None => Err(InputError::new("You must specify at least one input file path."))
    }?;

    for path in file_paths {
        println!("{}", path.display());
    }

    unimplemented!();

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
    collect_repo(path, branch, matcher).map_err(CliError::Git)
}

fn run(path: &str, branch: &str, matcher: &str) -> Result<DiffTotalCollection, CliError> {
    let collection = collect_repo(path, branch, matcher).map_err(CliError::Git)?;
    let totals = calculate_diff_totals(&collection).map_err(CliError::Git)?;
    Ok( DiffTotalCollection { totals } )
}
