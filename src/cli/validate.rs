use std::fs;
use std::path::{Path, PathBuf};
use crate::error::{CliError, CliErrorKind, KrikError, KrikResult};

/// Normalize a path to an absolute canonical path if possible.
/// - If `must_exist` is true, returns an error if the path does not exist.
/// - If `must_exist` is false, returns a best-effort absolute path (without canonicalizing missing path).
pub fn normalize_path<P: AsRef<Path>>(path: P, must_exist: bool, context: &str) -> KrikResult<PathBuf> {
    let p = path.as_ref();
    if must_exist {
        if !p.exists() {
            return Err(KrikError::Cli(CliError {
                kind: CliErrorKind::PathDoesNotExist,
                path: Some(p.to_path_buf()),
                context: context.to_string(),
            }));
        }
        match fs::canonicalize(p) {
            Ok(abs) => Ok(abs),
            Err(e) => Err(KrikError::Cli(CliError {
                kind: CliErrorKind::CanonicalizeFailed(e),
                path: Some(p.to_path_buf()),
                context: context.to_string(),
            })),
        }
    } else {
        // When the path may not exist yet, we canonicalize the parent if possible.
        if p.exists() {
            return fs::canonicalize(p).map_err(|e| KrikError::Cli(CliError {
                kind: CliErrorKind::CanonicalizeFailed(e),
                path: Some(p.to_path_buf()),
                context: context.to_string(),
            }));
        }
        if let Some(parent) = p.parent() {
            let base = if parent.exists() {
                fs::canonicalize(parent).unwrap_or_else(|_| parent.to_path_buf())
            } else {
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            };
            Ok(base.join(p.file_name().unwrap_or_default()))
        } else {
            Ok(std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("."))
                .join(p))
        }
    }
}

/// Validate that a path exists and is a directory, returning a normalized absolute path.
pub fn validate_directory<P: AsRef<Path>>(path: P, context: &str) -> KrikResult<PathBuf> {
    let p = path.as_ref();
    if !p.exists() {
        return Err(KrikError::Cli(CliError {
            kind: CliErrorKind::PathDoesNotExist,
            path: Some(p.to_path_buf()),
            context: context.to_string(),
        }));
    }
    if !p.is_dir() {
        return Err(KrikError::Cli(CliError {
            kind: CliErrorKind::NotADirectory,
            path: Some(p.to_path_buf()),
            context: context.to_string(),
        }));
    }
    normalize_path(p, true, context)
}

/// Ensure a directory exists, creating it if missing. Returns the absolute path.
pub fn ensure_directory<P: AsRef<Path>>(path: P, context: &str) -> KrikResult<PathBuf> {
    let p = path.as_ref();
    if p.exists() {
        return validate_directory(p, context);
    }
    if let Err(e) = fs::create_dir_all(p) {
        return Err(KrikError::Cli(CliError {
            kind: match e.kind() {
                std::io::ErrorKind::PermissionDenied => CliErrorKind::PermissionDenied,
                _ => CliErrorKind::CreateDirFailed(e),
            },
            path: Some(p.to_path_buf()),
            context: context.to_string(),
        }));
    }
    normalize_path(p, true, context)
}

/// Parse and validate a port value in the range 1..=65535
pub fn parse_port(value: &str, context: &str) -> KrikResult<u16> {
    match value.parse::<u16>() {
        Ok(port) if port >= 1 => Ok(port),
        _ => Err(KrikError::Cli(CliError {
            kind: CliErrorKind::InvalidPort(value.to_string()),
            path: None,
            context: context.to_string(),
        })),
    }
}

/// Validate an optional theme directory, falling back to default if None provided, and return absolute path.
pub fn validate_theme_dir<P: AsRef<Path>>(opt_dir: Option<P>, context: &str) -> KrikResult<Option<PathBuf>> {
    match opt_dir {
        Some(dir) => {
            let p = dir.as_ref();
            if !p.exists() {
                return Err(KrikError::Cli(CliError {
                    kind: CliErrorKind::ThemeNotFound,
                    path: Some(p.to_path_buf()),
                    context: context.to_string(),
                }));
            }
            Ok(Some(normalize_path(p, true, context)?))
        }
        None => Ok(None),
    }
}


