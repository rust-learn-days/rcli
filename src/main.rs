use clap::Parser;

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts)
}

#[derive(Parser, Debug)]
#[command(name = "rcli", about, version, author, long_about = None)]
struct Opts {
    #[clap(subcommand)]
    cmd: Subcommand,
}

#[derive(Parser, Debug)]
enum Subcommand {
    #[clap(name = "csv2json", about = "Convert CSV to JSON")]
    Csv2Json(Csv2JsonOpts),
    #[clap(name = "json2csv", about = "Convert JSON to CSV")]
    Json2Csv(Json2CsvOpts),
}

#[derive(Parser, Debug)]
struct Csv2JsonOpts {
    #[arg(short, long)]
    input: String,
    #[arg(short, long, default_value = "output.json")]
    output: String,
    #[arg(short, long, default_value = ",")]
    delimiter: char,
    #[arg(short = 'r', long = "show header", default_value_t = true)]
    header: bool,
}

#[derive(Parser, Debug)]
struct Json2CsvOpts {
    #[arg(short, long = "Input JSON file")]
    input: String,
    #[arg(short, long = "Output CSV file")]
    output: String,
}
