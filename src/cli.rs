use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rcli", about, version, author, long_about = None)]
pub struct Opts {
    #[clap(subcommand)]
    pub cmd: Subcommand,
    #[arg(
        short,
        long,
        default_value = "false",
        long_help = "Print debug information"
    )]
    pub debug: bool,
}

#[derive(Parser, Debug)]
pub enum Subcommand {
    #[clap(name = "csv2json", about = "Convert CSV to JSON")]
    Csv2Json(Csv2JsonOpts),
}

#[derive(Parser, Debug)]
pub struct Csv2JsonOpts {
    #[arg(short, long, value_parser = verify_file_exists)]
    pub input: String,
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    #[arg(short, long, default_value = ",")]
    pub delimiter: char,
    #[arg(short = 'r', long = "show header", default_value_t = true)]
    pub header: bool,
}

fn verify_file_exists(path: &str) -> Result<String, String> {
    if std::path::Path::new(path).exists() {
        Ok(path.into())
    } else {
        Err(format!("File not found: {}", path))
    }
}

#[derive(Parser, Debug)]
pub struct Json2CsvOpts {
    #[arg(short, long = "Input JSON file")]
    pub input: String,
    #[arg(short, long = "Output CSV file")]
    pub output: String,
}
