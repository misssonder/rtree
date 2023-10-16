use crate::args::{Charset, Context};
use std::cmp::Ordering;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use crate::format::Formatter;

pub struct Symbols {
    pub down: &'static str,
    pub tee: &'static str,
    pub ell: &'static str,
    pub right: &'static str,
}

static UTF8_SYMBOLS: Symbols = Symbols {
    down: "│",
    tee: "├",
    ell: "└",
    right: "─",
};

static ASCII_SYMBOLS: Symbols = Symbols {
    down: "|",
    tee: "|",
    ell: "`",
    right: "-",
};

pub struct Printer<'a> {
    dir: &'a Path,
    full_name: bool,
    charset: Charset,
    sorter: Option<
        Box<
            dyn FnMut(&DirEntry, &DirEntry) -> Ordering
            + Send
            + Sync
            + 'static,
        >,
    >,
    filter: Option<
        Box<
            dyn FnMut(&DirEntry) -> bool
            + Send
            + Sync
            + 'static,
        >,
    >,
}

impl<'a> Printer<'a> {
    pub fn new(args: &'a Context) -> Self {
        Printer {
            dir: args.dir(),
            full_name: args.full,
            charset: args.charset.clone(),
            sorter: None,
            filter: Self::build_filter(args.all, args.direction_only),
        }
    }
    pub fn print(&mut self) -> Result<(), anyhow::Error> {
        let mut level_continues = vec![];
        let symbols = match self.charset {
            Charset::Utf8 => &UTF8_SYMBOLS,
            Charset::Ascii => &ASCII_SYMBOLS,
        };
        self.print_file(self.dir, symbols, &mut level_continues)
    }
    fn print_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        symbols: &Symbols,
        levels_continue: &mut Vec<bool>,
    ) -> Result<(), anyhow::Error> {
        let dir = fs::read_dir(path)?;
        let mut entries: Vec<_> = if let Some(ref mut filter) = self.filter {
            dir.filter(|e| {
                match e {
                    Ok(entry) => filter(entry),
                    Err(_) => true,
                }
            }).collect()
        } else {
            dir.collect()
        };
        if let Some(ref mut cmp) = self.sorter {
            entries.sort_by(|a, b| match (a, b) {
                (Ok(a), Ok(b)) => cmp(a, b),
                (&Err(_), &Err(_)) => Ordering::Equal,
                (&Ok(_), &Err(_)) => Ordering::Greater,
                (&Err(_), &Ok(_)) => Ordering::Less,
            });
        }
        let mut it = entries.into_iter().peekable();
        while let Some(entry) = it.next() {
            let entry = entry?;
            levels_continue.push(it.peek().is_some());
            println!("{}", Formatter::new(self.full_name, levels_continue, &symbols, &entry));
            if entry.file_type()?.is_dir() {
                self.print_file(entry.path().as_path(), symbols, levels_continue)?;
            }
            levels_continue.pop();
        }
        Ok(())
    }

    fn build_filter(all: bool, dir_only: bool) -> Option<Box<
        dyn FnMut(&DirEntry) -> bool
        + Send
        + Sync
        + 'static,
    >> {
        if all {
            return None;
        }
        if dir_only {
            return Some(Box::new(|e| {
                if let Ok(file_type) = e.file_type() {
                    return file_type.is_dir();
                }
                false
            }));
        }
        Some(Box::new(|e| { !is_hidden(e) }))
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && !s.eq("."))
        .unwrap_or(false)
}