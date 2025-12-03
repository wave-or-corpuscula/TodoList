use std::collections::HashMap;
use std::error::Error;

use rusqlite::{Connection, Params, Result};

use crate::task::*;
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

    pub fn select_tasks_hierarchy(&self, completed: Option<bool>) -> Result<Vec<TaskWithKids>, Box<dyn Error>> {
        let all_tasks = self.select_tasks()?;

        let mut by_parent: HashMap<Option<u32>, Vec<Task>> = HashMap::new();

        for task in all_tasks {
            by_parent.entry(task.parent_id).or_default().push(task);
        }

        let root_tasks = by_parent.remove(&None).unwrap_or_default();

        let mut task_with_kids: Vec<TaskWithKids> = Vec::new();
        for task in root_tasks {
            task_with_kids.push(TaskWithKids::get_recursive(task, &by_parent));
        }

        Ok(task_with_kids)
    }

    pub fn select_completed_tasks(&self, completed: bool) -> Result<Vec<Task>, Box<dyn Error>> {
        self.query_to_tasks(
            "SELECT * FROM Task WHERE completed = ?1", 
            [if completed { 1 } else { 0 }] )
    }


    pub fn select_tasks(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        self.query_to_tasks("SELECT * FROM Task", [])
    }

    fn query_to_tasks<P: Params> (&self, query: &str, params: P) -> Result<Vec<Task>, Box<dyn Error>>  {
        let mut stmt = self.connection.prepare(query)?;
        let task_iter = stmt.query_map(params, |row| {
            Ok(SelectTask::from_row(row)?)
        })?;

        let mut result = Vec::new();
        for task in task_iter {
            result.push(Task::from_select(task?)?);
        }

        Ok(result)
        
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
    fn selecting_hierarchy() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().ok();
        let config = Config::build()?;
        let db = DB::new(&config)?;

        let _tasks = db.select_tasks_hierarchy(None)?;

        Ok(())
    }

    #[test]
    fn db_creation() -> Result<(), Box<dyn Error>>{
        dotenv::dotenv().ok();
        let config = Config::build()?;
        let _db = DB::new(&config);
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
    fn task_selection() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().ok();
        let config = Config::build()?;
        let db = DB::new(&config)?;

        let tasks = db.select_tasks()?;
        for task in tasks {
            println!("{}", &task);
        }

        Ok(())
    }

    #[test]
    fn task_updating() -> Result<(), Box<dyn Error>> {
        dotenv::dotenv().ok();
        let config = Config::build()?;
        let db = DB::new(&config)?;

        let update_task = UpdateTask {
            id: 9,
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

        db.delete_task(14)?;

        Ok(())
    }

}