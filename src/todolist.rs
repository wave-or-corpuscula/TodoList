use std::error::Error;
use std::io::stdout;

use crossterm::queue;
use crossterm::style::Print;
use dotenv;
use colored::Colorize;

use crate::task::*;
use crate::database::DB;
use crate::config::Config;


pub struct FlatTask {
    pub task: Task,
    pub depth: u32,
    pub parent_path: Vec<u32>,
    pub index_in_parent: usize,
}


pub struct TodoList {
    tasks: Vec<TaskWithKids>,
    db: DB,
    selected: i32,
}

impl TodoList {
    pub fn new() -> Result<Self, Box<dyn Error>> {

        dotenv::dotenv().ok();
        let config = Config::build()?;
        let db = DB::new(&config)?;
        let tasks = db.select_tasks_hierarchy(None)?;

        Ok(Self { 
            tasks, 
            db,
            selected: -1
        })
    }

    // Navigatoin up
    pub fn select_previous(&mut self) -> bool {
        let flat_tasks = self.get_flat_tasks();

        let current_index = flat_tasks.iter()
            .position(|t| t.task.id as i32 == self.selected);

        match current_index {
            Some(0) => {
                if let Some(last_task) = flat_tasks.last() {
                    self.selected = last_task.task.id as i32;
                    true
                } else {
                    self.selected = -1;
                    false
                }
            },
            Some(index) => {
                self.selected = flat_tasks[index - 1].task.id as i32;
                true
            }
            None => {
                if let Some(first_task) = flat_tasks.first() {
                    self.selected = first_task.task.id as i32;
                    true
                } else {
                    self.selected = -1;
                    false
                }
            }
        }
    }

    pub fn get_selected_children(&self) -> Result<Vec<Task>, Box<dyn Error>> {
        self.db.select_task_subtasks(self.selected as u32)
    }

    // Navigation down
    pub fn select_next(&mut self) -> bool {
        let flat_tasks = self.get_flat_tasks();
        let len = flat_tasks.len();

        let current_index = flat_tasks.iter()
            .position(|t| t.task.id as i32 == self.selected);

        match current_index {
            Some(index) if index < len - 1 => {
                self.selected = flat_tasks[index + 1].task.id as i32;
                true
            }
            _ => {
                if let Some(first_task) = flat_tasks.first() {
                    self.selected = first_task.task.id as i32;
                    true
                } else {
                    self.selected = -1;
                    false
                }
            }
        }
    }

    pub fn refresh_data(&mut self) -> Result<(), Box<dyn Error>> {
        self.tasks = self.db.select_tasks_hierarchy(None)?;
        Ok(())
    }

    pub fn get_selected_task(&self) -> Option<(Task, u32)> {
        let flat_tasks = self.get_flat_tasks();

        flat_tasks.iter()
            .find(|t| t.task.id as i32 == self.selected)
            .map(|ft| (ft.task.clone(), ft.depth))
    }

    pub fn get_flat_tasks(&self) -> Vec<FlatTask> {
        let mut result = Vec::new();
        for root_task in &self.tasks {
            self.flatten_task_tree(root_task, 0, Vec::new(), &mut result);
        }
        result
    }

    fn flatten_task_tree (
        &self,
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
            self.flatten_task_tree(subtask, depth + 1, current_path.clone(), result);
        }
    }


    fn _print_task_tree(&self, task: &TaskWithKids, depth: u32) -> Result<(), Box<dyn Error>> {
        let status_char = if task.task.completed { "✓".green() } else { "○".white() };
        
        let name = if task.task.completed { 
            task.task.name.strikethrough() 
        } else { 
            task.task.name.white() 
        };
        
        let name_colored = if task.task.id as i32 == self.selected {
            name.green().bold()
        } else {
            name
        };
        
        // Отступ в зависимости от глубины
        let indent = "  ".repeat(depth as usize);
        queue!(stdout(), 
            Print(format!("{} {} {}\r\n", indent, status_char, name_colored))
        )?;

        // Рекурсивно выводим подзадачи
        for subtask in &task.subtasks {
            self._print_task_tree(subtask, depth + 1)?;
        }

        Ok(())
    }
    
    pub fn print_tasks(&self) -> Result<(), Box<dyn Error>> {
        if self.tasks.is_empty() {
            queue!(stdout(), 
                Print("Нет задач\r\n".red()),
                Print("Чтобы добавить задачу нажмите клавишу [a]\r\n\r\n"),
            )?;
        }
        for task in &self.tasks {
            self._print_task_tree(task, 0)?;
        }

        Ok(())
    }

    pub fn toggle_completed(&self, id: u32) -> Result<(), Box<dyn Error>> {
        let current = self.get_selected_task();
        match current {
            Some((t, _)) => {
                self.db.update_task(&UpdateTask { 
                    id, 
                    name: None, 
                    parent_id: None, 
                    description: None, 
                    completed: Some(!t.completed as u32),
                })?
            }
            None => ()
        }
        Ok(())
    }

    pub fn toggle_current_completed(&self) -> Result<(), Box<dyn Error>> {
        self.toggle_completed(self.selected as u32)
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
        completed: Option<u32>,
        description: Option<String>,
    ) -> Result<bool, Box<dyn Error>> {
        let update = UpdateTask { id, name, parent_id, description, completed };
        if !update.updated() {
            return Ok(false)
        }
        self.db.update_task(&update)?;
        Ok(true)
    }

    pub fn delete_task(&mut self, id: u32) -> Result<(), Box<dyn Error>> {
        self.selected = -1;
        self.db.delete_task(id)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_tasks() -> Result<(), Box<dyn Error>> {
        let tdlist = TodoList::new()?;
        tdlist.print_tasks()?;
        Ok(())
    }

    #[test]
    fn flatten_the_tree() -> Result<(), Box<dyn Error>> {
        let tdlist = TodoList::new()?;
        let _flat = tdlist.get_flat_tasks();

        Ok(())
    }
}