use base64::engine::general_purpose::{STANDARD, URL_SAFE, URL_SAFE_NO_PAD};
use base64::prelude::*;

use crate::utils::process_from_input;
use crate::Format;

pub fn encode(input: &str, format: Format) -> anyhow::Result<()> {
    let input_str = match process_from_input(input) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let encoded = match format {
        Format::Standard => STANDARD.encode(input_str.as_bytes()),
        Format::UrlSafe => URL_SAFE.encode(input_str.as_bytes()),
    };
    println!("{}", encoded);
    Ok(())
}

pub fn decode(input: &str, format: Format) -> anyhow::Result<()> {
    let input_str = match process_from_input(input) {
        Ok(s) => s,
        Err(e) => return Err(e),
    };

    let decoded = match format {
        Format::Standard => STANDARD.decode(input_str.as_bytes()),
        Format::UrlSafe => URL_SAFE_NO_PAD.decode(input_str.as_bytes()),
    };
    let decoded = match decoded {
        Ok(d) => d,
        Err(e) => return Err(e.into()),
    };
    println!("{}", String::from_utf8_lossy(&decoded));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let input = "assets/base64.txt";
        let format = Format::Standard;
        assert!(encode(input, format).is_ok());
    }

    #[test]
    fn test_decode() {
        let input = "assets/base64.txt";
        let format = Format::Standard;
        assert!(decode(input, format).is_ok());
    }

    #[test]
    fn test_process_from_file() {
        let input = "Cargo.toml";
        assert!(process_from_input(input).is_ok());
    }

    // #[test]
    // fn test_process_from_input() {
    //     let input = "-";
    //     //需要输出ctrl+d 才能正常返回
    //     assert!(process_from_input(input).is_ok());
    // }
}
