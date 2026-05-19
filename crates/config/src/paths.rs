use std::path::PathBuf;

use anyhow::{Context, Result};

const APP_DIR_NAME: &str = "biggit";

/// Returns the platform-specific configuration directory for biggit.
///
/// - Linux: `~/.config/biggit/`
/// - macOS: `~/Library/Application Support/biggit/`
/// - Windows: `%APPDATA%\biggit\`
pub fn config_dir() -> Result<PathBuf> {
    let base = dirs::config_dir().context("could not determine config directory for this OS")?;
    Ok(base.join(APP_DIR_NAME))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_dir_ends_with_biggit() {
        let dir = config_dir().unwrap();
        assert_eq!(dir.file_name().unwrap(), APP_DIR_NAME);
    }
}
