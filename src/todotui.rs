use std::{error::Error, io::{BufRead, Write, stdin, stdout}};

use crossterm::{
    cursor::{Hide, MoveTo, Show, EnableBlinking}, event::{Event, KeyCode, read}, execute, queue, style::Print, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode}
};
use colored::Colorize;

use crate::task::*;
use crate::todolist::TodoList;


pub struct TodoTUI {
    todolist: TodoList,
    running: bool,
}

impl TodoTUI {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let todolist = TodoList::new()?;
        Ok(Self {
            todolist: todolist,
            running: true,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;
        execute!(std::io::stdout(), Hide, Clear(ClearType::All))?;
        
        while self.running {
            self.render()?;
            self.handle_events()?;
        }
        
        disable_raw_mode()?;
        execute!(std::io::stdout(), Show)?;
        Ok(())
    }

    // ===== Tasks List methods ===== // 

    fn render(&self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), 
            Clear(ClearType::All),
            MoveTo(0, 0),
            Hide
        )?;
        queue!(stdout(),
            Print("Ваши задачи:\r\n\r\n".cyan()),
        )?;
        
        // Список задач с выделением
        self.todolist.print_tasks()?;

        queue!(stdout(),
            Print("\r\n"),
            Print("Управление:\r\n"),
            Print("↑↓    Навигация\r\n"),
            Print("Enter Детали задачи\r\n"), 
            Print("a     Добавить задачу\r\n".green()),
            Print("d     Удалить задачу\r\n".red()),
            Print("q     Выход\r\n"),
        )?;
        stdout().flush()?;

        Ok(())
    }

    fn toggle_completed(&mut self) -> Result<(), Box<dyn Error>> {
        self.todolist.toggle_current_completed()?;
        self.todolist.refresh_data()?;
        Ok(())
    }

    // ===== Tasks List methods ===== // 

    fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Up => {
                        self.todolist.select_previous();
                    }
                    KeyCode::Down => {
                        self.todolist.select_next();
                    }
                    KeyCode::Enter => {
                        if let Some((_task, _depth)) = self.todolist.get_selected_task() {
                            self.show_task_details_menu()?;
                        }
                    }
                    KeyCode::Tab => {
                        self.toggle_completed()?;
                    }
                    KeyCode::Char('a') => {
                        self.add_task(None)?;
                    }
                    KeyCode::Char('d') => {
                        self.delete_selected_task()?;
                    }
                    KeyCode::Char('q') => {
                        self.running = false;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    // ===== Show Details methods ===== // 

    fn show_task_details_menu(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        disable_raw_mode()?;
        execute!(stdout(), 
            Clear(ClearType::All),
            MoveTo(0, 0),
            EnableBlinking,
            Show
        )?;

        loop {
            
            let (task, _) = self.todolist.get_selected_task().unwrap();
            self.show_task_details(&task)?;
            
            println!("{}", "\n\n1. Добавить подзадачу".green());
            println!("{}", "2. Изменить данные".yellow());
            println!("{}", "3. Удалить задачу".red());
            println!("4. Назад");
            print!("Ваш выбор: ");
            stdout().flush()?;
            
            let input: u32 = console_read()?.parse().unwrap_or(0);
            match input {
                1 => {
                    self.add_subtask(task.id)?;
                },
                2 => {
                    self.change_task_data(&task)?;
                    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
                    println!("{}", "Данные обновлены!\n".green());
                },
                3 => {
                    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
                    if self._delete_selected_task()? {
                        break
                    }
                    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
                },
                _ => break
            }
        }
        enable_raw_mode()?;
        Ok(())
    }

    fn add_subtask(&mut self, task_id: u32) -> Result<(), Box<dyn Error>> {
        execute!(stdout(), Clear(ClearType::All),MoveTo(0, 0),EnableBlinking,Show,)?;
        println!("{}", "\n➕ Добавление подзадачи...".green());
        let result = self._add_task(Some(task_id))?;
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        if result {
            println!("{}", "Подзадача добавлена!\n".green());
        }

        Ok(())
    }

    fn change_task_data(&mut self, task: &Task) -> Result<(), Box<dyn Error>> {
        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
        println!("Введите новые данные для задачи:");
        let (name, desc) = self.enter_task_data()?;
        let updated = self.todolist.update_task(
            task.id, 
            name, 
            None,
            None, 
            desc
        )?;

        if updated {
            self.todolist.refresh_data()?;
        }

        Ok(())

    }

    fn show_task_details(&self, task: &Task) -> Result<(), Box<dyn Error>> {
        println!("📋 Детали задачи");
        println!("{}", "─".repeat(30));
        println!("📝 Название: {}", task.name);
        println!("✅ Статус: {}", if task.completed { "Выполнена" } else { "В работе" });
        println!("📅 Создана: {}", task.creation_date.format("%Y-%m-%d %H:%M"));
        
        if let Some(desc) = &task.description {
            println!("📄 Описание: {}", desc);
        }
        
        // Показываем дочерние задачи
        let children = self.todolist.get_selected_children()?;
        if !children.is_empty() {
            println!("\n🔗 Подзадачи:\r\n");
            for (i, child) in children.iter().enumerate() {
                println!("  {}. {} {}", i + 1, 
                                        if child.completed { "✓" } else { "○" }, 
                                        child.name);
            }
        }
        Ok(())
    }

    // ===== Show Details methods ===== //

    // ===== Adding task methods ===== // 

    fn enter_task_data(&self) -> Result<(Option<String>, Option<String>), Box<dyn Error>> {
        print!("Введите название задачи [Enter для отмены]: ");
        stdout().flush()?;

        let name = console_read()?;
        if name.is_empty() {
            return Ok((None, None))
        }

        println!("Введите описание задачи [Enter чтобы пропустить]: ");
        let desc = console_read()?;
        Ok((Some(name), if desc.is_empty() { None } else { Some(desc) }))
    }

    fn _add_task(&mut self, parent_id: Option<u32>) -> Result<bool, Box<dyn std::error::Error>> {
        let (name, desc) = self.enter_task_data()?;
        if name.is_none() {
            return Ok(false)
        }
        self.todolist.create_task(
            name.unwrap(), 
            parent_id, 
            desc
        )?;

        self.todolist.refresh_data()?;
        Ok(true)
    }

    fn add_task(&mut self, parent_id: Option<u32>) -> Result<(), Box<dyn Error>> {
        disable_raw_mode()?;
        execute!(stdout(), 
            Clear(ClearType::All),
            MoveTo(0, 0),
            EnableBlinking,
            Show,
        )?;
        println!("{}", "\n➕ Добавление задачи...".green());
        self._add_task(parent_id)?;
        enable_raw_mode()?;
        Ok(())
    }

    // ===== Adding task methods ===== // 

    // ===== Deleting task methods ===== // 
    
    fn delete_selected_task(&mut self) -> Result<bool, Box<dyn std::error::Error>> { 
        execute!(stdout(), 
            Clear(ClearType::All),
            MoveTo(0, 0),
            EnableBlinking,
            Show,
        )?;
        disable_raw_mode()?;
        let result = self._delete_selected_task()?;
        enable_raw_mode()?;

        Ok(result)
    }

    fn _delete_selected_task(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        if let Some((task, _)) = self.todolist.get_selected_task() {
            loop {
                println!("\n🗑️ Удаление задачи: {}", task.name.red());
                println!("Вы уверены, что хотите удалить выбранную задачу (так же удалятся все ее подзадачи)?");
                println!("{}", "1. Да".red());
                println!("2. Нет");
                print!("Ваш выбор: ");
                stdout().flush()?;
    
                let input: i32 = console_read()?.parse().unwrap_or(0) ;
    
                match input {
                    1 => {
                        self.todolist.delete_task(task.id)?;
                        self.todolist.refresh_data()?;
                        return Ok(true)
                    },
                    2 => return Ok(false),
                    _ => {
                        println!("Выберите одно из предложенного\n");
                        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
                    }
    
                }
            }
        }
        Ok(false)
    }

    // ===== Deleting task methods ===== // 

}

fn console_read() -> Result<String, Box<dyn Error>> {
    Ok(stdin().lock().lines().next().unwrap()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn change_task_data() -> Result<(), Box<dyn Error>> {
        let mut tui = TodoTUI::new()?;
        tui.todolist.select_previous();
        tui.todolist.select_previous();
        
        let (task, _) = tui.todolist.get_selected_task().unwrap();
        tui.change_task_data(&task)?;

        Ok(())
    }
}