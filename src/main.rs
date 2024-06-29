use clap::Parser;
use colored::Colorize;

use rcli::{Opts, Subcommand};

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    if opts.debug {
        println!("{} {:?}", "DEBUG".green(), opts);
    }
    match opts.cmd {
        Subcommand::Csv(csv_opts) => csv_opts.execute()?,
        Subcommand::GenPass(gen_opts) => gen_opts.execute()?,
        Subcommand::Base64(base64_opts) => base64_opts.execute()?,
        Subcommand::Text(text_opts) => text_opts.execute()?,
    }
    Ok(())
}
