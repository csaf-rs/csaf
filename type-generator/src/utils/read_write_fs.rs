use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::build_errors::BuildError;

/// Returns the root directory of the `type-generator` crate.
///
/// All relative paths used throughout the generator are resolved against this
/// directory so that the tool works regardless of the current working directory.
pub fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Resolves `path` against [`crate_root`] when it is relative, leaving
/// absolute paths untouched.
pub fn resolve_path(path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        crate_root().join(path)
    }
}

/// Writes generated code to a file, ensuring all parent directories exist.
///
/// Logs the output path to stdout before writing.
/// Relative paths in `target_folder` are resolved against [`crate_root`].
pub fn write_generated_file(
    target_folder: &str,
    relative_path: &str,
    content: &str,
    title: &str,
) -> Result<(), BuildError> {
    let rel = Path::new(relative_path);

    // Reject absolute paths to avoid writing outside the target folder.
    if rel.has_root() {
        return Err(BuildError::PathEscape(format!(
            "relative_path must not be absolute: {relative_path}"
        )));
    }

    // Reject any ".." components to avoid writing outside the target folder.
    if rel.components().any(|c| matches!(c, std::path::Component::ParentDir)) {
        return Err(BuildError::PathEscape(format!(
            "relative_path must not contain '..' components: {relative_path}"
        )));
    }

    let out_path = resolve_path(Path::new(target_folder)).join(rel);

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

    if relative_path.ends_with(".rs") {
        format_with_rustfmt(&out_path)?;
    }

    Ok(())
}

/// Reads a file to a string, returning a descriptive error on failure.
/// Relative paths are resolved against [`crate_root`].
pub fn read_file_to_string(path: &Path) -> Result<String, BuildError> {
    let resolved = resolve_path(path);
    fs::read_to_string(&resolved).map_err(|e| {
        BuildError::Io(std::io::Error::new(
            e.kind(),
            format!("Failed to read file at {}: {e}", resolved.display()),
        ))
    })
}

/// Formats a Rust source file in place by invoking `rustfmt`.
fn format_with_rustfmt(path: &Path) -> Result<(), BuildError> {
    let output = Command::new("rustfmt").arg(path).output()?;

    if !output.status.success() {
        Err(std::io::Error::other("rustfmt failed"))?;
    }

    Ok(())
}
