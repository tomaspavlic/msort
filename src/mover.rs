use crate::{fs, generator::plex::PlexPathGenerator, resolver::MediaResolver};
use anyhow::Context;
use std::{ffi::OsStr, path::PathBuf};

pub struct FileMover {
    resolver: Box<dyn MediaResolver>,
    generator: PlexPathGenerator,
    options: FileMoverOptions,
}

pub struct FileMoverOptions {
    pub dry_run: bool,
    pub overwrite: bool,
}

impl FileMover {
    pub fn new(
        resolver: impl MediaResolver + 'static,
        generator: PlexPathGenerator,
        options: FileMoverOptions,
    ) -> Self {
        Self {
            resolver: Box::new(resolver),
            generator,
            options,
        }
    }

    pub fn run(&self, input: PathBuf) -> anyhow::Result<()> {
        log::debug!("processing input file = {:?}", &input);
        let s = self
            .resolver
            .resolve(&input)
            .context("failed getting information about the file")?;

        let mut output_path = self.generator.generate(s)?;
        output_path.set_extension(input.extension().unwrap_or(OsStr::new("")));
        log::debug!("generated output path = {:?}", &output_path);

        log::info!("moving {:?} to {:?}", input, output_path);
        if !self.options.dry_run {
            fs::move_file(input, output_path, self.options.overwrite)?;
        }

        Ok(())
    }
}
