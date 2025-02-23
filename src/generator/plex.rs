use super::media::Media;
use std::path::PathBuf;

pub struct PlexPathGenerator {
    base_dir: PathBuf,
}

impl PlexPathGenerator {
    pub fn new<P>(base_dir: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            base_dir: base_dir.into(),
        }
    }

    pub fn generate(&self, md: Media) -> anyhow::Result<PathBuf> {
        match md {
            Media::Episode(episode) => {
                let n = format!(
                    "{} - S{:0>2}E{:0>2} - {}",
                    episode.show_name, episode.season, episode.episode, episode.episode_name
                );

                let p = self
                    .base_dir
                    .join("TV Shows")
                    .join(&episode.show_name)
                    .join(format!("Season {}", episode.season))
                    .join(&n);

                Ok(p)
            }
            Media::Movie(movie) => {
                let n = format!("{} ({})", movie.movie_name, movie.year);
                let p = self.base_dir.join("Movies").join(&n).join(&n);
                Ok(p)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::media::{Episode, Movie};
    use std::str::FromStr;

    #[test]
    fn generate_episode_path_works() {
        let episode = Media::Episode(Episode {
            season: 1,
            episode: 2,
            episode_name: "test".into(),
            show_name: "House".into(),
        });
        let actual = PlexPathGenerator::new("/DATA").generate(episode).unwrap();
        let expected_path =
            PathBuf::from_str("/DATA/TV Shows/House/Season 1/House - S01E02 - test").unwrap();

        assert_eq!(actual, expected_path)
    }

    #[test]
    fn generate_movie_path_works() {
        let movie = Media::Movie(Movie {
            year: 2025,
            movie_name: "test".into(),
        });
        let actual = PlexPathGenerator::new("/DATA").generate(movie).unwrap();
        let expected_path = PathBuf::from_str("/DATA/Movies/test (2025)/test (2025)").unwrap();

        assert_eq!(actual, expected_path)
    }
}
