use git2::{Repository, Revwalk, Oid};

fn main() {
    let repository = get_repository(".");
    let revwalk = get_revwalk(&repository, "master");
    for commit in revwalk {
        let commit_id = match commit {
            Ok(id) => {
                println!("Looking at commit {}.", id);
                id
            },
            Err(error) => {
                println!("Skipping commit during revwalk. Error: {}", error.to_string());
                continue;
            }
        };

        parse_commit(commit_id);
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

fn get_revwalk<'a>(repository: &'a Repository, rev: &str) -> Revwalk<'a> {
    match repository.revwalk() {
        Ok(mut the_revwalk) => {
            the_revwalk.reset();
            match the_revwalk.push_ref(rev) {
                Ok(_) => {},
                Err(error) => println!("Problem pushing rev {} into revwalk.  Error: {}", rev, error.to_string()),
            };
            the_revwalk
        },
        Err(error) => {
            println!("Unable to open revision: {}.  Error: {}", rev, error.to_string());
            panic!();
        }
    }
}

fn parse_commit(_oid: Oid) {
}
