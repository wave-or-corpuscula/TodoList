use std::{error::Error};

use dotenv;

use todolist::config::Config;
use todolist::database::DB;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();
    let config = Config::build()?;
    let _db = DB::new(&config);

    Ok(())
}
