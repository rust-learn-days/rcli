use std::path::PathBuf;

use clap::Parser;

use crate::{http_server, CmdExec};

use super::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

impl CmdExec for HttpSubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            HttpSubCommand::Serve(serve_opts) => serve_opts.execute().await,
        }
    }
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, default_value = "8080", long_help = "The port to bind to")]
    pub port: u16,
    #[arg(
        short, long, default_value = ".", value_parser = verify_path, long_help = "The directory to serve"
    )]
    pub dir: PathBuf,
}

impl CmdExec for HttpServeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        http_server(self.dir.clone(), self.port).await
    }
}
