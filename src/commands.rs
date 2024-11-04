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
    /// List todos.
    ///
    /// This command lists all todos.
    /// --filter: Filter todos by status.
    /// --sort: Sort todos by status.
    /// --reverse: Reverse the order of the todos.
    /// --limit: Limit the number of todos.
    /// --output: Output format.
    List(ListArgs),
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
