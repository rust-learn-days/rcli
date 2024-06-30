use clap::Parser;
use enum_dispatch::enum_dispatch;

use crate::{
    Base64Subcommand, CsvOpts, GenPassOpts, HttpSubCommand, JwtSubCommand, TextSubcommand,
};

#[derive(Parser, Debug)]
#[command(name = "rcli", about, version, author, long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Parser, Debug)]
#[enum_dispatch(CmdExec)]
pub enum Subcommand {
    #[clap(name = "csv", about = "Convert CSV to File")]
    Csv(CsvOpts),
    #[clap(name = "genpass", about = "Generate password for random")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "Encode/Decode Base64")]
    Base64(Base64Subcommand),
    #[command(subcommand, about = "Convert text to file")]
    Text(TextSubcommand),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
    #[command(subcommand, about = "JWT")]
    Jwt(JwtSubCommand),
}
