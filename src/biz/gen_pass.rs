use anyhow::Error;
use rand::prelude::SliceRandom;
use rand::Rng;

const UPPER_CHARS: &[u8] = b"ABCDEFGHJKLMNOPQRSTUVWXYZ";
const LOWER_CHARS: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER_CHARS: &[u8] = b"123456789";
const SYMBOL_CHARS: &[u8] = b"!@#$%^&*-_=+";

pub fn gen_pass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> Result<String, Error> {
    let mut pass = Vec::with_capacity(length as usize);
    let mut rng = rand::thread_rng();
    let mut chars = Vec::new();
    if upper {
        chars.extend_from_slice(UPPER_CHARS);
        pass.push(UPPER_CHARS[rng.gen_range(0..UPPER_CHARS.len())] as char);
    }
    if lower {
        chars.extend_from_slice(LOWER_CHARS);
        pass.push(LOWER_CHARS[rng.gen_range(0..LOWER_CHARS.len())] as char);
    }
    if number {
        chars.extend_from_slice(NUMBER_CHARS);
        pass.push(NUMBER_CHARS[rng.gen_range(0..NUMBER_CHARS.len())] as char);
    }
    if symbol {
        chars.extend_from_slice(SYMBOL_CHARS);
        pass.push(SYMBOL_CHARS[rng.gen_range(0..SYMBOL_CHARS.len())] as char);
    }
    for _ in 0..length - (pass.len() as u8) {
        let idx = rng.gen_range(0..chars.len());
        pass.push(chars[idx] as char);
    }
    pass.shuffle(&mut rng);
    Ok(pass.iter().collect())
}
