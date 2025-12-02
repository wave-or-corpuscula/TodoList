use std::{env, error::Error};


pub struct Config {
    pub db_path: String,
}

impl Config {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let db_path = env::var("DB_PATH")?.parse()?;
        Ok(Self { db_path })
    }    
}