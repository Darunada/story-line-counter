use git2::{Repository, Oid, BranchType, Branch, Commit, Sort, DiffStats, DiffOptions, Index};

fn main() {
    let repo_path = "../../mastercontrol/";
    let branch = "master";
    parse_repo(repo_path, branch);

    // TODO:
}

/**
 * Loads a repo, parses the tree, and builds a map of story numbers -> diff
 */
fn parse_repo(repo_path: &str, branch: &str) {
    let repository = get_repository(repo_path).unwrap();
    let branch = get_branch(&repository, branch, BranchType::Local).unwrap();
    let head = match branch.into_reference().peel_to_commit() {
        Ok(commit) => Some(commit),
        Err(error) => {
            println!("Unable to get commit for branch reference. Error: {}", error.to_string());
            None
        }
    }.unwrap();

    let mut revwalk = repository.revwalk().unwrap();
    revwalk.set_sorting(Sort::NONE);
    revwalk.push(head.id()).unwrap_or_else(|error | {
        println!("Unable to push head revision '{}' to rev walker. Error: {}", head.id().to_string(), error);
        panic!();
    });

    revwalk.into_iter().filter_map(|oid| {
        match oid {
            Ok(oid) => {
                get_commit(&repository, &oid)
            },
            Err(error) => {
                println!("Walking error: {}", error.to_string());
                None
            }
        }
    }).for_each(|commit: Commit| parse_commit(&repository, &commit));
}

fn get_repository(path: &str) -> Option<Repository> {
    match Repository::discover(path) {
        Ok(repo) => Some(repo),
        Err(error) => {
            println!("Unable to find repository: {}.  Error: {}", path, error.to_string());
            None
        }
    }
}

fn get_branch<'repo>(repository: &'repo Repository, branch: &str, branch_type: BranchType) -> Option<Branch<'repo>> {
    match repository.find_branch(branch, branch_type) {
        Ok(branch) => Some(branch),
        Err(error) => {
            println!("Unable to get branch for reference '{}'. Error: {}", branch, error.to_string());
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

fn parse_commit<'repo>(repository: &'repo Repository, commit: &Commit) {
    // TODO: do stuff with the commit
    // get a DiffStats and use it to build a map of stories -> whatever things we want to aggregate
    println!("{}", commit.summary().unwrap().to_string());
}
