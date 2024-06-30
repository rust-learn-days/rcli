use colored::Colorize;
use duration_str::parse;
use jsonwebtoken::{
    decode, encode, get_current_timestamp, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String, // Optional. Audience
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    sub: String, // Optional. Subject (whom token refers to)
}

impl Claims {
    fn new(aud: String, exp: usize, sub: String) -> Self {
        Self { aud, exp, sub }
    }

    pub fn try_new(aud: String, exp: &str, sub: String) -> anyhow::Result<Self> {
        let duration = match parse(exp) {
            Ok(duration) => duration,
            Err(e) => return Err(anyhow::anyhow!(e)),
        };
        let now = get_current_timestamp();
        let exp_usize = duration.as_secs() as usize + now as usize;
        Ok(Self::new(aud, exp_usize, sub))
    }

    pub fn sign(self) -> anyhow::Result<String> {
        let header = Header::default();
        let token = encode(&header, &self, &EncodingKey::from_secret("secret".as_ref()))?;
        Ok(token)
    }

    pub fn verify(token: &str) -> anyhow::Result<Self> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_aud = false;
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret("secret".as_ref()),
            &validation,
        )?;
        Self::try_new(
            token_data.claims.aud,
            &token_data.claims.exp.to_string(),
            token_data.claims.sub,
        )
    }

    pub fn print(&self) {
        println!("{} {}", "Audience:".blue(), self.aud);
        println!("{} {}", "Expiration:".blue(), self.exp);
        println!("{} {}", "Subject:".blue(), self.sub);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use duration_str::parse;

    use super::*;

    #[test]
    fn test_time() {
        let duration = parse("1d").unwrap();
        assert_eq!(duration, Duration::new(24 * 60 * 60, 0));
    }

    #[test]
    fn test_claims() {
        let claims = Claims::new("aud".to_string(), 1, "sub".to_string());
        assert_eq!(claims.aud, "aud");
        assert_eq!(claims.exp, 1);
        assert_eq!(claims.sub, "sub");
    }

    #[test]
    fn test_try_new() {
        let claims = Claims::try_new("aud".to_string(), "1d", "sub".to_string()).unwrap();
        assert_eq!(claims.aud, "aud");
        let now = get_current_timestamp();
        assert_eq!(claims.exp, 86400 + now as usize);
        assert_eq!(claims.sub, "sub");
    }

    #[test]
    fn test_sign() {
        let claims = Claims::try_new("aud".to_string(), "1d", "sub".to_string()).unwrap();
        let token = claims.sign();
        assert!(token.is_ok());
    }

    #[test]
    fn test_verify() {
        let claims = Claims::try_new("aud".to_string(), "1d", "sub".to_string()).unwrap();
        let token = claims.sign().unwrap();
        let claims = Claims::verify(token.as_str()).unwrap();
        assert_eq!(claims.aud, "aud");
        assert_eq!(claims.sub, "sub");
    }
}
