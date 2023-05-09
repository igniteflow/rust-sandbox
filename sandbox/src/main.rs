/*
 To run
 > cargo build
 > ./target/debug/sandbox
 */
use std::error::Error;
use std::fs::File;
use csv::Reader;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data.csv")?;
    let mut reader = Reader::from_reader(file);
    for result in reader.records() {
        let record = result?;
        let apples = &record[0];
        let pears = &record[1];
        let oranges = &record[2];
        println!("Apples: {} Pears: {} Oranges: {}", apples, pears, oranges);
    }
    Ok(())
}