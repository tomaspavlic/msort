use anyhow::Context;
use std::{fs, path::Path};

pub fn move_file<P>(from: P, to: P, overwrite: bool) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    if !overwrite && fs::exists(&to)? {
        anyhow::bail!("destination file already exists");
    }

    fs::create_dir_all(&to.as_ref().parent().context("invalid path")?)?;

    if let Ok(_) = fs::rename(&from, &to) {
        return Ok(());
    }

    copy(&from, &to)?;
    fs::remove_file(from)?;

    Ok(())
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn copy<P>(from: P, to: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    std::fs::copy(&from, to)?;

    Ok(())
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn copy<P>(from: P, to: P) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    #[cfg(target_os = "macos")]
    use crate::macos::fs::copy;
    #[cfg(target_os = "windows")]
    use crate::windows::fs::copy;
    use indicatif::{ProgressBar, ProgressStyle};
    use std::fs::File;
    use std::rc::Rc;

    const PROGRESS_STYLE_TEMPLATE: &str =
        "[{wide_bar}] {bytes_per_sec}   {bytes}/{total_bytes} ({eta})";

    let file_len = File::open(&from)?.metadata()?.len();
    let style = ProgressStyle::with_template(PROGRESS_STYLE_TEMPLATE)?.progress_chars("##-");
    let bar = Rc::new(ProgressBar::new(file_len).with_style(style));
    let bar_clone = Rc::clone(&bar);
    copy(&from, &to, move |p| {
        bar_clone.set_position(p as u64);
    })?;
    bar.finish_and_clear();

    Ok(())
}
