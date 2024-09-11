// FROM HERE
// https://docs.rs/csv/latest/csv/


use std::{error::Error, io, process};

#[derive(Debug, serde::Deserialize)]
struct Record {
    #[allow(dead_code)]
    city: String,
    #[allow(dead_code)]
    region: String,
    #[allow(dead_code)]
    country: String,
    #[allow(dead_code)]
    population: Option<u64>,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

/* 
cargo run --example explore_csv_serde < /home/trapapa/docker_debian_12_ti/src/workspace/new_thirtyfour/result.csv


Module csv::tutorial
https://docs.rs/csv/latest/csv/tutorial/index.html
https://docs.rs/csv/latest/csv/tutorial/index.html#setup

*/
