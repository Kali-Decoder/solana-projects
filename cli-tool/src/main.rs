use clap::Parser;
use maud::{Doctype,Markup,html};
use pulldown_cmark::{Options,Parser as MarkdownParser,html};
use std::{fs,path::PathBuf};
#[derive(Parser,Debug)]
struct Args{

    /// Input makrdown file path
    #[arg(long,short)]
    input : String

    /// Output html file path
    #[arg(long,short)]
    output : String
}

fn main() {
    let args = Args::parse();
    let markdown_input = fs::read_to_string(&args.input).expect("Failed to read string");


    let options = Options::empty();
    options.insert(Options:: )
}
