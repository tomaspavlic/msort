use anyhow::{Context, bail};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, path::Path, rc::Rc};

pub fn move_file<P>(from: P, to: P, overwrite: bool) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    if !overwrite && std::fs::exists(&to)? {
        bail!("destination file already exists");
    }

    std::fs::create_dir_all(&to.as_ref().parent().context("invalid path")?)?;

    if let Ok(_) = std::fs::rename(&from, &to) {
        return Ok(());
    }

    if cfg!(target_os = "macos") {
        let file_len = File::open(&from)?.metadata()?.len();
        let style = ProgressStyle::with_template("[{wide_bar}] {bytes}/{total_bytes}")?
            .progress_chars("##-");
        let bar = Rc::new(ProgressBar::new(file_len).with_style(style));
        let b = Rc::clone(&bar);
        crate::macos::fs::copy(&from, &to, move |p| {
            b.set_position(p as u64);
        })?;
        bar.finish_and_clear();
    } else {
        std::fs::copy(&from, to)?;
    }

    std::fs::remove_file(from)?;

    Ok(())
}
