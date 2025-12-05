use std::error::Error;

use crossterm::{
    event::KeyCode, queue, style::Print
};
use colored::Colorize;

use crate::{config::Config, database::DB, services::{navigation_service::NavigationService, task_service::TaskService}, task::*, ui::{input::InputHandler, task_renderer::TaskRenderer, terminal::TerminalRenderer}};


pub struct TodoTUI {
    task_service: TaskService,
    navigation: NavigationService,
    renderer: TerminalRenderer,
    selected_id: i32,
    running: bool,
}

impl TodoTUI {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let task_service = TaskService::new(DB::new(&Config::build()?)?);
        let tasks = task_service.load_hierarchy()?;
        let navigation = NavigationService::new(&tasks);
        let renderer = TerminalRenderer::new();
        
        Ok(Self {
            task_service,
            selected_id: navigation.get_first_id().unwrap_or(-1),
            navigation,
            renderer,
            running: true,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.renderer.enter_raw_mode()?;
        
        while self.running {
            self.refresh_navigation()?;
            self.render_main_view()?;
            self.handle_main_events()?;
        }
        
        self.renderer.exit_raw_mode()?;
        Ok(())
    }

    fn refresh_navigation(&mut self) -> Result<(), Box<dyn Error>> {
        let tasks = self.task_service.load_hierarchy()?;
        self.navigation = NavigationService::new(&tasks);
        Ok(())
    }

    fn render_main_view(&mut self) -> Result<(), Box<dyn Error>> {
        self.renderer.clear_screen()?;
        
        queue!(std::io::stdout(), Print("Your tasks:\r\n\r\n".cyan()))?;
        
        let tasks = self.task_service.load_hierarchy()?;
        TaskRenderer::render_task_list(&tasks, self.selected_id)?;
        TaskRenderer::render_main_menu()?;
        
        self.renderer.flush()?;
        Ok(())
    }

    fn handle_main_events(&mut self) -> Result<(), Box<dyn Error>> {
        match InputHandler::read_key()? {
            KeyCode::Up => self.navigate_up()?,
            KeyCode::Down => self.navigate_down()?,
            KeyCode::Enter => self.show_task_details()?,
            KeyCode::Tab => self.toggle_task_completion()?,
            KeyCode::Char('a') => self.add_task(None)?,
            KeyCode::Char('d') => self.delete_selected_task()?,
            KeyCode::Char('q') => self.running = false,
            _ => {}
        }
        Ok(())
    }

    fn navigate_up(&mut self) -> Result<(), Box<dyn Error>> {
        if self.selected_id == -1 {
            return Ok(()); // No tasks for navigation
        }
        if let Some(new_id) = self.navigation.get_previous_id(self.selected_id) {
            self.selected_id = new_id;
        }
        Ok(())
    }

    fn navigate_down(&mut self) -> Result<(), Box<dyn Error>> {
        if self.selected_id == -1 {
            return Ok(()); // No tasks for navigation
        }
        if let Some(new_id) = self.navigation.get_next_id(self.selected_id) {
            self.selected_id = new_id;
        }
        Ok(())
    }

    fn toggle_task_completion(&mut self) -> Result<(), Box<dyn Error>> {
        if self.selected_id == -1 {
            return Ok(()); // No task selected
        }
        self.task_service.toggle_task_completion(self.selected_id as u32)?;
        Ok(())
    }

    fn show_task_details(&mut self) -> Result<(), Box<dyn Error>> {
        if self.selected_id == -1 {
            return Ok(()); // No task selected
        }
        self.renderer.enter_interactive_mode()?;
        
        loop {
            let task = match self.navigation.get_task_with_depth(self.selected_id) {
                Some((t, _)) => t,
                None => return Ok(())
            };
            let children = self.task_service.get_children(task.id)?;
            TaskRenderer::render_task_details(&task, &children)?;
            TaskRenderer::render_task_detail_menu()?;
            
            let choice = InputHandler::read_choice("Your choice: ")?;
            
            match choice {
                1 => {
                    self.renderer.clear_screen()?;
                    self.add_subtask(task.id)?;
                    self.renderer.clear_screen()?;
                },
                2 => {
                    self.renderer.clear_screen()?;
                    let updated = self.change_task_data(&task)?;
                    self.renderer.clear_screen()?;
                    if updated {
                        self.refresh_navigation()?;
                    }
                    println!("{}", "Data updated!\n".green());
                },
                3 => {
                    self.renderer.clear_screen()?;
                    if InputHandler::confirm_deletion(&task.name)? {
                        self.task_service.delete_task(task.id)?;
                        self.selected_id = self.navigation.get_first_id().unwrap_or(-1);
                        break;
                    }
                },
                _ => break
            }
        }
            
            self.renderer.enter_raw_mode()?;
        Ok(())
    }

    fn add_task(&mut self, parent_id: Option<u32>) -> Result<(), Box<dyn Error>> {
        self.renderer.enter_interactive_mode()?;
        let insert_id = self._add_task(parent_id)?;
        if self.navigation.is_empty() {
            self.selected_id = insert_id;
        }
        self.renderer.enter_raw_mode()?;
        Ok(())
    }

    fn _add_task(&mut self, parent_id: Option<u32>) -> Result<i32, Box<dyn Error>> {
        println!("{}", "\n➕ Adding task...".green());
        
        let name = InputHandler::read_text("Enter task name [Enter to cancel]: ")?;
        if name.is_empty() {
            return Ok(-1);
        }
        
        let desc = InputHandler::read_text("Enter task description [Enter to skip]: ")?;
        let description = if desc.is_empty() { None } else { Some(desc) };
        
        let insert_id = self.task_service.create_task(name, parent_id, description)?;
        
        self.renderer.clear_screen()?;
        println!("{}", "Task added!\n".green());
        
        // Wait for Enter to continue
        InputHandler::read_text("Press Enter to continue...")?;
        Ok(insert_id)
    }

    fn add_subtask(&mut self, task_id: u32) -> Result<(), Box<dyn Error>> {
        self._add_task(Some(task_id))?;
        Ok(())
    }

    fn change_task_data(&mut self, task: &Task) -> Result<bool, Box<dyn Error>> {
        println!("Enter new data for task:");

        let name = InputHandler::read_text(&format!("Name [{}]: ", task.name))?;
        let description = InputHandler::read_text("Description [Enter to skip]: ")?;
        
        let name_final = if name.is_empty() { None } else { Some(name) };
        let desc_final = if description.is_empty() { None } else { Some(description) };
        
        let updated = self.task_service.update_task(task.id, name_final, desc_final)?;

        InputHandler::read_text("Press Enter to continue...")?;

        Ok(updated)
    }

    fn delete_selected_task(&mut self) -> Result<(), Box<dyn Error>> {

        if self.selected_id == -1 {
            self.renderer.enter_interactive_mode()?;
            self.renderer.clear_screen()?;
            println!("{}", "No task selected for deletion\n".yellow());
            InputHandler::read_text("Press Enter to continue...")?;
            self.renderer.enter_raw_mode()?;
            return Ok(());
        }
        if let Some((task, _)) = self.navigation.get_task_with_depth(self.selected_id) {
            self.renderer.enter_interactive_mode()?;
            
            if InputHandler::confirm_deletion(&task.name)? {
                self.task_service.delete_task(task.id)?;
                self.refresh_navigation()?;
                self.selected_id = self.navigation.get_first_id().unwrap_or(-1);
                
                self.renderer.clear_screen()?;
                println!("{}", "Task deleted!\n".red());
            } else {
                self.renderer.clear_screen()?;
                println!("{}", "Deletion cancelled\n");
            }

            // Wait for Enter to continue
            InputHandler::read_text("Press Enter to continue...")?;
            
            self.renderer.enter_raw_mode()?;
        }
        Ok(())
    }
}
