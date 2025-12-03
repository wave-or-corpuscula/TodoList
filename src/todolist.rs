use std::error::Error;

use dotenv;

use crate::task::*;
use crate::database::DB;
use crate::config::Config;


pub struct TodoList {
    tasks: Vec<TaskWithKids>,
    db: DB,
}

impl TodoList {
    // pub fn new() -> Result<Self, Box<dyn Error>> {

    //     dotenv::dotenv().ok();
    //     let config = Config::build()?;
    //     let db = DB::new(&config)?;

    //     Ok(Self { 
    //         tasks: (), 
    //         db 
    //     })
    // }
}