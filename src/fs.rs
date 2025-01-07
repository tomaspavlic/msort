use std::path::Path;

use anyhow::bail;

pub fn move_file<P>(from: P, to: P, overwrite: bool) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    if !overwrite && std::fs::exists(&to)? {
        bail!("destination file already exists");
    }

    if let Ok(_) = std::fs::rename(&from, &to) {
        return Ok(());
    }

    std::fs::copy(&from, to)?;
    std::fs::remove_file(from)?;

    Ok(())
}
