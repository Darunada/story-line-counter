use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;

#[derive(Serialize, Deserialize, Debug)]
pub struct DiffResult {
    pub story_number: Vec<String>,
    pub points: String,
    pub first_summary: String,
    pub second_summary: String,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
}

impl fmt::Display for DiffResult {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}) :: ({} files, ∑({},|-{}|) = {}) :: {} -> {}",
            self.story_number.join(", "),
            self.points,
            self.files_changed,
            self.insertions,
            self.deletions,
            self.insertions + self.deletions,
            self.first_summary,
            self.second_summary
        )?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiffCollection {
    pub diffs: Vec<DiffResult>,
}

impl fmt::Display for DiffCollection {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.diffs.iter().for_each(|diff_result| {
            writeln!(f, "{}", diff_result.to_string()).unwrap();
        });

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoryPoint {
    pub story_number: String,
    pub points: String,
}

impl fmt::Display for StoryPoint {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{} => {}sp", self.story_number, self.points).unwrap();
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoryPointCollection {
    pub story_points: Vec<StoryPoint>,
}

impl fmt::Display for StoryPointCollection {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.story_points.iter().for_each(|story_point| {
            writeln!(f, "{}", story_point.to_string()).unwrap();
        });

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiffTotal {
    pub story_number: String,
    pub points: String,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub total_diff_results: usize,
}

impl fmt::Display for DiffTotal {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}) :: ({} files, ∑({},|-{}|) = {}) in {} commits",
            self.story_number,
            self.points,
            self.files_changed,
            self.insertions,
            self.deletions,
            self.insertions + self.deletions,
            self.total_diff_results
        )?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiffTotalCollection {
    pub totals: HashMap<String, DiffTotal>,
}

impl fmt::Display for DiffTotalCollection {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.totals.iter().for_each(|(_, diff_total)| {
            writeln!(f, "{}", diff_total.to_string()).unwrap();
        });

        Ok(())
    }
}
