
#[macro_use]
extern crate clap;

use clap::App;
use crate::repo::parse_repo;

mod repo;

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


    let result = parse_repo(path, branch, matcher);

    for diff_total in result {
        let (_, diff_total) = diff_total;
        println!("{}", diff_total.to_string());
    }
}
