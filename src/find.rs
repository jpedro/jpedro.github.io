use std::io::Result;

use std::fs;
use std::path::Path;
use std::path::PathBuf;


pub fn files(path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
    let mut paths = vec![];
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subs = files(entry.path())?;
            paths.append(&mut subs);
        }

        if meta.is_file() {
            paths.push(entry.path());
        }
    }

    Ok(paths)
}
