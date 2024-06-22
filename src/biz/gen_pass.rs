use anyhow::Error;
use rand::Rng;

pub fn gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<String, Error> {
    let mut pass = String::new();
    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();
    if upper {
        chars.extend_from_slice(b"ABCDEFGHJKLMNOPQRSTUVWXYZ");
    }
    if lower {
        chars.extend_from_slice(b"abcdefghijkmnpqrstuvwxyz");
    }
    if number {
        chars.extend_from_slice(b"123456789");
    }
    if symbol {
        chars.extend_from_slice(b"!@#$%^&*()-_=+");
    }
    for _ in 0..length {
        let idx = rng.gen_range(0..chars.len());
        pass.push(chars[idx] as char);
    }
    Ok(pass)
}
