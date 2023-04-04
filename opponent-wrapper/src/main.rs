#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod cliargs;
mod lichess;
mod engine;
mod user;
mod gametype;

pub const LICHESS_TOKEN: &str = include_str!("../token.txt");
pub const LICHESS_HOST: &str = "https://lichess.org";

#[cfg(windows)]
pub const VIRIDITHAS_EXECUTABLE_PATH: &str = "engines\\viridithas.exe";
#[cfg(windows)]
pub const MAIA_EXECUTABLE_PATH: &str = "engines\\maia.exe";
#[cfg(unix)]
pub const VIRIDITHAS_EXECUTABLE_PATH: &str = "engines/viridithas";
#[cfg(unix)]
pub const MAIA_EXECUTABLE_PATH: &str = "engines/maia";

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = <cliargs::Cli as clap::Parser>::parse();

    if args.debug {
        log::set_max_level(log::LevelFilter::Debug);
    }

    if args.lichess {
        lichess::main().await;
    }

    if args.engine {
        engine::main();
    }

    print!("\x04");
}
