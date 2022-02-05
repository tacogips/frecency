use clap::Parser;

#[derive(Parser, Debug)]
pub struct Add {
    sentence: String,
}

#[derive(Parser, Debug)]
pub struct Fetch {
    #[clap(short, long)]
    reverse: bool,
    #[clap(short, long)]
    limit: Option<u64>,
    #[clap(short, long)]
    with_score: bool,
}
