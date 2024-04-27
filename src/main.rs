use clap::builder::TypedValueParser;
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::{env, fs};

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
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}
fn main() -> anyhow::Result<()> {
    let opts = Cli::parse();
    println!("{:?}", opts);
    match opts.cmd {
        Commands::Csv(csv_opts) => {
            let mut reader = Reader::from_path(csv_opts.input)?;
            let mut records = Vec::with_capacity(128);
            for record in reader.deserialize() {
                let record: Player = record?;
                records.push(record);
            }
            let json = serde_json::to_string_pretty(&records)?;
            fs::write(csv_opts.output, json)?;
            // let records = reader
            //     .deserialize()
            //     .map(|result| result.unwrap())
            //     .collect::<Vec<Player>>();
            // let headers = reader.headers().unwrap().clone();
            // let mut records = Vec::new();
            // for record in reader.records() {
            //     records.push(record.unwrap());
            // }
            println!("{:?}", records.len());
            Ok(())
        }
    }

    // Continued program logic goes here...
}
