use std::collections::HashMap;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::fs;
use serde_json::Value;

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
    let mut ret = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for record in reader.records() {
        let record = record?;
        let mut json_value = HashMap::new();
        for i in 0..headers.len() {
            json_value.insert(headers[i].to_string(), record[i].to_string());
        }
        // let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value);
        // println!("{:?}", map);
    }
    // println!("{:?}", ret);
    let json = serde_json::to_string_pretty(&ret)?;
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
    // println!("{:?}", records.len());
    Ok(())
}
