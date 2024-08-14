use std::path::{Path, PathBuf};

#[must_use]
pub fn canonicalize_unexistent(s: &Path) -> Option<PathBuf> {
    for p in s.ancestors() {
        if let Ok(path) = (|| {
            let canonical = p.canonicalize()?;
            let stripped = s.strip_prefix(p)?;
            Ok::<PathBuf, anyhow::Error>(canonical.join(stripped))
        })() {
            return Some(path);
        };
    }
    None
}
