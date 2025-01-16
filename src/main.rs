use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};
use generator::plex::PlexPathGenerator;
use mover::{FileMover, FileMoverOptions};
use resolver::OpenSubtitlesMediaResolver;
use std::path::PathBuf;

mod fs;
mod generator;
mod macos;
mod mover;
mod opensubtitles;
mod resolver;

/// Simple program to generate path for given file
#[derive(Parser, Debug)]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: PathBuf,

    /// Base director to use for generated file path
    #[arg(short, long)]
    base_dir: PathBuf,

    /// API key for OpenSubtitles integration.
    #[arg(long, env)]
    api_key: String,

    /// Preview changes without moving a file.
    #[arg(long)]
    dry_run: bool,

    /// Replace existing files in the destination folder.
    #[arg(long)]
    overwrite: bool,

    /// Verbosity lebel
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::builder()
        .filter_level(args.verbose.log_level_filter())
        .init();

    let resolver = OpenSubtitlesMediaResolver::new(&args.api_key);
    let plex_generator = PlexPathGenerator::new(args.base_dir);
    let options = FileMoverOptions {
        dry_run: args.dry_run,
        overwrite: args.overwrite,
    };
    let mover = FileMover::new(resolver, plex_generator, options);
    if let Err(err) = mover.run(args.input) {
        log::error!("{}", err);
    }

    Ok(())
}
