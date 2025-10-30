use std::any::type_name;

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
    pub fn add<T: MediaResolver + 'static>(mut self, resolver: Option<T>) -> Self {
        let resolver_name = type_name::<T>().rsplit("::").next().unwrap();
        if let Some(r) = resolver {
            self.resolvers.push(Box::new(r));
            log::debug!("Resolver {:?} registered.", &resolver_name);
        } else {
            log::debug!("Resolver {:?} not registered.", &resolver_name);
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
