use crate::{
    fs,
    generator::plex::PlexPathGenerator,
    opensubtitles::{client::OpenSubtitlesClient, hasher},
};
use anyhow::Context;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

pub struct FileMover {
    client: OpenSubtitlesClient,
    generator: PlexPathGenerator,
}

impl FileMover {
    pub fn new(client: OpenSubtitlesClient, generator: PlexPathGenerator) -> Self {
        Self { client, generator }
    }

    pub fn run(&self, input: PathBuf) -> anyhow::Result<()> {
        log::debug!("processing input file = {:?}", &input);
        let input_file = Path::new(&input);
        let movie_hash = hasher::compute_moviehash(&input_file)?;
        log::debug!("moviehash = {}", &movie_hash);

        let subtitles = self.client.search_by_moviehash(&movie_hash)?;
        let s = subtitles
            .into_iter()
            .flat_map(|s| s.try_into())
            .next()
            .context("could not find any information for given file")?;

        let mut output_path = self.generator.generate(s)?;
        output_path.set_extension(input.extension().unwrap_or(OsStr::new("")));
        log::debug!("generated output path = {:?}", &output_path);

        log::info!("moving {:?} to {:?}", input, output_path);
        fs::move_file(input, output_path, false)?;

        Ok(())
    }
}
