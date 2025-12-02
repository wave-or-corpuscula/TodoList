use std::path::Path;
use std::error::Error;

use rusqlite::{Connection, Result};

use crate::task::{CreateTask, UpdateTask, Task};
use crate::config::Config;

pub struct DB {
    connection: Connection,
}

impl DB {

    pub fn new(config: &Config) -> Result<Self, Box<dyn Error>> {
        let connection = Connection::open(&config.db_path)?;
        connection.execute("PRAGMA foreign_keys = ON", ())?;
        connection.execute(
        "CREATE TABLE IF NOT EXISTS Task (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                parent_id     INTEGER DEFAULT NULL,
                name          TEXT NOT NULL,
                completed     INTEGER DEFAULT 0,
                description   TEXT,
                creation_date TEXT DEFAULT CURRENT_TIMESTAMP,
                
                FOREIGN KEY (parent_id) REFERENCES Task(id) ON DELETE CASCADE
            );",
        ()
        )?;


        Ok (Self {connection})
    }

    pub fn select_tasks(&self) -> Result<(), Box<dyn Error>> {
        let mut stmt = self.connection.prepare("SELECT * FROM Task")?;
        let task_iter = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                parent_id: row.get(1)?,
                name: row.get(2)?,
                completed: row.get(3)?,
                description: row.get(4)?,
                creation_date: row.get(5)?

            })
        })?;



        Ok(())
    }

    pub fn create_task(&self, task: &CreateTask) -> Result<(), Box<dyn Error>>{
        self.connection.execute(
            "INSERT INTO Task (parent_id, name, description) VALUES (?1, ?2, ?3)",
        (
            &task.parent_id,
            &task.name,
            &task.description
        ))?;

        Ok(())
    }

    pub fn update_task(&self, task: &UpdateTask) -> Result<(), Box<dyn Error>> {
        let mut params = Vec::new();
        let mut query = "UPDATE Task SET ".to_string();
        if let Some(ref name) = task.name {
            query.push_str("name = ?, ");
            params.push(name as &dyn rusqlite::ToSql);
        }
        if let Some(ref description) = task.description {
            query.push_str("description = ?, ");
            params.push(description as &dyn rusqlite::ToSql);
        }
        if let Some(ref parent_id) = task.parent_id {
            query.push_str("parent_id = ?, ");
            params.push(parent_id as &dyn rusqlite::ToSql);
        }
        query.pop(); query.pop();

        query.push_str("WHERE id = ?");

        params.push(&task.id as &dyn rusqlite::ToSql);

        let mut stmt = self.connection.prepare(&query)?;
        stmt.execute(rusqlite::params_from_iter(params))?;

        Ok(())
    }

    pub fn delete_task(&self, task_id: u32) -> Result<(), Box<dyn Error>> {
        self.connection.execute(
            "DELETE FROM Task WHERE id = ?1", 
        (task_id,))?;
        
        Ok(())
    }
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn db_creation() -> Result<(), Box<dyn Error>>{
        dotenv::dotenv().ok();
        let config = Config::build()?;
        let db = DB::new(&config);
        Ok(())
    }

    #[test]
    fn task_creation() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().ok();
        let config = Config::build()?;
        let db = DB::new(&config)?;

        let create_task = CreateTask{ 
            name: String::from("Cleaning"), 
            parent_id: None, 
            description: None 
        };
        db.create_task(&create_task)?;

        Ok(())
    }

    #[test]
    fn task_updating() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().ok();
        let config = Config::build()?;
        let db = DB::new(&config)?;

        let update_task = UpdateTask {
            id: 1,
            name: None,
            parent_id: None,
            description: Some(String::from("New description")),
        };

        db.update_task(&update_task)?;

        Ok(())
    }

    #[test]
    fn task_deletion() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().ok();
        let config = Config::build()?;
        let db = DB::new(&config)?;

        db.delete_task(2)?;

        Ok(())
    }

}