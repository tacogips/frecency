mod cmd;
mod config;
mod frecency;

use chrono::Utc;
use clap::Parser;
use cmd::*;
use config::*;
use frecency::*;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CmdError {
    #[error("{0}")]
    FrecencyError(#[from] FrecencyError),

    #[error("{0}")]
    ConfigError(#[from] ConfigError),
}

pub type Result<T> = std::result::Result<T, CmdError>;

#[derive(Parser, Debug)]
#[clap(version = "0.1.0", author = "tacogips")]
struct Opts {
    #[clap(short, long)]
    db_file: Option<String>,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[clap(about = "Add path")]
    Add(Add),
    #[clap(about = "Show paths list orderd by frecency")]
    Fetch(Fetch),
    #[clap(about = "Remove paths that not exists anymore.")]
    RemoveNotExists,
}

fn show_only_path(path: &str, _: f64) {
    println!("{path}")
}

fn show_with_score(path: &str, score: f64) {
    println!("{score}    {path}")
}

fn run() -> Result<()> {
    let opts: Opts = Opts::parse();
    let config = Config::new(opts.db_file)?;
    let mut db = DB::new(config.dbpath, None)?;
    match opts.subcmd {
        SubCommand::Add(add) => {
            let now = Utc::now();
            let latest_visit_in_milli_sec = now.timestamp_millis() as u64;
            if let Err(_) = add_visit(&mut db, &add.path, latest_visit_in_milli_sec) {
                create_tables(&db)?;
                add_visit(&mut db, &add.path, latest_visit_in_milli_sec)?
            }
        }
        SubCommand::Fetch(fetch) => {
            let scores = fetch_scores(&db, fetch.limit)?;

            let print_fn = if fetch.with_score {
                show_with_score
            } else {
                show_only_path
            };

            if fetch.asc {
                for (path, score) in scores.into_iter().rev() {
                    print_fn(&path, score)
                }
            } else {
                for (path, score) in scores.into_iter() {
                    print_fn(&path, score)
                }
            }
        }
        SubCommand::RemoveNotExists => {
            let scores = fetch_scores(&db, None)?;

            let mut paths_to_remove = Vec::new();
            for (path, _) in scores.iter() {
                match PathBuf::try_from(path.as_str()) {
                    Ok(file_path) => {
                        if !file_path.exists() {
                            paths_to_remove.push(path.as_str())
                        };
                    }
                    Err(_) => paths_to_remove.push(path.as_str()),
                }
            }
            remove_paths(&mut db, paths_to_remove.as_slice())?;
        }
    }
    Ok(())
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error:{:?}", err);
            1
        }
    })
}
