use clap::Parser;
use colored::Colorize;

use rcli::{csv2file, Opts, Subcommand};

fn main() {
    let opts: Opts = Opts::parse();
    if opts.debug {
        println!("{} {:?}", "DEBUG".green(), opts);
    }
    match opts.cmd {
        Subcommand::Csv2File(csv_opts) => {
            println!("{} {}", "Convert CSV to JSON".blue(), "format: json".blue());
            match csv2file(csv_opts) {
                Ok(output) => {
                    if opts.debug {
                        println!("{}", jsonxf::pretty_print(output.as_str()).unwrap().green());
                    }
                    println!("{}", "Success Convert CSV to JSON".blue());
                }
                Err(e) => {
                    eprintln!("{} {}", "Error: {}".red(), e);
                }
            }
        }
    }
}
