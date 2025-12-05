use std::error::Error;

use crate::{database::DB, task::*};


pub struct TaskService {
    db: DB,
}

impl TaskService {
    pub fn new(db: DB) -> Self {
        Self { db }
    }
    
    pub fn load_hierarchy(&self) -> Result<Vec<TaskWithKids>, Box<dyn Error>> {
        self.db.select_tasks_hierarchy(None)
    }
    
    pub fn get_task_by_id(&self, id: u32) -> Result<Task, Box<dyn Error>> {
        let tasks = self.db.select_tasks(None)?;
        tasks.into_iter()
            .find(|t| t.id == id)
            .ok_or("Task not found".into())
    }
    
    pub fn get_children(&self, parent_id: u32) -> Result<Vec<Task>, Box<dyn Error>> {
        self.db.select_task_subtasks(parent_id)
    }
    
    pub fn toggle_task_completion(&self, task_id: u32) -> Result<(), Box<dyn Error>> {
        let task = self.get_task_by_id(task_id)?;
        self.db.update_task(&UpdateTask {
            id: task_id,
            name: None,
            parent_id: None,
            description: None,
            completed: Some(if task.completed { 0 } else { 1 }),
        })?;
        Ok(())
    }
    
    pub fn create_task(
        &self, 
        name: String, 
        parent_id: Option<u32>,
        description: Option<String>
    ) -> Result<i32, Box<dyn Error>> {
        self.db.create_task(&CreateTask { parent_id, name, description })
    }
    
    pub fn update_task(
        &self,
        task_id: u32,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<bool, Box<dyn Error>> {
        let update = UpdateTask {
            id: task_id,
            name,
            parent_id: None,
            description,
            completed: None,
        };
        
        if !update.updated() {
            return Ok(false);
        }
        
        self.db.update_task(&update)?;
        Ok(true)
    }
    
    pub fn delete_task(&self, task_id: u32) -> Result<(), Box<dyn Error>> {
        self.db.delete_task(task_id)
    }
}