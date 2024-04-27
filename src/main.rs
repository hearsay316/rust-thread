use clap::Parser;
use std::env;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, name = "cli")]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Debug, Parser)]
enum Commands {
    #[command(name = "csv", about = take())]
    Csv(CsvOpts),
}
fn take() -> String {
    println!("{}", env::consts::OS);
    format!("转化成Json1{}", env::consts::OS)
}
#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long,value_parser=verify_file_exists)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool,
}
fn verify_file_exists(file_name: &str) -> Result<String, &'static str> {
    if Path::new(file_name).exists() {
        Ok(file_name.to_string())
    } else {
        Err("没有文件")
    }
}
fn main() {
    let opts = Cli::parse();

    println!("{:?}", opts)

    // Continued program logic goes here...
}
