pub use biz::*;
pub use cli::*;
pub use utils::*;

mod biz;
mod cli;
mod utils;

#[allow(async_fn_in_trait)]
pub trait CmdExec {
    async fn execute(self) -> anyhow::Result<()>;
}
