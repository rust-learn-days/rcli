use clap::Parser;

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
