
use crate::repo::parse_repo;

mod repo;

fn main() {
    let repo_path = ".";
    let branch = "master";

    let result = parse_repo(repo_path, branch);

    for diff_total in result {
        let (_, diff_total) = diff_total;
        println!("{}", diff_total.to_string());
    }
}
