// FROM HERE
// https://docs.rs/csv/latest/csv/

// read csv -> result.csv => generate from src/workspace/new_thirtyfour/examples/tokio_finviz_method_five.rs

use std::{error::Error, io, process};

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
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
cargo run --example explore_csv < /home/trapapa/docker_debian_12_ti/src/workspace/new_thirtyfour/result.csv

*/