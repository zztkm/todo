use clap::{Parser, Subcommand};
use oktodo::db;
use oktodo::todo;
use oktodo::util;

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
    pub fn execute(&self, contoroller: todo::TodoController) {
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
    Add(oktodo::AddOptions),

    /// List todos.
    ///
    /// This command lists all todos.
    List(oktodo::ListOptions),
}

fn main() {
    // TODO(zztkm): 引数が指定されていない場合は TUI モードで起動する
    let conn = db::open_db(&util::get_app_db_path()).unwrap();
    let controller = todo::TodoController::new(conn);
    let cli = Cli::parse();
    cli.execute(controller);
}
