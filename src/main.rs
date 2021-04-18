#[allow(unused_imports)]

use std::error::Error;
use std::io;
use std::process;

fn main() {
    // graceful exit
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

// Box<dyn Error> means 'any kind of error' - hard to inspect
fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.records() {
        // the match block can be replaced with syntatic sugar `?`
        // let record = result?;
        // println!("{:?}", record);
        match result {
            Err(err) => return Err(From::from(err)),
            Ok(record) => {
                println!("{:?}", record);
            }
        }
    }
    Ok(())
}