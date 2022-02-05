mod cmd;
mod config;
mod db;

use clap::Parser;
use cmd::*;

#[derive(Parser, Debug)]
#[clap(version = "0.2.5", author = "tacogips")]
struct Opts {
    #[clap(short, long)]
    db_file: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(about = "Increments number of appearance of the word")]
    Add(Add),
    #[clap(about = "Show words sorted by its number of appearance")]
    Fetch(Fetch),
}

fn main() {
    println!("Hello, world!");
}
