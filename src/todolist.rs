use std::error::Error;

use dotenv;
use colored::Colorize;

use crate::task::*;
use crate::database::DB;
use crate::config::Config;


pub struct TodoList {
    tasks: Vec<TaskWithKids>,
    db: DB,
}

impl TodoList {
    pub fn new() -> Result<Self, Box<dyn Error>> {

        dotenv::dotenv().ok();
        let config = Config::build()?;
        let db = DB::new(&config)?;
        let tasks = db.select_tasks_hierarchy(None)?;

        Ok(Self { 
            tasks, 
            db 
        })
    }

    fn _print_task(&self, task: &TaskWithKids, depth: u32) {
        println!("{}{} {}", 
            "  ".repeat(2 * depth as usize), 
            ["○".black(), "✓".green()][task.task.completed as usize],
            [task.task.name.white(), task.task.name.strikethrough()][task.task.completed as usize]
        );
        if !task.subtasks.is_empty() {
            for t in &task.subtasks {
                self._print_task(&t, depth + 1);
            }
        }
    }

    pub fn print_tasks(&self) {
        for task in &self.tasks {
            self._print_task(task, 0);
        }
    }

    pub fn create_task(
        &self, 
        name: String, 
        parent_id: Option<u32>,
        description: Option<String>
     ) -> Result<(), Box<dyn Error>> {
        self.db.create_task(&CreateTask { parent_id, name, description })
    }

    pub fn update_task(
        &self, 
        id: u32,
        name: Option<String>,
        parent_id: Option<u32>,
        description: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        self.db.update_task(&UpdateTask { id, name, parent_id, description })
    }

    pub fn delete_task(&self, id: u32) -> Result<(), Box<dyn Error>> {
        self.db.delete_task(id)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_tasks() -> Result<(), Box<dyn Error>> {
        let todolist = TodoList::new()?;
        todolist.print_tasks();
        Ok(())
    }
}