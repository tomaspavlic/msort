use crate::{fs, generator::plex::PlexPathGenerator, resolvers::MediaResolver};
use anyhow::Context;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

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

    pub fn run(&self, input: &PathBuf) -> anyhow::Result<()> {
        log::debug!("processing input file = {:?}", &input);

        let media = self
            .resolver
            .resolve(input)?
            .context("no information found")?;
        let mut output_path = self.generator.generate(media)?;
        output_path.set_extension(input.extension().unwrap_or(OsStr::new("")));
        log::debug!("generated output path = {:?}", &output_path);

        if !self.options.dry_run {
            log::info!("moving {:?} to {:?}", input, output_path);
            fs::move_file(input, &output_path, self.options.overwrite)?;
        } else {
            self.move_file_dry(input, &output_path);
        }

        Ok(())
    }

    fn move_file_dry<P>(&self, input: &P, output_path: &P)
    where
        P: AsRef<Path>,
    {
        log::info!(
            "[DRY-RUN] would move {:?} to {:?}",
            input.as_ref(),
            output_path.as_ref()
        )
    }
}
