pub mod db;
pub mod todo;
pub mod util;

use clap::Args;

#[derive(Args)]
pub struct AddOptions {
    /// Title of the todo (required).
    pub title: String,

    /// Start date of the todo (optional).
    ///
    /// Example: "2024-12-31"
    #[arg(long)]
    // description と頭文字が被っているため、short オプションを指定しない
    pub date: Option<String>,

    /// Start time of the todo (optional).
    ///
    /// Example: "14:30:00"
    // TODO(zztkm): time は hh:mm もサポートする
    #[arg(short, long)]
    pub time: Option<String>,

    /// Description of the todo (optional).
    #[arg(short, long)]
    pub description: Option<String>,

    /// URL of the todo (optional).
    #[arg(short, long)]
    pub url: Option<String>,
}

#[derive(Args)]
pub struct ListOptions {
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
