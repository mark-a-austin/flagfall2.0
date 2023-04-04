use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(short, long)]
    pub debug: bool,
    #[clap(short, long)]
    pub lichess: bool,
    #[clap(short, long)]
    pub engine: bool,
}