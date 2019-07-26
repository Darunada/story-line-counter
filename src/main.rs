use git2::{Repository, Revwalk, Oid, BranchType, Branch, Commit, Sort, DiffStats};

fn main() {
    let repository = get_repository("../../mastercontrol/");

    let branch = get_branch(&repository, "master", BranchType::Local);
    let head = match branch.into_reference().peel_to_commit() {
        Ok(commit) => commit,
        Err(error) => {
            println!("Unable to get commit for branch reference. Error: {}", error.to_string());
            panic!();
        }
    };

    let mut revwalk = repository.revwalk().unwrap();
    revwalk.push(head.id());
    revwalk.set_sorting(Sort::NONE);

    for commit in revwalk.into_iter() {
        match commit {
            Ok(oid) => {
                let commit = find_commit(&repository, &oid);
                parse_commit(&commit);
            },
            Err(err) => {
                println!("Walking error: {}", err.to_string());
                panic!();
            }
        }
    }
}

fn get_repository(path: &str) -> Repository {
    match Repository::discover(path) {
        Ok(repo) => repo,
        Err(error) => {
            println!("Unable to find repository: {}.  Error: {}", path, error.to_string());
            panic!();
        }
    }
}

fn get_branch<'repo>(repository: &'repo Repository, branch: &str, branch_type: BranchType) -> Branch<'repo> {
    match repository.find_branch(branch, branch_type) {
        Ok(branch) => branch,
        Err(error) => {
            println!("Unable to get branch for reference '{}'. Error: {}", branch, error.to_string());
            panic!();
        }
    }
}

fn find_commit<'repo>(repository: &'repo Repository, oid: &Oid) -> Commit<'repo> {
    match repository.find_commit(*oid) {
        Ok(commit) => commit,
        Err(error) => {
            println!("Oh no! {}", oid.to_string());
            panic!();
        }
    }
}

fn parse_commit(commit: &Commit) {
    // do stuff with the commit
    println!("{}", commit.summary().unwrap().to_string());
}
