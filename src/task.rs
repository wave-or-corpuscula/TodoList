use std::{collections::HashMap, error::Error};

use rusqlite::Row;
use chrono::NaiveDateTime;


#[derive(Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub name: String,
    pub completed: bool,
    pub description: Option<String>,
    pub creation_date: chrono::NaiveDateTime,
}


impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<Task id: {}, name: {} >", self.id, self.name)
    }
}

impl Task {
    pub fn from_select(select: SelectTask) -> Result<Self, Box<dyn Error>> {
        let parse_date = NaiveDateTime::parse_from_str(
            &select.creation_date, 
            "%Y-%m-%d %H:%M:%S")?;
        Ok(Self {
            id: select.id,
            parent_id: select.parent_id,
            name: select.name,
            completed: select.completed,
            description: select.description,
            creation_date: parse_date
        })
    }
}

pub struct TaskWithKids {
    pub task: Task,
    pub subtasks: Vec<TaskWithKids>
}

impl TaskWithKids {
    pub fn get_recursive(
        parent: Task, 
        by_parent: &HashMap<Option<u32>, Vec<Task>>
    ) -> Self {
        let children = by_parent.get(&Some(parent.id));

        match children {
            Some(child_tasks) => {
                let mut subtasks = Vec::new();
                for child in child_tasks {
                    subtasks.push(TaskWithKids::get_recursive(child.clone(), by_parent));
                }
                TaskWithKids {
                    task: parent,
                    subtasks
                }
            }
            None => {
                TaskWithKids {
                    task: parent,
                    subtasks: Vec::new()
                }
            }
        }

    }
}

pub struct CreateTask {
    pub name: String,
    pub parent_id: Option<u32>,
    pub description: Option<String>,
}

pub struct UpdateTask {
    pub id: u32,
    pub name: Option<String>,
    pub parent_id: Option<u32>,
    pub description: Option<String>,
}

pub struct SelectTask {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub name: String,
    pub completed: bool,
    pub description: Option<String>,
    pub creation_date: String, 
}

impl SelectTask {
    pub fn from_row(row: &Row<'_>) -> Result<Self, rusqlite::Error> {
        Ok(Self {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            name: row.get(2)?,
            completed: row.get(3)?,
            description: row.get(4)?,
            creation_date: row.get(5)?
        })
    }
}