use clap::Parser;
use rcli::{csv2json, Opts, Subcommand};

fn main() {
    let opts: Opts = Opts::parse();
    if opts.debug {
        println!("{:?}", opts);
    }
    match opts.cmd {
        Subcommand::Csv2Json(opts) => {
            if let Err(e) = csv2json(opts) {
                eprintln!("Error: {}", e);
            }
        }
        Subcommand::Json2Csv(_) => {
            println!("Json2Csv");
        }
    }
}
