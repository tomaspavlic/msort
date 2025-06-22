use crate::generator::media::Media;
use std::path::Path;

pub mod multiresolver;
pub mod openai;
pub mod opensubtitles;

pub trait MediaResolver {
    fn resolve(&self, path: &Path) -> anyhow::Result<Option<Media>>;
    fn name(&self) -> &'static str {
        "unknown"
    }
}
