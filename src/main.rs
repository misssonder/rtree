#![feature(let_chains)]

use crate::args::Context;
use crate::dir::Printer;
use clap::Parser;

mod args;
mod dir;
mod format;

fn main() {
    let context = Context::parse();
    let mut printer = Printer::new(&context);
    printer.print().unwrap();
}
