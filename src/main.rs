use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, name = "cli")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Parser)]
enum Commands {
    #[command(name = "csv", about = "转化成Json")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}

fn main() {
    let opts = Cli::parse();

    println!("{:?}", opts)

    // Continued program logic goes here...
}
