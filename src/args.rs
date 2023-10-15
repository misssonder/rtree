use clap::{Parser, ValueEnum};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
pub struct Context {
    /// Directory to list; defaults to current working directory.
    pub dir: Option<PathBuf>,
    #[arg(long = "charset", value_enum, default_value_t = Charset::Utf8)]
    /// Character set to use in output: utf8, ascii.
    pub charset: Charset,
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

impl Context {
    pub fn dir(&self) -> &Path {
        self.dir
            .as_ref()
            .map_or_else(|| Path::new("."), |pb| pb.as_path())
    }
}
