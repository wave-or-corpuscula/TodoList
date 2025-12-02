use std::{error::Error, fs};

use dotenv;

use todolist::config::Config;
use todolist::database::DB;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let config = Config::build()?;
    let db = DB::new(&config);

    Ok(())
}
