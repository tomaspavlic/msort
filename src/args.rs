use clap::{Args, Parser, ValueEnum};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use std::path::PathBuf;

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
pub struct ResolverArgs {
    /// API key for Azure OpenAI
    #[arg(
        long,
        env,
        requires("openai_deployment_name"),
        requires("openai_version"),
        requires("openai_instance_name")
    )]
    pub openai_api_key: Option<String>,

    /// Deployment name for Azure OpenAI
    #[arg(
        long,
        env,
        requires("openai_api_key"),
        requires("openai_version"),
        requires("openai_instance_name")
    )]
    pub openai_deployment_name: Option<String>,

    /// Version for Azure OpenAI
    #[arg(
        long,
        env,
        requires("openai_deployment_name"),
        requires("openai_api_key"),
        requires("openai_instance_name")
    )]
    pub openai_version: Option<String>,

    /// Instance name for Azure OpenAI
    #[arg(
        long,
        env,
        requires("openai_api_key"),
        requires("openai_version"),
        requires("openai_deployment_name")
    )]
    pub openai_instance_name: Option<String>,

    /// API key for OpenSubtitles integration.
    #[arg(long, env)]
    pub opensubtitles_api_key: Option<String>,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum Resolver {
    All,
    OpenAI,
    OpenSubtitles,
}

/// Simple program to generate path for given file
#[derive(Parser, Debug)]
pub struct RootArgs {
    /// Input file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Base director to use for generated file path
    #[arg(short, long)]
    pub base_dir: PathBuf,

    #[command(flatten)]
    pub resolvers: ResolverArgs,

    /// Preview changes without moving a file.
    #[arg(long)]
    pub dry_run: bool,

    /// Replace existing files in the destination folder.
    #[arg(long)]
    pub overwrite: bool,

    /// Verbosity lebel
    #[command(flatten)]
    pub verbose: Verbosity<InfoLevel>,

    /// Resolver used to lookup information about the media.
    #[arg(long, value_enum, default_value_t = Resolver::All)]
    pub resolver: Resolver,
}
