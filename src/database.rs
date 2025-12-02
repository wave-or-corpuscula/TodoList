use std::{env, error::Error};

use rusqlite::{Connection, Result};

use crate::config::Config;


pub struct DB {
    config: Config,
    connection: Connection,
}

impl DB {
    pub fn connect() -> Result<Self, Box<dyn Error>> {
        let config = Config::build()?;
        let conn = Connection::open(&config.db_path)?;

        Ok (Self {config, connection: conn})
    }
}  