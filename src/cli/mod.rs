use std::path::{Path, PathBuf};

use anyhow::Result;

pub use b64::*;
pub use cli::*;
pub use csv::*;
pub use gen_pass::*;
pub use http::*;
pub use text::*;
pub use text_encrypt::*;

mod b64;
#[allow(clippy::module_inception)]
mod cli;
mod csv;
mod gen_pass;
mod http;
mod text;
mod text_encrypt;

fn verify_file(path: &str) -> Result<String, &'static str> {
    if path == "-" || Path::new(path).exists() {
        Ok(path.into())
    } else {
        Err("File not found")
    }
}

fn verify_path(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        assert!(verify_file("-").is_ok());
        assert!(verify_file("Cargo.toml").is_ok());
        assert!(verify_file("nonexistent").is_err());
    }
}
