
use git2::{Branch, BranchType, Commit, Diff, DiffOptions, Error, Oid, Repository, Sort, ResetType};

pub struct RepoPosition<'repo> {
    pub repository: &'repo Repository,
    pub branch: Branch<'repo>,
    pub commit: Commit<'repo>
}

fn get_repository(path: &str) -> Result<Repository, Error> {
    Repository::discover(path)
}

fn get_branch<'repo>(repository: &'repo Repository, branch: &str, branch_type: BranchType) -> Result<Branch<'repo>, Error> {
    repository.find_branch(branch, branch_type)
}

pub fn get_commit<'repo>(repository: &'repo Repository, oid: &Oid) -> Result<Commit<'repo>, Error> {
    repository.find_commit(*oid)
}

pub fn get_repo_head<'repo>(repo_path: &str, branch_name: &str) -> Result<RepoPosition<'repo>, Error> {
    let repository = get_repository(repo_path)?;
    let branch = get_branch(&repository, branch_name, BranchType::Local)?;
    let head = branch.into_reference().peel_to_commit()?;

    Ok(RepoPosition {
        repository: &repository,
        branch,
        commit: head,
    })
}
