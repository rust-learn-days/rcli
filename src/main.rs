use clap::Parser;
use colored::Colorize;

use rcli::{csv2file, Opts, Subcommand};

fn main() {
    let opts: Opts = Opts::parse();
    if opts.debug {
        println!("{} {:?}", "DEBUG".green(), opts);
    }
    match opts.cmd {
        Subcommand::Csv(csv_opts) => {
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
        Subcommand::GenPass(gen_opts) => {
            let pass = rcli::gen_pass(
                gen_opts.length,
                gen_opts.upper,
                gen_opts.lower,
                gen_opts.number,
                gen_opts.symbol,
            );
            match pass {
                Ok(pass) => {
                    println!("{} {}", "Generated Password: ".blue(), pass.blue());
                }
                Err(e) => {
                    eprintln!("{} {}", "Error: ".red(), e);
                }
            }
        }
    }
}
