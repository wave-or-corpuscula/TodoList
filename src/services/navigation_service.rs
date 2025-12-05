use crate::{task::{Task, TaskWithKids}, todolist::FlatTask};

pub struct NavigationService {
    flat_tasks: Vec<FlatTask>,
}

impl NavigationService {
    pub fn new(tasks: &[TaskWithKids]) -> Self {
        let mut result = Vec::new();
        for root_task in tasks {
            Self::flatten_task_tree(root_task, 0, Vec::new(), &mut result);
        }
        
        Self { flat_tasks: result }
    }

    pub fn is_empty(&self) -> bool {
        self.flat_tasks.is_empty()
    }
    
    pub fn get_next_id(&self, current_id: i32) -> Option<i32> {
        let current_index = self.find_index_by_id(current_id)?;
        
        if current_index < self.flat_tasks.len() - 1 {
            Some(self.flat_tasks[current_index + 1].task.id as i32)
        } else if !self.flat_tasks.is_empty() {
            Some(self.flat_tasks[0].task.id as i32)
        } else {
            None
        }
    }
    
    pub fn get_previous_id(&self, current_id: i32) -> Option<i32> {
        let current_index = self.find_index_by_id(current_id)?;
        
        if current_index > 0 {
            Some(self.flat_tasks[current_index - 1].task.id as i32)
        } else if !self.flat_tasks.is_empty() {
            Some(self.flat_tasks.last().unwrap().task.id as i32)
        } else {
            None
        }
    }
    
    pub fn get_task_with_depth(&self, task_id: i32) -> Option<(Task, u32)> {
        self.flat_tasks.iter()
            .find(|ft| ft.task.id as i32 == task_id)
            .map(|ft| (ft.task.clone(), ft.depth))
    }
    
    pub fn get_first_id(&self) -> Option<i32> {
        self.flat_tasks.first()
            .map(|ft| ft.task.id as i32)
    }
    
    fn find_index_by_id(&self, task_id: i32) -> Option<usize> {
        self.flat_tasks.iter()
            .position(|ft| ft.task.id as i32 == task_id)
    }
    
    fn flatten_task_tree(
        task_node: &TaskWithKids,
        depth: u32,
        parent_path: Vec<u32>,
        result: &mut Vec<FlatTask>,
    ) {
        let mut current_path = parent_path.clone();
        current_path.push(task_node.task.id);
        
        result.push(FlatTask { 
            task: task_node.task.clone(), 
            depth, 
            parent_path: current_path.clone(), 
            index_in_parent: result.len(),
        });
        
        for subtask in &task_node.subtasks {
            Self::flatten_task_tree(subtask, depth + 1, current_path.clone(), result);
        }
    }
}