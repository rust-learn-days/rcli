use clap::Parser;
use colored::Colorize;

use rcli::{csv2file, encode, sign, verify, Base64Subcommand, Opts, Subcommand, TextSubcommand};

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
                !gen_opts.no_upper,
                !gen_opts.no_lower,
                !gen_opts.no_number,
                !gen_opts.no_symbol,
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
        Subcommand::Base64(base64_opts) => match base64_opts.cmd {
            Base64Subcommand::Encode(encode_opts) => {
                if let Err(e) = encode(&encode_opts.input, encode_opts.format) {
                    eprintln!("{} {}", "Error: ".red(), e);
                }
            }
            Base64Subcommand::Decode(decode_opts) => {
                if let Err(e) = rcli::decode(&decode_opts.input, decode_opts.format) {
                    eprintln!("{} {}", "Error: ".red(), e);
                }
            }
        },
        Subcommand::Text(text_opts) => match text_opts.cmd {
            TextSubcommand::GenerateKey(generate_key_opts) => {
                println!("generate_key_opts: {:?}", generate_key_opts)
            }
            TextSubcommand::Sign(sign_opts) => {
                if let Err(e) = sign(&sign_opts.input, &sign_opts.key, sign_opts.format) {
                    eprintln!("{} {}", "Error: ".red(), e);
                }
            }
            TextSubcommand::Verify(verify_opts) => {
                if let Err(e) = verify(
                    &verify_opts.input,
                    &verify_opts.key,
                    verify_opts.format,
                    &verify_opts.signature,
                ) {
                    eprintln!("{} {}", "Error: ".red(), e);
                }
            }
        },
    }
}
