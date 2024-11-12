use clap::{Args, Parser, Subcommand};

/// A basic example
///
/// This command is a basic example of a command that does nothing.
#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new todo.
    ///
    /// This command creates a new todo.
    ///
    /// Example:
    /// $ todo create "Buy milk" --description "Buy 2 milk" --due-date "2024-12-31"
    Add(AddTodoArgs),

    /// List todos.
    ///
    /// This command lists all todos.
    List(ListArgs),
}

#[derive(Args)]
pub struct AddTodoArgs {
    /// Title of the todo (required).
    pub title: String,

    /// Start date of the todo (optional).
    ///
    /// Example: "2024-12-31"
    #[arg(short, long)]
    pub date: Option<String>,

    /// Start time of the todo (optional).
    ///
    /// Example: "14:30:00"
    // TODO(zztkm): time は hh:mm もサポートする
    #[arg(short, long)]
    pub time: Option<String>,

    /// Description of the todo (optional).
    #[arg(long)]
    // date と頭文字が被っているため、short オプションを指定しない
    pub description: Option<String>,

    /// URL of the todo (optional).
    #[arg(short, long)]
    pub url: Option<String>,
}

#[derive(Args)]
pub struct ListArgs {
    /// Filter todos by status.
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Sort todos by status.
    #[arg(short, long)]
    pub sort: Option<String>,

    /// Reverse the order of the todos.
    #[arg(short, long)]
    pub reverse: bool,

    /// Limit the number of todos.
    #[arg(short, long)]
    pub limit: Option<usize>,

    /// Output format.
    #[arg(short, long)]
    pub output: Option<String>,
}
