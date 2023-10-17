use std::fmt::Display;
use std::fs::DirEntry;
use colored::Colorize;
use crate::dir::Symbols;

pub struct Formatter<'a> {
    full_name: bool,
    level_status: &'a Vec<bool>,
    symbols: &'a Symbols,
    entry: &'a DirEntry,
}

impl<'a> Formatter<'a> {
    pub fn new(full_name: bool, level_status: &'a Vec<bool>, symbols: &'a Symbols, entry: &'a DirEntry) -> Self {
        Self {
            full_name,
            level_status,
            symbols,
            entry,
        }
    }
}

impl Display for Formatter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some((last_continues, rest)) = self.level_status.split_last() {
            for continues in rest {
                let c = if *continues { self.symbols.down } else { " " };
                write!(f, "{}   ", c)?;
            }
            let c = if *last_continues {
                self.symbols.tee
            } else {
                self.symbols.ell
            };
            write!(f, "{0}{1}{1} ", c, self.symbols.right)?;
        }
        if self.full_name {
            if let Ok(file_type) = self.entry.file_type() && file_type.is_dir() {
                write!(f, "{}", self.entry.path().as_path().to_str().unwrap_or_default().blue())
            } else {
                write!(f, "{}", self.entry.path().as_path().to_str().unwrap_or_default())
            }
        } else if let Ok(file_type) = self.entry.file_type() && file_type.is_dir() {
            write!(f, "{}", self.entry.file_name().to_str().unwrap_or_default().blue())
        } else {
            write!(f, "{}", self.entry.file_name().to_str().unwrap_or_default())
        }
    }
}