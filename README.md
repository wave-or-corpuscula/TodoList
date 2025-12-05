# 📋 TodoList - Terminal Task Management Application

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![SQLite](https://img.shields.io/badge/sqlite-%2307405e.svg?style=for-the-badge&logo=sqlite&logoColor=white)
![Terminal](https://img.shields.io/badge/terminal-%23121212.svg?style=for-the-badge&logo=linux&logoColor=white)

An elegant terminal application for managing hierarchical tasks with unlimited subtask nesting. Written in Rust with SQLite for data persistence.

## ✨ Features

- 🌳 **Hierarchical Tasks** - Create subtasks with unlimited nesting depth
- ⌨️ **Navigation Interface** - Navigate through tasks with arrow keys ↑↓
- ⚡ **Quick Actions** - Toggle completion, add, edit, delete tasks instantly
- 💾 **Reliable Storage** - SQLite database with cascade deletion for subtasks
- 🎨 **Colored Interface** - Visual status indication and task highlighting
- 🔄 **Live Updates** - Instant task completion toggling (Tab)

## 🚀 Quick Start

### Install Dependencies

Make sure you have [Rust](https://rustup.rs/) and [SQLite](https://sqlite.org/download.html) installed.

### Clone and Build

```bash
git clone <repository-url>
cd todolist
cargo build --release
```

### Database Setup

1. Copy `.env.example` to `.env`:
```bash
cp .env.example .env
```

2. Fill environment variables in `.env`:
```env
DB_PATH=./data/todolist.sqlite3
```

3. Run migrations to create tables and populate with test data:
```bash
cargo run --bin migration
```

### Run the Application

```bash
cargo run --bin todolist
```

## 🎮 Controls

### Main Screen
- **↑↓** - Navigate through task list
- **Enter** - Open selected task details
- **Tab** - Toggle task completion status
- **a** - Add new task
- **d** - Delete selected task
- **q** - Quit

### Task Details
- **1** - Add subtask
- **2** - Edit task data
- **3** - Delete task (cascades to all subtasks)
- **4** - Return to main list

## 📁 Project Structure

```
src/
├── lib.rs                 # Module re-exports
├── main.rs               # Application entry point
├── task.rs               # Data models (Task, TaskWithKids)
├── todotui.rs            # Terminal user interface controller
├── todolist.rs           # Legacy task management logic
├── database.rs           # SQLite database operations
├── config.rs             # Application configuration
├── ui/                   # User interface components
│   ├── mod.rs
│   ├── input.rs          # Input handling utilities
│   ├── task_renderer.rs  # Task display formatting
│   └── terminal.rs       # Terminal control utilities
└── services/             # Business logic services
    ├── mod.rs
    ├── task_service.rs   # Task management operations
    └── navigation_service.rs # Navigation and traversal logic

```

## 🛠️ Technology Stack

- **Language:** Rust 2024 Edition
- **Database:** SQLite with `rusqlite`
- **UI Framework:** `crossterm` for terminal interface
- **Configuration:** `dotenv` for environment variables
- **Colors:** `colored` for visual formatting
- **Date/Time:** `chrono` for timestamp management

## 📊 Architecture

The application follows the separation of concerns principle:

1. **Data Layer** (`database.rs`) - SQLite abstraction
2. **Business Logic** (`services/`) - Task management and navigation
3. **Presentation Layer** (`todotui.rs`, `ui/`) - User interface
4. **Models** (`task.rs`) - Data structures

### Service Layer Architecture

- **TaskService**: Handles all task CRUD operations and database interactions
- **NavigationService**: Manages task traversal and selection logic
- **UI Components**: Separated concerns for input handling, rendering, and terminal control

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test print_tasks
```

## 📋 Usage Example

```
Your tasks:

○ Learn Rust
  ○ Read The Rust Book
  ✓ Do exercises
  ○ Write simple program
✓ Create TodoList project
  ✓ Set up project
    ✓ Install dependencies
  ○ Implement CRUD operations
  ✓ Add migrations
○ Write documentation

Controls:
↑↓    Navigate
Enter Task details
a     Add task
d     Delete task
q     Quit
```

## 🔧 Development

### Adding New Commands

To add a new command to the interface:

1. Add handler in `handle_events()` in `todotui.rs`
2. Implement the corresponding method
3. Update command documentation

### Extending Data Models

To add new fields to the `Task` model:

1. Update the structure in `task.rs`
2. Add migration in `migration.rs`
3. Update SQL queries in `database.rs`
4. Update UI to display new fields

## 🐛 Troubleshooting

### Common Issues

**Database Error:**
```bash
# Make sure SQLite is installed
sqlite3 --version

# Check directory permissions
mkdir -p data
chmod 755 data
```

**Terminal Display Issues:**
```bash
# Check UTF-8 support
export LANG=en_US.UTF-8
export LC_ALL=en_US.UTF-8
```

## 🤝 Contributing

1. Fork the project
2. Create a branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Commit (`git commit -m 'Add amazing feature'`)
5. Push to repository (`git push origin feature/amazing-feature`)
6. Open a Pull Request

## 📄 License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- The Rust team for the wonderful programming language
- `crossterm` creators for excellent terminal utilities
- SQLite community for reliable embedded database

---

**Made with ❤️ in Rust**