use crate::dir::Symbols;
use colored::Colorize;
use std::fmt::Display;
use std::fs::DirEntry;
use std::os::unix::fs::PermissionsExt;

pub struct Formatter<'a> {
    full_name: bool,
    level_status: &'a Vec<bool>,
    symbols: &'a Symbols,
    entry: &'a DirEntry,
}

impl<'a> Formatter<'a> {
    pub fn new(
        full_name: bool,
        level_status: &'a Vec<bool>,
        symbols: &'a Symbols,
        entry: &'a DirEntry,
    ) -> Self {
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
                write!(f, "{}   ", c.cyan())?;
            }
            let c = if *last_continues {
                self.symbols.tee.cyan()
            } else {
                self.symbols.ell.cyan()
            };
            write!(f, "{0}{1}{1} ", c, self.symbols.right.cyan())?;
        }
        write!(f, "{}", EntryDisplay::new(self.entry, self.full_name))
    }
}

struct EntryDisplay<'a> {
    entry: &'a DirEntry,
    full_name: bool,
}

impl<'a> EntryDisplay<'a> {
    fn new(entry: &'a DirEntry, full_name: bool) -> Self {
        EntryDisplay { entry, full_name }
    }

    fn executable(&self) -> bool {
        let metadata = match self.entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => return false,
        };
        let permissions = metadata.permissions();
        metadata.is_file() && permissions.mode() & 0o111 != 0
    }
}

impl<'a> Display for EntryDisplay<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.entry.metadata() {
            Ok(metadata) => {
                let filename = if self.full_name {
                    self.entry.path().to_str().unwrap_or_default().to_string()
                } else {
                    self.entry
                        .file_name()
                        .to_str()
                        .unwrap_or_default()
                        .to_string()
                };
                if self.executable() {
                    return write!(f, "{}", filename.green());
                }
                if metadata.is_dir() {
                    return write!(f, "{}", filename.blue());
                }
                write!(f, "{}", filename)
            }
            Err(_) => write!(f, "{}", self.entry.file_name().to_str().unwrap_or_default()),
        }
    }
}
