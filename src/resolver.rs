use crate::{
    generator::media::Media,
    opensubtitles::{client::OpenSubtitlesClient, hasher},
};
use anyhow::Context;
use std::{collections::HashMap, path::Path};

pub trait MediaResolver {
    fn resolve(&self, path: &Path) -> anyhow::Result<Media>;
}

pub struct OpenSubtitlesMediaResolver {
    client: OpenSubtitlesClient,
}

impl OpenSubtitlesMediaResolver {
    pub fn new(api_key: impl ToString) -> Self {
        let client = OpenSubtitlesClient::new(api_key);
        Self { client }
    }
}

impl MediaResolver for OpenSubtitlesMediaResolver {
    fn resolve(&self, input: &Path) -> anyhow::Result<Media> {
        let input_file = Path::new(&input);
        let movie_hash = hasher::compute_moviehash(input_file)?;
        log::debug!("moviehash = {}", &movie_hash);
        let file_name = input
            .file_name()
            .context("missing file_name")?
            .to_str()
            .context("string conversion")?;

        let subtitles = self
            .client
            .search(&movie_hash, file_name)?
            .into_iter()
            .flat_map(|s| s.try_into());

        let media = find_most_frequent(subtitles).context("no information found")?;

        Ok(media)
    }
}

fn find_most_frequent<I>(media: I) -> Option<Media>
where
    I: IntoIterator<Item = Media>,
{
    let m = media.into_iter().fold(HashMap::new(), |mut acc, s| {
        acc.entry(s).and_modify(|c| *c += 1).or_insert(1);
        acc
    });

    let media_type = m.into_iter().max_by_key(|&(_, count)| count)?.0;

    Some(media_type)
}
