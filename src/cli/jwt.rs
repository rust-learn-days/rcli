use clap::Parser;
use colored::Colorize;
use enum_dispatch::enum_dispatch;

use crate::biz::jwt::Claims;
use crate::CmdExec;

use super::verify_duration;

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExec)]
pub enum JwtSubCommand {
    #[command(about = "Sign a json web token")]
    Sign(JwtSignOpts),
    #[command(about = "Verify a json web token")]
    Verify(JwtVerifyOpts),
}

#[derive(Debug, Parser)]
pub struct JwtSignOpts {
    #[arg(short, long, long_help = "subject for json web token")]
    pub sub: String,
    #[arg(short, long, long_help = "audience for json web token")]
    pub aud: String,
    #[arg(
        short, long, value_parser = verify_duration, long_help = "expiration time for json web token"
    )]
    pub exp: String,
}

#[derive(Debug, Parser)]
pub struct JwtVerifyOpts {
    #[arg(short, long, long_help = "json web token to verify")]
    pub token: String,
}

impl CmdExec for JwtSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let claims = Claims::try_new(self.aud, &self.exp, self.sub)?;
        let token = claims.sign()?;
        println!("{} {}", "Jwt token:".blue(), token);
        Ok(())
    }
}

impl CmdExec for JwtVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let claims = Claims::verify(&self.token)?;
        claims.print();
        Ok(())
    }
}
