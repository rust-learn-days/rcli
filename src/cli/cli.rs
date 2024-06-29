use clap::Parser;

use crate::{Base64Opts, CsvOpts, GenPassOpts, HttpSubCommand, TextOpts};

#[derive(Parser, Debug)]
#[command(name = "rcli", about, version, author, long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Subcommand,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
    #[clap(name = "csv", about = "Convert CSV to File")]
    Csv(CsvOpts),
    #[clap(name = "genpass", about = "Generate password for random")]
    GenPass(GenPassOpts),
    #[clap(name = "base64", about = "Encode/Decode Base64")]
    Base64(Base64Opts),
    #[clap(name = "text", about = "Convert text to file")]
    Text(TextOpts),
    #[command(subcommand, about = "HTTP server")]
    Http(HttpSubCommand),
}
