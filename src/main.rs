use crate::args::Context;
use clap::Parser;
use crate::dir::Printer;

mod args;
mod dir;
mod format;

fn main() {
    let context = Context::parse();
    let mut printer = Printer::new(&context);
    printer.print().unwrap();
}
