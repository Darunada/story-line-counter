
use git2::{Commit, Error, Oid, Repository, Branch, BranchType};

pub struct RepoPosition<'repo> {
    pub repository: &'repo Repository,
    pub branch: Branch<'repo>,
    pub commit: Commit<'repo>
}

pub fn get_repository(path: &str) -> Result<Repository, Error> {
    Repository::discover(path)
}

pub fn get_commit<'repo>(repository: &'repo Repository, oid: &Oid) -> Result<Commit<'repo>, Error> {
    repository.find_commit(*oid)
}

pub fn get_repo_head<'repo>(repository: &'repo Repository, branch_name: &str) -> Result<RepoPosition<'repo>, Error> {
    let branch = get_branch(repository, branch_name, BranchType::Local)?;
    let head = branch.get().peel_to_commit()?;

    Ok(RepoPosition {
        repository: &repository,
        branch,
        commit: head,
    })
}

pub fn get_branch<'repo>(repository: &'repo Repository, branch: &str, branch_type: BranchType) -> Result<Branch<'repo>, Error> {
    repository.find_branch(branch, branch_type)
}
