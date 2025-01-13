use crate::{
    fs,
    generator::{media_type::MediaType, plex::PlexPathGenerator},
    opensubtitles::{client::OpenSubtitlesClient, hasher},
};
use anyhow::Context;
use std::{
    collections::HashMap,
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

        let media = self
            .client
            .search_by_moviehash(&movie_hash)?
            .into_iter()
            .flat_map(|s| s.try_into());
        let s = FileMover::find_most_frequent(media)
            .context("could not find any information for given file")?;

        let mut output_path = self.generator.generate(s)?;
        output_path.set_extension(input.extension().unwrap_or(OsStr::new("")));
        log::debug!("generated output path = {:?}", &output_path);

        log::info!("moving {:?} to {:?}", input, output_path);
        fs::move_file(input, output_path, false)?;

        Ok(())
    }

    fn find_most_frequent<I>(media: I) -> Option<MediaType>
    where
        I: IntoIterator<Item = MediaType>,
    {
        let m = media.into_iter().fold(HashMap::new(), |mut acc, s| {
            acc.entry(s).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

        let media_type = m.into_iter().max_by_key(|&(_, count)| count)?.0;

        Some(media_type)
    }
}
