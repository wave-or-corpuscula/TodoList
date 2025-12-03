use std::error::Error;

use chrono::NaiveDateTime;


pub struct Task {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub name: String,
    pub completed: bool,
    pub description: Option<String>,
    pub creation_date: chrono::NaiveDateTime,
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