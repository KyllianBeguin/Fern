use csv::Reader;

use std::error::Error;
use std::process;

// to get the name
use std::fs::File;
use std::env;

fn read_csv(filename: &str) -> Result<(), Box<dyn Error>> {

    // Open the file
    let file = File::open(filename)?;

    // read the file
    let mut rdr= Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    // Get command line argument
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: cargo run <filename>");
        process::exit(1);
    }

    if let Err(err) = read_csv(&args[1]) {
        println!("error reading CSV file: {}", err);
        process::exit(1);
    }
}
