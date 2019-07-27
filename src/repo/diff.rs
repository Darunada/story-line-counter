use std::fmt;
use std::fmt::Formatter;

pub struct DiffResult {
    pub story_number: String,
    pub first_summary: String,
    pub second_summary: String,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize
}

impl fmt::Display for DiffResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{} :: ({} files, ∑({},|-{}|) = {}) :: {} -> {}",
               self.story_number,
               self.files_changed,
               self.insertions,
               self.deletions,
               self.insertions + self.deletions,
               self.first_summary,
               self.second_summary)?;
        Ok(())
    }
}

pub struct DiffTotal {
    pub story_number: String,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub total_diff_results: usize,
}

impl fmt::Display for DiffTotal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f,
               "{} :: ({} files, ∑({},|-{}|) = {}) in {} commits",
               self.story_number,
               self.files_changed,
               self.insertions,
               self.deletions,
               self.insertions + self.deletions,
               self.total_diff_results)?;
        Ok(())
    }
}
