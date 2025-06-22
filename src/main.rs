use args::RootArgs;
use clap::Parser;
use generator::plex::PlexPathGenerator;
use mover::{FileMover, FileMoverOptions};
use resolvers::{
    multiresolver::MultiResolverBuilder, openai::OpenAiMediaResolver,
    opensubtitles::OpenSubtitlesMediaResolver,
};

mod args;
mod fs;
mod generator;
mod macos;
mod mover;
mod openai;
mod opensubtitles;
mod resolvers;
mod windows;

fn main() -> anyhow::Result<()> {
    let args = RootArgs::parse();
    env_logger::builder()
        .filter_level(args.verbose.log_level_filter())
        .init();

    // init media resolvers
    let openai_resolver = OpenAiMediaResolver::from_args(&args);
    let opensubtitles_resolver = OpenSubtitlesMediaResolver::from_args(&args);
    let resolver = MultiResolverBuilder::default()
        .add(opensubtitles_resolver)
        .add(openai_resolver)
        .build()?;

    // init output file path generator
    let plex_generator = PlexPathGenerator::new(args.base_dir);
    let options = FileMoverOptions {
        dry_run: args.dry_run,
        overwrite: args.overwrite,
    };
    let mover = FileMover::new(resolver, plex_generator, options);

    // run file mover
    if let Err(err) = mover.run(&args.input) {
        log::error!("failed processing {:?}: {}", args.input, err);
    }

    Ok(())
}
