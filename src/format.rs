use std::fmt::Display;
use std::fs::DirEntry;

pub struct Formatter<'a> {
    full_name: bool,
    entry: &'a DirEntry,
}

impl<'a> Formatter <'a>{
    pub fn new(full_name: bool, entry: &'a DirEntry) -> Self {
        Self {
            full_name,
            entry,
        }
    }
}

impl Display for Formatter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.full_name {
            write!(f, "{}", self.entry.path().as_path().to_str().unwrap())
        } else {
            write!(f, "{}", self.entry.file_name().to_str().unwrap())
        }
    }
}