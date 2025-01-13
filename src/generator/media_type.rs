use crate::opensubtitles::model::{FeatureDetail, Subtitle};
use anyhow::bail;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Episode {
    pub season: i32,
    pub episode: i32,
    pub episode_name: String,
    pub show_name: String,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Movie {
    pub year: u32,
    pub movie_name: String,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum MediaType {
    Episode(Episode),
    Movie(Movie),
}

impl TryInto<MediaType> for Subtitle {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MediaType, Self::Error> {
        match self.attributes.feature_details {
            FeatureDetail::Movie(movie) => Ok(MediaType::Movie(Movie {
                movie_name: movie.title,
                year: movie.year,
            })),
            FeatureDetail::Episode(episode) => Ok(MediaType::Episode(Episode {
                season: episode.season_number,
                episode: episode.episode_number,
                episode_name: episode.title,
                show_name: episode.parent_title,
            })),
            FeatureDetail::Tvshow(_) => bail!("unsupported media type"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opensubtitles::model::Attributes;

    #[test]
    fn can_convert_movie() {
        let feature_details = FeatureDetail::Movie(crate::opensubtitles::model::Movie {
            year: 2025,
            title: "test".into(),
        });
        let attributes = Attributes { feature_details };
        let subtitles = Subtitle { attributes };
        let actual = subtitles.try_into().unwrap();
        let expected = MediaType::Movie(Movie {
            year: 2025,
            movie_name: "test".into(),
        });

        assert_eq!(expected, actual)
    }

    #[test]
    fn can_convert_episode() {
        let feature_details = FeatureDetail::Episode(crate::opensubtitles::model::Episode {
            episode_number: 2,
            season_number: 1,
            title: "Some Episode".into(),
            parent_title: "House".into(),
        });
        let attributes = Attributes { feature_details };
        let subtitles = Subtitle { attributes };
        let actual = subtitles.try_into().unwrap();
        let expected = MediaType::Episode(Episode {
            episode: 2,
            season: 1,
            show_name: "House".into(),
            episode_name: "Some Episode".into(),
        });

        assert_eq!(expected, actual)
    }

    #[test]
    fn cannot_convert_tvshow() {
        let feature_details = FeatureDetail::Tvshow(crate::opensubtitles::model::TvShow {});
        let attributes = Attributes { feature_details };
        let subtitles = Subtitle { attributes };
        let actual: Result<MediaType, anyhow::Error> = subtitles.try_into();

        assert!(actual.is_err())
    }
}
