use std::collections::HashMap;
use std::string::ToString;
use regex::Regex;
use git2::{Commit, Diff, DiffOptions, Error, Oid, Repository, Sort};

use crate::repo::diff::{DiffResult, DiffTotal, DiffTotalCollection, DiffCollection};
use crate::repo::core::RepoPosition;
use crate::repo::core::get_commit;

pub mod diff;
mod core;

struct OidPair(Oid, Oid);

struct CommitPair<'repo> {
    first: Commit<'repo>,
    second: Commit<'repo>,
    diff: Diff<'repo>,
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
pub fn collect_repo(repo_path: &str, branch: &str, matcher: &str) -> Result<DiffCollection, Error> {
    let repo = core::get_repository(repo_path)?;
    let repo_start = core::get_repo_head(&repo, branch)?;
    let diff_collection = collect_diffs(&repo_start, matcher)?;
    Ok(diff_collection)
}

fn collect_diffs(start: &RepoPosition, matcher: &str) -> Result<DiffCollection, Error> {
    let RepoPosition { repository, branch: _, commit } = start;

    let mut first_rev_collection = repository.revwalk()?;
    first_rev_collection.set_sorting(Sort::NONE);
    first_rev_collection.push(commit.id())?;

    let mut second_rev_collection = repository.revwalk()?;
    second_rev_collection.set_sorting(Sort::NONE);
    second_rev_collection.push(commit.id())?;

    let mut first_commit_iterator = first_rev_collection.into_iter();
    let second_commit_iterator = second_rev_collection.into_iter();
    first_commit_iterator.next();

    let mut diff_totals_sum: HashMap<String, DiffTotal> = HashMap::new();

    let result: Result<Vec<DiffResult>, Error> = first_commit_iterator
        .zip(second_commit_iterator)
        .filter(|oids| {
            let (first, second) = oids;
            first.is_ok() && second.is_ok()
        }).map(|oids| {
        let (first, second) = oids;
        OidPair(first.unwrap(), second.unwrap())
    }).filter_map(|oid_pair| {
        get_commit_pair(&repository, oid_pair)
    }).map(|commit_pair| {
        parse_commit_pair(&commit_pair, matcher)
    }).collect();

    match result {
        Ok(diffs) => Ok( DiffCollection { diffs } ),
        Err(error) => Err(error)
    }
}

fn get_commit_pair(repository: &Repository, oid_pair: OidPair) -> Option<CommitPair> {
    let mut diff_options = DiffOptions::new();

    let OidPair(first_oid, second_oid) = oid_pair;
    let first_commit = get_commit(&repository, &first_oid).unwrap();
    let second_commit = get_commit(&repository, &second_oid).unwrap();

    let diff = repository.diff_tree_to_tree(Some(&first_commit.tree().unwrap()), Some(&second_commit.tree().unwrap()), Some(&mut diff_options)).unwrap();

    Some(CommitPair {
        first: first_commit,
        second: second_commit,
        diff,
    })
}


fn parse_commit_pair(diff: &CommitPair, matcher: &str) -> Result<DiffResult, Error> {
    let CommitPair { first, second, diff } = diff;


    let first_summary = first.summary().unwrap_or("").to_string();
    let second_summary = second.summary().unwrap_or("").to_string();

    let story_number = match get_story_numbers(&second_summary, matcher) {
        Ok(story_number) => story_number,
        Err(_) => vec!["orphan".to_string()]
    };

    let diff_stats = diff.stats()?;
    let files_changed = diff_stats.files_changed();
    let insertions = diff_stats.insertions();
    let deletions = diff_stats.deletions();

    Ok(DiffResult {
        story_number,
        points: "0".to_string(),
        first_summary,
        second_summary,
        files_changed,
        insertions,
        deletions,
    })
}

pub fn calculate_diff_totals(diff_collection: &DiffCollection) -> Result<HashMap<String, DiffTotal>, Error> {
    let mut diff_totals_sum: HashMap<String, DiffTotal> = HashMap::new();

    diff_collection.diffs.iter().for_each(|diff_result| {

//        println!("{}", diff_result.to_string());
        for story_number in diff_result.story_number.iter() {
            match diff_totals_sum.get(story_number) {
                Some(diff_total) => {
                    let new_total = DiffTotal {
                        story_number: (*(diff_total).story_number).to_string(),
                        points: "0".to_string(),
                        files_changed: diff_total.files_changed + diff_result.insertions,
                        insertions: diff_total.insertions + diff_result.deletions,
                        deletions: diff_total.deletions + diff_result.files_changed,
                        total_diff_results: diff_total.total_diff_results + 1,
                    };

                    //println!("{}", new_total.to_string());
                    diff_totals_sum.insert(story_number.to_string(), new_total);
                }
                None => {
                    let new_total = DiffTotal {
                        story_number: story_number.to_string(),
                        points: "0".to_string(),
                        files_changed: diff_result.files_changed,
                        insertions: diff_result.insertions,
                        deletions: diff_result.deletions,
                        total_diff_results: 1,
                    };

                    //println!("{}", new_total.to_string());
                    diff_totals_sum.insert(story_number.to_string(), new_total);
                }
            }
        };
    });

    Ok(diff_totals_sum)
}

