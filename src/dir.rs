use crate::args::{Charset, Context};
use std::cmp::Ordering;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use crate::format::Formatter;

struct Symbols {
    down: &'static str,
    tee: &'static str,
    ell: &'static str,
    right: &'static str,
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
    all: bool,
    dir_only: bool,
    charset: Charset,
    sorter: Option<
        Box<
            dyn FnMut(&DirEntry, &DirEntry) -> Ordering
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
            all: args.all,
            dir_only: args.direction_only,
            charset: args.charset.clone(),
            sorter: None,
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
        let mut entries: Vec<_> = dir.collect();
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
            if let Some((last_continues, rest)) = levels_continue.split_last() {
                for continues in rest {
                    let c = if *continues { symbols.down } else { " " };
                    print!("{}   ", c);
                }

                let c = if *last_continues {
                    symbols.tee
                } else {
                    symbols.ell
                };
                print!("{0}{1}{1} ", c, symbols.right);
            }
            println!("{}", Formatter::new(self.full_name, &entry));
            if entry.file_type()?.is_dir() {
                self.print_file(entry.path().as_path(), symbols, levels_continue)?;
            }
            levels_continue.pop();
        }
        Ok(())
    }

}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.') && !s.eq("."))
        .unwrap_or(false)
}