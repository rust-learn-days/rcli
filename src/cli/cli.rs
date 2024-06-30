use clap::Parser;

use crate::{Base64Opts, CmdExec, CsvOpts, GenPassOpts, HttpSubCommand, TextOpts};

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

impl Subcommand {
    pub async fn execute(self) -> anyhow::Result<()> {
        match self {
            Subcommand::Csv(csv_opts) => csv_opts.execute().await,
            Subcommand::GenPass(gen_opts) => gen_opts.execute().await,
            Subcommand::Base64(base64_opts) => base64_opts.execute().await,
            Subcommand::Text(text_opts) => text_opts.execute().await,
            Subcommand::Http(http_opts) => http_opts.execute().await,
        }
    }
}
