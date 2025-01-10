use super::MediaType;
use std::path::PathBuf;

pub struct PlexPathGenerator {
    base_dir: PathBuf,
}

impl PlexPathGenerator {
    pub fn new(base_dir: PathBuf) -> Self {
        Self { base_dir }
    }

    pub fn generate(&self, md: MediaType) -> anyhow::Result<PathBuf> {
        match md {
            MediaType::Episode(episode) => {
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
            MediaType::Movie(movie) => {
                let n = format!("{} ({})", movie.movie_name, movie.year);
                let p = self.base_dir.join("Movies").join(&n).join(&n);
                Ok(p)
            }
        }
    }
}
