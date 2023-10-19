use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
pub struct Context {
    /// Directory to list; defaults to current working directory.
    pub dir: Option<PathBuf>,
    #[arg(long = "charset", short, value_enum, default_value_t = Charset::Utf8)]
    /// Character set to use in output
    pub charset: Charset,
    #[arg(long, short, value_enum, default_value_t = SortArgs::Filename)]
    /// Sorting options
    pub sort: SortArgs,
    #[arg(long, short)]
    /// Reverse the order of the sort.
    pub reverse: bool,
    #[arg(long = "all", short)]
    /// All files are listed.
    pub all: bool,
    #[arg(long = "dir", short)]
    /// List directories only.
    pub direction_only: bool,
    #[arg(long = "full", short)]
    /// Print the full path prefix for each file.
    pub full: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Charset {
    Utf8,
    Ascii,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum SortArgs {
    Filename,
    Size,
    CreatedTime,
    ModifiedTime,
}

impl Context {
    pub fn dir(&self) -> &Path {
        self.dir
            .as_ref()
            .map_or_else(|| Path::new("."), |pb| pb.as_path())
    }
}
