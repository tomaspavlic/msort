use super::MediaResolver;
use crate::{
    args::{Resolver, RootArgs},
    generator::media::Media,
    openai::client::Client,
};
use anyhow::Context;

pub struct OpenAiMediaResolver {
    client: Client,
}

impl OpenAiMediaResolver {
    pub fn new(instance_name: &str, deployment_name: &str, version: &str, api_token: &str) -> Self {
        let client = Client::new(instance_name, deployment_name, version, api_token);
        Self { client }
    }

    pub fn from_args(args: &RootArgs) -> Option<OpenAiMediaResolver> {
        if !(Resolver::All == args.resolver || Resolver::OpenAI == args.resolver) {
            return None;
        }

        let openai_resolver = OpenAiMediaResolver::new(
            args.resolvers.openai_instance_name.as_ref()?.as_str(),
            args.resolvers.openai_deployment_name.as_ref()?.as_str(),
            args.resolvers.openai_version.as_ref()?.as_str(),
            args.resolvers.openai_api_key.as_ref()?.as_str(),
        );

        Some(openai_resolver)
    }
}

impl MediaResolver for OpenAiMediaResolver {
    fn resolve(&self, path: &std::path::Path) -> anyhow::Result<Option<Media>> {
        let file_name = path
            .file_name()
            .context("failed getting filename from file path")?
            .to_str()
            .context("failed converting filename")?
            .to_string();

        let response = self.client.prompt(file_name)?;
        if !response.success {
            return Ok(None);
        }

        Ok(Some(response.into()))
    }

    fn name(&self) -> &'static str {
        "open-ai"
    }
}
