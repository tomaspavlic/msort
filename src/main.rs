use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use generator::plex::PlexPathGenerator;
use mover::FileMover;
use opensubtitles::{client::OpenSubtitlesClient, hasher};
use std::path::PathBuf;

mod fs;
mod generator;
mod macos;
mod mover;
mod opensubtitles;

/// Simple program to generate path for given file
#[derive(Parser, Debug)]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: PathBuf,

    /// Base director to use for generated file path
    #[arg(short, long)]
    base_dir: PathBuf,

    #[arg(long, env)]
    api_key: String,

    /// Verbosity lebel
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() -> anyhow::Result<()> {
    //macos::fs::copy(
    //    "/Users/tomas/Downloads/docker-27.4.1.tgz",
    //    "/Users/tomas/Downloads/docker-27.4.1.tgz_b",
    //    |p| println!("{}", p),
    //)?;

    let args = Args::parse();
    env_logger::builder()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let client = OpenSubtitlesClient::new(&args.api_key);
    let plex_generator = PlexPathGenerator::new(args.base_dir);
    let mover = FileMover::new(client, plex_generator);
    if let Err(err) = mover.run(args.input) {
        log::error!("{}", err);
    }

    Ok(())
}
