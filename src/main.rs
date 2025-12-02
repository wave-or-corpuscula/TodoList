use std::{error::Error, fs};

mod task;
use dotenv;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    fs::File::open("hello.txt")?;
    println!("Hello, world!");

    Ok(())
}
