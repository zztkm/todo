use clap::{Parser, Subcommand};
use rusqlite::OpenFlags;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use todo::control::TodoController;
use todo::db;
/// A basic example
///
/// This command is a basic example of a command that does nothing.
#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub fn execute(&self, contoroller: TodoController) {
        match &self.command {
            Commands::Add(args) => match contoroller.add_todo(args) {
                Ok(_) => println!("Todo created successfully."),
                Err(e) => eprintln!("Failed to create a todo: {}", e),
            },
            Commands::List(_) => {
                let todos = contoroller.list_todos().unwrap();
                if todos.is_empty() {
                    println!("No todos found.");
                    return;
                }
                for todo in todos {
                    let done = if todo.done == 0 { " " } else { "x" };
                    println!(
                        "[{}] {}, start: {}, description: {}, url: {}",
                        done,
                        todo.title,
                        todo.start_date
                            .map(|d| d.format("%Y-%m-%d").to_string())
                            .unwrap_or("".to_string()),
                        todo.description.unwrap_or("".to_string()),
                        todo.url.unwrap_or("".to_string())
                    );
                }
            }
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new todo.
    ///
    /// This command creates a new todo.
    ///
    /// Example:
    /// $ todo create "Buy milk" --description "Buy 2 milk" --due-date "2024-12-31"
    Add(todo::AddOptions),

    /// List todos.
    ///
    /// This command lists all todos.
    List(todo::ListOptions),
}

fn main() {
    // TODO(zztkm): 引数が指定されていない場合は TUI モードで起動する
    let app_dir = init_app_dir().unwrap();
    let conn = open_db(&app_dir.join(".todo")).unwrap();
    let controller = TodoController::new(conn);
    let cli = Cli::parse();
    cli.execute(controller);
}

pub fn open_db(p: &Path) -> rusqlite::Result<rusqlite::Connection> {
    // p のパスにデータベースファイルが存在しない場合は新規作成する
    if !p.exists() {
        // db ファイルが存在しない場合は初回起動とみなし、テーブルを作成する
        let conn = rusqlite::Connection::open_with_flags(
            p,
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )?;
        db::init(&conn)?;
        return Ok(conn);
    }
    rusqlite::Connection::open_with_flags(p, OpenFlags::SQLITE_OPEN_READ_WRITE)
}

/// Get the path to the todo directory.
/// home ディレクトリを取得できなかった場合はエラーを返す
fn get_app_dir_path() -> Result<PathBuf, String> {
    match home::home_dir() {
        Some(path) => Ok(path.join(".todo")),
        _ => Err("Could not find home directory.".to_string()),
    }
}

fn init_app_dir() -> Result<PathBuf, String> {
    let path = get_app_dir_path()?;
    if !path.exists() {
        match fs::create_dir(&path) {
            Ok(_) => (),
            Err(e) => {
                return Err(format!("Could not create directory: {:?}", e));
            }
        }
    }
    Ok(path)
}
