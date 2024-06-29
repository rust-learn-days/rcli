pub use b64::*;
pub use cli::*;
pub use csv::*;
pub use gen_pass::*;
pub use text::*;
pub use text_encrypt::*;

mod b64;
#[allow(clippy::module_inception)]
mod cli;
mod csv;
mod gen_pass;
mod text;
mod text_encrypt;

fn verify_file_exists(path: &str) -> Result<String, String> {
    if path == "-" || std::path::Path::new(path).exists() {
        Ok(path.into())
    } else {
        Err(format!("File not found: {}", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file_exists() {
        assert!(verify_file_exists("-").is_ok());
        assert!(verify_file_exists("Cargo.toml").is_ok());
        assert!(verify_file_exists("nonexistent").is_err());
    }
}
