use std::fmt::{Display, Formatter, Result};


pub struct Task {
    id: u32,
    parent_id: Option<u32>,
    name: String,
    completed: bool,
    description: Option<String>,
    creation_date: chrono::DateTime<chrono::Utc>,
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<Task id: {}, name: {} >", self.id, self.name)
    }
}

