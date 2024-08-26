use crate::args::ignore_facts::parse_facts;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Ignore file.
/// `name` Ignore file name
#[derive(Clone)]
pub struct IgnoreFile {
    name: String,
}

impl IgnoreFile {
    /// New ignore file.
    pub fn new(name: String) -> IgnoreFile {
        IgnoreFile { name }
    }

    /// Is file exists?
    pub fn exists(self) -> bool {
        Path::new(&self.name).exists()
    }

    /// Facts.
    pub fn facts(self) -> HashMap<String, Vec<String>> {
        let file = File::open(self.name);
        let reader = BufReader::new(file);
        let facts: Vec<String> = reader.lines()
            .filter_map(Result::ok)
            .collect();
        parse_facts(facts)
    }
}
