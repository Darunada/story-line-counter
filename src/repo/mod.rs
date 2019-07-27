use std::collections::HashMap;
use std::string::ToString;
use regex::Regex;

use git2::{Branch, BranchType, Commit, Diff, DiffOptions, Error, Oid, Repository, Sort};

use crate::repo::diff::{DiffResult, DiffTotal};

mod diff;

struct OidPair(Oid, Oid);

struct CommitPair<'repo> {
    first: Commit<'repo>,
    second: Commit<'repo>,
    diff: Diff<'repo>
}


fn get_story_numbers(summary: &str, matcher: &str) -> Result<Vec<String>, Error> {


    let regex = match matcher {
        // matches 'SO-123', 'VEN-444'
        "jira" => Regex::new(r"([A-Za-z]+)[\s\-]*(\d+)").unwrap(),

        // matches 's-10345', 's 10345', 'd-10345', 'd 10345', 'S-10345', 'S 10345',
        // 'D-10345', 'D 10345', 's- 10345', 'S -10345', 'd  - 10345', 'D -  10345'
        _ => Regex::new(r"([sdSD])[\s\-]*(\d{5})").unwrap()
    };

    let mut story_numbers = Vec::new();

    for cap in regex.captures_iter(summary) {
        // 0 refers to the entire match
        let story_type = &cap[1];
        let story_number = &cap[2];
        story_numbers.push(format!("{}-{}", story_type.to_uppercase(), story_number));
    }

    Ok(story_numbers)

}

// Loads a repo, parses the tree, and builds a map of story numbers -> diff
pub fn parse_repo(repo_path: &str, branch: &str, matcher: &str) -> HashMap<String, DiffTotal> {
    let repository = get_repository(repo_path).unwrap_or_else(|| {
        eprintln!("Unable to find repository at {}", repo_path);
        panic!()
    });

    println!("Found repository at {}", repository.path().to_str().unwrap());
    println!("Checking for branch {}", branch);

    let branch_ = get_branch(&repository, branch, BranchType::Local).unwrap_or_else(|| {
        eprintln!("Unable to find branch {} in repo.  Does it exist?", branch);
        panic!()
    });

    println!("Found branch {}", branch_.name().unwrap_or_else(|error| {
        eprintln!("There was a problem fetching branch {} in repo.  Error: {}", branch, error.to_string());
        panic!()
    }).unwrap_or_else(|| {
        eprintln!("Unable to find branch {} in repo.  Does it exist?  Error: ", branch);
        panic!()
    }));


    let head = match branch_.into_reference().peel_to_commit() {
        Ok(commit) => commit,
        Err(error) => {
            eprintln!("There was a problem fetching the head of branch {}. Error: {}", branch, error);
            panic!();
        }
    };


    println!("Got branch head {}.  Traversing...", head.id());

    calculate_diff_totals(&repository, head, matcher)
}

fn get_commit_pair(repository: &Repository, oid_pair: OidPair) -> Option<CommitPair> {
    let mut diff_options = DiffOptions::new();

    let OidPair(first_oid, second_oid) = oid_pair;
    let first_commit = get_commit(&repository, &first_oid).unwrap();
    let second_commit= get_commit(&repository, &second_oid).unwrap();

    let diff = repository.diff_tree_to_tree(Some(&first_commit.tree().unwrap()), Some(&second_commit.tree().unwrap()), Some(&mut diff_options)).unwrap();


    Some(CommitPair {
        first: first_commit,
        second: second_commit,
        diff
    })
}


fn get_repository(path: &str) -> Option<Repository> {
    match Repository::discover(path) {
        Ok(repo) => Some(repo),
        Err(error) => {
            println!("Unable to find repository: {}.  Error: {}", path, error);
            None
        }
    }
}

fn get_branch<'repo>(repository: &'repo Repository, branch: &str, branch_type: BranchType) -> Option<Branch<'repo>> {
    match repository.find_branch(branch, branch_type) {
        Ok(branch) => Some(branch),
        Err(error) => {
            println!("Unable to get branch for reference '{}'. Error: {}", branch, error);
            None
        }
    }
}

fn get_commit<'repo>(repository: &'repo Repository, oid: &Oid) -> Option<Commit<'repo>> {
    match repository.find_commit(*oid) {
        Ok(commit) => Some(commit),
        Err(error) => {
            println!("Unable to find commit for oid '{}'.  Error: {}", oid.to_string(), error);
            None
        }
    }
}

fn parse_commit_pair(diff: &CommitPair, matcher: &str) -> Option<DiffResult> {
    let CommitPair { first, second, diff } = diff;


    let first_summary = first.summary().unwrap().to_string();
    let second_summary = second.summary().unwrap().to_string();

    let story_number = match get_story_numbers(&second_summary, matcher) {
        Ok(story_number) => story_number,
        Err(_) => vec!["orphan".to_string()]
    };

    let diff_stats = diff.stats().unwrap();
    let files_changed = diff_stats.files_changed();
    let insertions = diff_stats.insertions();
    let deletions = diff_stats.deletions();

    Some(DiffResult { story_number, first_summary, second_summary, files_changed, insertions, deletions })
}

fn calculate_diff_totals(repository: &Repository, head: Commit, matcher: &str) -> HashMap<String, DiffTotal> {

    let mut first_rev_collection = repository.revwalk().unwrap();
    first_rev_collection.set_sorting(Sort::NONE);
    first_rev_collection.push(head.id()).unwrap_or_else(|error | {
        println!("Unable to push head revision '{}' to rev walker. Error: {}", head.id().to_string(), error);
        panic!();
    });

    let mut second_rev_collection = repository.revwalk().unwrap();
    second_rev_collection.set_sorting(Sort::NONE);
    second_rev_collection.push(head.id()).unwrap_or_else(|error | {
        println!("Unable to push head revision '{}' to rev walker. Error: {}", head.id().to_string(), error);
        panic!();
    });

    let mut first_commit_iterator = first_rev_collection.into_iter();
    let second_commit_iterator = second_rev_collection.into_iter();
    first_commit_iterator.next();


    let mut diff_totals_sum: HashMap<String, DiffTotal> = HashMap::new();

    first_commit_iterator // committerator, if you will
        .zip(second_commit_iterator)
        .filter(|oids| {

            let (first, second) = oids;
            first.is_ok() && second.is_ok()

        }).map(|oids| {

        let (first, second) = oids;
        OidPair(first.unwrap(), second.unwrap())

    }).filter_map(|oid_pair| {

        get_commit_pair(&repository, oid_pair)

    }).filter_map(|commit_pair| {

        parse_commit_pair(&commit_pair, matcher)

    }).for_each(|diff_result| {

//        println!("{}", diff_result.to_string());
        for story_number in diff_result.story_number.iter() {

            let diff_total = diff_totals_sum.get(story_number);

            if diff_total.is_some() {
                let diff_total = diff_total.unwrap();
                let new_total = DiffTotal {
                    story_number: (&diff_total.story_number).to_string(),
                    files_changed: &diff_total.files_changed + diff_result.insertions,
                    insertions: diff_total.insertions + diff_result.deletions,
                    deletions: diff_total.deletions + diff_result.files_changed,
                    total_diff_results: diff_total.total_diff_results + 1
                };

//                println!("{}", new_total.to_string());
                diff_totals_sum.insert(story_number.to_string(), new_total);

            } else {
                let new_total = DiffTotal {
                    story_number: story_number.to_string(),
                    files_changed: diff_result.files_changed,
                    insertions: diff_result.insertions,
                    deletions: diff_result.deletions,
                    total_diff_results: 1
                };

//                println!("{}", new_total.to_string());
                diff_totals_sum.insert(story_number.to_string(), new_total);
            }
        };
    });

    diff_totals_sum
}

