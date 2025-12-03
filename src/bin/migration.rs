use std::{env, error::Error};
use dotenv;
use rusqlite::{Connection, Result};



fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv().ok();

    let db_path: String = env::var("DB_PATH")?.parse()?;
    let mut conn = Connection::open(db_path)?;

    conn.execute("PRAGMA foreign_keys = ON", ())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Task (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                parent_id     INTEGER DEFAULT NULL,
                name          TEXT NOT NULL,
                completed     INTEGER DEFAULT 0,
                description   TEXT,
                creation_date TEXT DEFAULT CURRENT_TIMESTAMP,
                
                FOREIGN KEY (parent_id) REFERENCES Task(id) ON DELETE CASCADE
            );",
        ()
    )?;

    seed_database(&mut conn)?;

    Ok(())
}

fn seed_database(conn: &mut Connection) -> Result<()> {
    let tx = conn.transaction()?;

    // Уровень 1: Корневые задачи (3 задачи)
    tx.execute(
        "INSERT INTO Task (name, completed, description) VALUES 
         ('Изучить Rust', 0, 'Изучить основы языка программирования Rust'),
         ('Создать проект TodoList', 1, 'Разработать консольное приложение для управления задачами'),
         ('Написать документацию', 0, 'Подготовить документацию к проекту')",
        (),
    )?;

    // Получаем ID только что вставленных корневых задач
    let rust_id: u32 = tx.last_insert_rowid() as u32 - 2; // "Изучить Rust"
    let todo_id: u32 = tx.last_insert_rowid() as u32 - 1; // "Создать проект TodoList" 
    let _doc_id: u32 = tx.last_insert_rowid() as u32;       // "Написать документацию"

    // Уровень 2: Подзадачи для "Изучить Rust" (3 задачи)
    tx.execute(
        "INSERT INTO Task (parent_id, name, completed, description) VALUES 
         (?1, 'Прочитать The Rust Book', 0, 'Прочитать официальную книгу по Rust'),
         (?1, 'Сделать упражнения', 1, 'Выполнить практические задания из книги'),
         (?1, 'Написать простую программу', 0, 'Создать первое приложение на Rust')",
        (&rust_id,),
    )?;

    // Уровень 3: Подзадачи для "Создать проект TodoList" (3 задачи)
    tx.execute(
        "INSERT INTO Task (parent_id, name, completed, description) VALUES 
         (?1, 'Настроить проект', 1, 'Создать Cargo.toml и структуру проекта'),
         (?1, 'Реализовать CRUD операции', 0, 'Добавить создание, чтение, обновление, удаление задач'),
         (?1, 'Добавить миграции', 1, 'Создать и настроить миграции базы данных')",
        (&todo_id,),
    )?;

    // Получаем ID задачи "Настроить проект" для последнего уровня
    let setup_id: u32 = tx.last_insert_rowid() as u32 - 2;

    // Уровень 4: Подзадача для "Настроить проект" (1 задача)
    tx.execute(
        "INSERT INTO Task (parent_id, name, completed, description) VALUES 
         (?1, 'Установить зависимости', 1, 'Добавить rusqlite, chrono, dotenv в Cargo.toml')",
        (&setup_id,),
    )?;

    tx.commit()?;
    println!("База данных успешно заполнена 10 задачами!");
    
    Ok(())
}