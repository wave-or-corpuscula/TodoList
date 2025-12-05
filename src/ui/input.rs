use std::{error::Error, io::{BufRead, Write}};

use crossterm::event::{Event, KeyCode, read};
use colored::Colorize;

pub struct InputHandler;

impl InputHandler {
    pub fn read_key() -> Result<KeyCode, Box<dyn Error>> {
        match read()? {
            Event::Key(key_event) => Ok(key_event.code),
            _ => Ok(KeyCode::Null),
        }
    }
    
    pub fn read_text(prompt: &str) -> Result<String, Box<dyn Error>> {
        print!("{}", prompt);
        std::io::stdout().flush()?;
        
        let mut input = String::new();
        std::io::stdin().lock().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
    
    pub fn read_choice(prompt: &str) -> Result<u32, Box<dyn Error>> {
        loop {
            let input = Self::read_text(prompt)?;
            match input.parse::<u32>() {
                Ok(num) if num > 0 && num < 5 => return Ok(num),
                _ => println!("Please enter a number from 1 to 4"),
            }
        }
    }
    
    pub fn confirm_deletion(task_name: &str) -> Result<bool, Box<dyn Error>> {
        loop {
            println!("\n🗑️ Deleting task: {}", task_name.red());
            println!("Are you sure you want to delete this task (all subtasks will also be deleted)?");
            println!("{}", "1. Yes".red());
            println!("2. No");

            let choice = Self::read_choice("Your choice: ")?;
            match choice {
                1 => return Ok(true),
                2 => return Ok(false),
                _ => println!("Please choose one of the options"),
            }
        }
    }
}