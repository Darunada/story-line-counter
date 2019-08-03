#[macro_use]
extern crate clap;

use crate::repo::{collect, run, total};

use crate::args_parser::{parse_collect_args, parse_total_args, CollectArgs, TotalArgs};
use crate::errors::{CliError, InputError};
use crate::repo::diff::{
    DiffCollection, DiffResult, DiffTotalCollection, Pointable, StoryPointCollection,
};
use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

mod args_parser;
mod errors;
mod repo;

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
        Arg::with_name("filepath")
            .help("The path to the git repo to scan.")
            .required(false)
            .default_value(".")
            .index(1),
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
                .args(&collect_args)
                .arg(Arg::with_name("points")
                         .help("The path to a points file to merge.")
                         .takes_value(true)
                         .required(false)
                         .long("points")
                         .short("p")),
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
                ]),
        ])
        .get_matches();

    let program_result = match matches.subcommand() {
        ("collect", Some(collect_args)) => collect_command(&parse_collect_args(collect_args)),
        ("total", Some(total_args)) => total_command(&parse_total_args(total_args)),
        ("run", collect_args) => {
            if collect_args.is_some() {
                run_command(&parse_collect_args(collect_args.unwrap()))
            } else {
                let default = CollectArgs {
                    branch: "master".to_string(),
                    matcher: "v1".to_string(),
                    path: ".".to_string(),
                    points_path: None,
                };
                run_command(&default)
            }
        },
        _ => {
            println!("Please specify a valid subcommand: collect, total, or run");
            Ok(())
        }
    };

    match program_result {
        Err(error) => {
            eprintln!("An error occurred: {}", error.to_string());
        }
        _ => (),
    }
}

fn total_command(args: &TotalArgs) -> Result<(), CliError> {
    let TotalArgs { paths, points_path } = args;

    let file_paths = match paths {
        Some(paths) => Ok(paths
            .into_iter()
            .map(|string| Path::new(string))
            .collect::<Vec<&Path>>()),
        None => Err(InputError::from(
            "You must specify at least one input file path.",
        )),
    }?;

    let mut collections: Vec<DiffCollection> = Vec::new();
    for path in file_paths {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let collection = serde_json::from_reader(reader)?;
        collections.push(collection);
    }

    let mut diff_results: Vec<DiffResult> = Vec::new();
    for collection in collections {
        collection.diffs.into_iter().for_each(|diff_result| {
            diff_results.push(diff_result);
        })
    }
    let diff_collection = DiffCollection {
        diffs: diff_results,
    };
    let mut diff_total_collection = total(diff_collection)?;
    add_points(&mut diff_total_collection, points_path)?;

    let json = serde_json::to_string(&diff_total_collection)?;
    println!("{}", json);
    Ok(())
}

fn collect_command(args: &CollectArgs) -> Result<(), CliError> {
    let CollectArgs {
        branch,
        matcher,
        path,
        points_path: _points_path,
    } = args;
    let diff_collection = collect(path, branch, matcher)?;

    let json = serde_json::to_string(&diff_collection)?;
    println!("{}", json);
    Ok(())
}

fn run_command(args: &CollectArgs) -> Result<(), CliError> {
    let CollectArgs {
        branch,
        matcher,
        path,
        points_path,
    } = args;

    let mut diff_total_collection = run(path, branch, matcher)?;
    add_points(&mut diff_total_collection, points_path)?;

    let json = serde_json::to_string(&diff_total_collection)?;
    println!("{}", json);
    Ok(())
}

fn add_points(
    diff_total_collection: &mut DiffTotalCollection,
    points_path: &Option<String>,
) -> Result<(), CliError> {
    if points_path.is_some() {
        let str_path = points_path.to_owned().unwrap();
        let path = Path::new(&str_path);
        let points_collection = load_points(path)?;
        diff_total_collection.add_points(&points_collection);
    }

    Ok(())
}

fn load_points(points_path: &Path) -> Result<StoryPointCollection, CliError> {
    let file = File::open(&points_path)?;
    let reader = BufReader::new(file);
    let points_collection = serde_json::from_reader(reader)?;
    Ok(points_collection)
}
