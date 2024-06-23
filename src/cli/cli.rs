use clap::Parser;

use crate::{CsvOpts, GenPassOpts};

#[derive(Parser, Debug)]
#[command(name = "rcli", about, version, author, long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Subcommand,
    #[arg(
        short,
        long,
        default_value = "false",
        long_help = "Print debug information"
    )]
    pub debug: bool,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
    #[clap(name = "csv", about = "Convert CSV to File")]
    Csv(CsvOpts),
    #[clap(name = "genpass", about = "Generate password for random")]
    GenPass(GenPassOpts),
}
