use clap::Parser;
use colored::Colorize;

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,
    #[arg(long, default_value_t = false)]
    pub no_number: bool,
    #[arg(long, default_value_t = false)]
    pub no_upper: bool,
    #[arg(long, default_value_t = false)]
    pub no_lower: bool,
    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

impl GenPassOpts {
    pub fn execute(self) -> anyhow::Result<()> {
        let pass = crate::gen_pass(
            self.length,
            !self.no_upper,
            !self.no_lower,
            !self.no_number,
            !self.no_symbol,
        );
        match pass {
            Ok(pass) => {
                println!("{} {}", "Generated Password: ".blue(), pass.blue());
                Ok(())
            }
            Err(e) => {
                eprintln!("{} {}", "Error: ".red(), e);
                Err(e)
            }
        }
    }
}
