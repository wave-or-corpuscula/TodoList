use std::{error::Error, io::Write};

use crossterm::{
    cursor::{Hide, MoveTo, Show, EnableBlinking}, execute, queue, style::Print, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode}
};

pub struct TerminalRenderer {
    stdout: std::io::Stdout,
}

impl TerminalRenderer {
    pub fn new() -> Self {
        Self { stdout: std::io::stdout() }
    }
    
    pub fn clear_screen(&mut self) -> Result<(), Box<dyn Error>> {
        execute!(self.stdout, Clear(ClearType::All), MoveTo(0, 0))?;
        Ok(())
    }
    
    pub fn enter_raw_mode(&mut self) -> Result<(), Box<dyn Error>> {
        enable_raw_mode()?;
        execute!(self.stdout, Hide)?;
        Ok(())
    }
    
    pub fn exit_raw_mode(&mut self) -> Result<(), Box<dyn Error>> {
        execute!(self.stdout, Show)?;
        disable_raw_mode()?;
        Ok(())
    }
    
    pub fn enter_interactive_mode(&mut self) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(self.stdout, 
            Clear(ClearType::All),
            MoveTo(0, 0),
            EnableBlinking,
            Show
        )?;
        Ok(())
    }
    
    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {
        self.stdout.flush()?;
        Ok(())
    }
    
    pub fn print_line(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
        queue!(self.stdout, Print(format!("{}\r\n", text)))?;
        Ok(())
    }

    pub fn print(&mut self, text: &str) -> Result<(), Box<dyn Error>> {
        queue!(self.stdout, Print(format!("{}", text)))?;
        Ok(())
    }
}