use clap::Parser;

use rcli::{Opts, Subcommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    match opts.cmd {
        Subcommand::Csv(csv_opts) => csv_opts.execute()?,
        Subcommand::GenPass(gen_opts) => gen_opts.execute()?,
        Subcommand::Base64(base64_opts) => base64_opts.execute()?,
        Subcommand::Text(text_opts) => text_opts.execute()?,
        Subcommand::Http(http_opts) => http_opts.execute().await?,
    }
    Ok(())
}
