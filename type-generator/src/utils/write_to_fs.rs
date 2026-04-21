use std::fs;
use std::path::Path;

use crate::build_errors::BuildError;

/// Writes generated code to a file, ensuring all parent directories exist.
///
/// Logs the output path to stdout before writing.
pub fn write_generated_file(
    target_folder: &str,
    relative_path: &str,
    content: &str,
    title: &str,
) -> Result<(), BuildError> {
    let out_path = Path::new(target_folder).join(relative_path);

    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            BuildError::Io(std::io::Error::new(
                e.kind(),
                format!("Failed to create output directory at {}: {e}", parent.display()),
            ))
        })?;
    }

    println!("Writing {title} to: {}", out_path.display());
    fs::write(&out_path, content).map_err(|e| {
        BuildError::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to write file {}: {e}", out_path.display()),
        ))
    })?;

    Ok(())
}
