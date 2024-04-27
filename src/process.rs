use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
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
pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut records = Vec::with_capacity(128);
    for record in reader.deserialize() {
        let record: Player = record?;
        records.push(record);
    }
    let json = serde_json::to_string_pretty(&records)?;
    fs::write(output, json)?;
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
