use std::{error::Error, io::{BufRead, Write, stdin, stdout}};

use colored::Colorize;
use crossterm::{
    cursor::{Hide, MoveTo, Show, EnableBlinking}, event::{Event, KeyCode, read}, execute, queue, style::Print, terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode}
};

use crate::task::*;
use crate::todolist::TodoList;

pub enum AppState {
    TaskMenu,
    TaskDetails,
    AddTask,
}

pub struct TodoTUI {
    todolist: TodoList,
    running: bool,
    state: AppState,
}

impl TodoTUI {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let todolist = TodoList::new()?;
        Ok(Self {
            todolist: todolist,
            running: true,
            state: AppState::TaskMenu,
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
        
        // Заголовок
        queue!(stdout(), 
            Print("\r\n"),
            Print("🎯 TodoList - Управление задачами\r\n"),
            Print("═".repeat(50)),
            Print("\r\n")
        )?;
        
        // Список задач с выделением
        self.todolist.print_tasks()?;

        queue!(stdout(),
            Print("\r\n"),
            Print("Управление:\r\n"),
            Print("↑↓    Навигация\r\n"),
            Print("Enter Детали задачи\r\n"), 
            Print("q     Выход\r\n")
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
                        if let Some((task, _depth)) = self.todolist.get_selected_task() {
                            self.show_task_details(&task)?;
                        }
                    }
                    KeyCode::Tab => {
                        self.toggle_completed();
                    }
                    KeyCode::Char('a') => {
                        self.add_task()?;
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

    fn show_task_details(&self, task: &Task) -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), 
            Clear(ClearType::All),
            MoveTo(0, 0),
            Show
        )?;
        disable_raw_mode()?;
        queue!(stdout(),
            Print("📋 Детали задачи\r\n"),
            Print(format!("{}\n", "─".repeat(30))),
            Print(format!("📝 Название: {}\r\n", task.name)),
            Print(format!("✅ Статус: {}\r\n", if task.completed { "Выполнена" } else { "В работе" })),
            Print(format!("📅 Создана: {}\r\n", task.creation_date.format("%Y-%m-%d %H:%M"))),
        )?;
        
        if let Some(desc) = &task.description {
            println!("📄 Описание: {}", desc);
        }
        
        // Показываем дочерние задачи
        let children = self.todolist.get_selected_children()?;
        if !children.is_empty() {
            queue!(stdout(),Print("\n🔗 Подзадачи:\r\n"))?;
            for (i, child) in children.iter().enumerate() {
                queue!(stdout(),
                Print(format!("  {}. {} {}", i + 1, 
                    if child.completed { "✓" } else { "○" }, 
                    child.name)))?;
            }
        }
        let mut input = String::new();

        println!("\n\n1. Добавить подзадачу");
        println!("2. Изменить название");
        println!("3. Изменить описание");
        println!("4. Удалить задачу");
        println!("5. Назад");
        println!("Ваш выбор: ");

        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");
        println!("Your choice: {}", input);
        
        Ok(())
    }

    // ===== Show Details methods ===== //

    // ===== Adding task methods ===== // 

    fn add_task(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), 
            Clear(ClearType::All),
            MoveTo(0, 0),
            EnableBlinking,
            Show,
        )?;
        disable_raw_mode()?;
        // Здесь будет логика добавления задачи
        println!("\n➕ Добавление задачи...");
        print!("Введите название задачи [Enter для отмены]: ");
        stdout().flush()?;

        let name = console_read()?;
        if name.is_empty() {
            enable_raw_mode()?;
            return Ok(())
        }

        println!("Введите описание задачи [Enter чтобы пропустить]: ");
        let desc = console_read()?;

        self.todolist.create_task(
            name, 
            None, 
            if desc.is_empty() { None } else { Some(desc) }
        )?;

        self.todolist.refresh_data()?;
        enable_raw_mode()?;
        Ok(())
    }

    // ===== Adding task methods ===== // 

    // ===== Deleting task methods ===== // 
    
    fn delete_selected_task(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        execute!(stdout(), 
            Clear(ClearType::All),
            MoveTo(0, 0),
            EnableBlinking,
            Show,
        )?;
        disable_raw_mode()?;
        if let Some((task, _)) = self.todolist.get_selected_task() {
            loop {
                println!("\n🗑️ Удаление задачи: {}", task.name);
                println!("Вы уверены, что хотите удалить выбранную задачу (так же удалятся все ее подзадачи)?");
                println!("1. Да");
                println!("2. Нет");
                print!("Ваш выбор: ");
                stdout().flush()?;
    
                let input: i32 = console_read()?.parse().unwrap_or(0) ;
    
                match input {
                    1 => {
                        self.todolist.delete_task(task.id)?;
                        self.todolist.refresh_data()?;
                        break;
                    },
                    2 => break,
                    _ => {
                        println!("Выберите одно из предложенного\n");
                        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;
                    }
    
                }
            }
            enable_raw_mode()?;
        }
        Ok(())
    }

    // ===== Deleting task methods ===== // 

}

fn console_read() -> Result<String, Box<dyn Error>> {
    Ok(stdin().lock().lines().next().unwrap()?)
}
