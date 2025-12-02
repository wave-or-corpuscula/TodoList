use std::fmt::{Display, Formatter, Result};


pub struct Task {
    pub id: u32,
    pub parent_id: Option<u32>,
    pub name: String,
    pub completed: bool,
    pub description: Option<String>,
    pub creation_date: chrono::DateTime<chrono::Utc>,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Task id: {}, name: {} >", self.id, self.name)
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