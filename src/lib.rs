pub use biz::*;
pub use cli::*;
use enum_dispatch::enum_dispatch;
pub use utils::*;

mod biz;
mod cli;
mod utils;

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExec {
    async fn execute(self) -> anyhow::Result<()>;
}
