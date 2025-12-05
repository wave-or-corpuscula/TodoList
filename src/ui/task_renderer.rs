use std::error::Error;

use colored::Colorize;
use crossterm::{
    queue, style::Print
};

use crate::task::*;

pub struct TaskRenderer;

impl TaskRenderer {
    pub fn render_task_list(
        tasks: &[TaskWithKids], 
        selected_id: i32
    ) -> Result<(), Box<dyn Error>> {
        if tasks.is_empty() {
            queue!(std::io::stdout(), 
                Print("No tasks\r\n".red()),
                Print("Press [a] to add a task\r\n\r\n"),
            )?;
        } else {
            for task in tasks {
                Self::render_task_tree(task, 0, selected_id)?;
            }
        }
        Ok(())
    }
    
    fn render_task_tree(
        task: &TaskWithKids, 
        depth: u32, 
        selected_id: i32
    ) -> Result<(), Box<dyn Error>> {
        let status_char = if task.task.completed { 
            "✓".green() 
        } else { 
            "○".white() 
        };
        
        let name = if task.task.completed { 
            task.task.name.strikethrough() 
        } else { 
            task.task.name.white() 
        };
        
        let name_colored = if task.task.id as i32 == selected_id {
            name.green().bold()
        } else {
            name
        };
        
        let indent = "  ".repeat(depth as usize);
        queue!(std::io::stdout(), 
            Print(format!("{} {} {}\r\n", indent, status_char, name_colored))
        )?;
        
        for subtask in &task.subtasks {
            Self::render_task_tree(subtask, depth + 1, selected_id)?;
        }
        
        Ok(())
    }
    
    pub fn render_task_details(task: &Task, children: &[Task]) -> Result<(), Box<dyn Error>> {
        println!("📋 Task Details");
        println!("{}", "─".repeat(30));
        println!("📝 Name: {}", task.name);
        println!("✅ Status: {}", if task.completed { "Completed" } else { "In Progress" });
        println!("📅 Created: {}", task.creation_date.format("%Y-%m-%d %H:%M"));

        if let Some(desc) = &task.description {
            println!("📄 Description: {}", desc);
        }

        if !children.is_empty() {
            println!("\n🔗 Subtasks:");
            for (i, child) in children.iter().enumerate() {
                let status = if child.completed { "✓" } else { "○" };
                println!("  {}. {} {}", i + 1, status, child.name);
            }
        }
        Ok(())
    }
    
    pub fn render_main_menu() -> Result<(), Box<dyn Error>> {
        queue!(std::io::stdout(),
            Print("\r\n"),
            Print("Controls:\r\n"),
            Print("↑↓    Navigate\r\n"),
            Print("Enter Task details\r\n"),
            Print("a     Add task\r\n".green()),
            Print("d     Delete task\r\n".red()),
            Print("q     Quit\r\n"),
        )?;
        Ok(())
    }
    
    pub fn render_task_detail_menu() -> Result<(), Box<dyn Error>> {
        println!("{}", "\n1. Add subtask".green());
        println!("{}", "2. Edit task".yellow());
        println!("{}", "3. Delete task".red());
        println!("4. Back");
        Ok(())
    }
}