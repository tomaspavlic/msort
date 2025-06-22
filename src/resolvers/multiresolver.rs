use super::MediaResolver;
use crate::generator::media::Media;

pub struct MultiResolver {
    resolvers: Vec<Box<dyn MediaResolver>>,
}

#[derive(Default)]
pub struct MultiResolverBuilder {
    resolvers: Vec<Box<dyn MediaResolver>>,
}

impl MultiResolverBuilder {
    pub fn add(mut self, resolver: Option<impl MediaResolver + 'static>) -> Self {
        if let Some(r) = resolver {
            self.resolvers.push(Box::new(r));
        }

        self
    }

    pub fn build(self) -> anyhow::Result<MultiResolver> {
        if self.resolvers.is_empty() {
            anyhow::bail!("missing resolvers")
        }

        Ok(MultiResolver {
            resolvers: self.resolvers,
        })
    }
}

impl MediaResolver for MultiResolver {
    fn resolve(&self, path: &std::path::Path) -> anyhow::Result<Option<Media>> {
        for resolver in &self.resolvers {
            log::debug!("trying {} resolver", resolver.name());

            match resolver.resolve(path) {
                Ok(Some(media)) => return Ok(Some(media)),
                Ok(None) => {
                    log::debug!("no information found using {} resolver", resolver.name());
                    continue;
                }
                Err(err) => log::debug!(
                    "failed getting information using {} resolver: {}",
                    resolver.name(),
                    err
                ),
            }
        }

        Ok(None)
    }
}
