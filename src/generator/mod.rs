use crate::opensubtitles::model::{FeatureDetail, Subtitle};
use anyhow::bail;

pub mod plex;

#[derive(PartialEq, Eq, Hash)]
pub struct TvShowInfo {
    season: i32,
    episode: i32,
    episode_name: String,
    show_name: String,
}

#[derive(PartialEq, Eq, Hash)]
pub struct MovieInfo {
    year: u32,
    movie_name: String,
}

#[derive(PartialEq, Eq, Hash)]
pub enum MediaType {
    Episode(TvShowInfo),
    Movie(MovieInfo),
}

impl TryInto<MediaType> for Subtitle {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MediaType, Self::Error> {
        match self.attributes.feature_details {
            FeatureDetail::Movie(movie) => Ok(MediaType::Movie(MovieInfo {
                movie_name: movie.title,
                year: movie.year,
            })),
            FeatureDetail::Episode(episode) => Ok(MediaType::Episode(TvShowInfo {
                season: episode.season_number,
                episode: episode.episode_number,
                episode_name: episode.title,
                show_name: episode.parent_title,
            })),
            FeatureDetail::Tvshow(_) => bail!("unsupported media type"),
        }
    }
}
