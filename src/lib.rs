pub mod control;
pub mod db;
pub mod model;
mod util;

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

#[derive(Debug, clap::Parser)]
pub struct ListOptions {
    /// 完了 / 未完了どちらのタスクを表示するか
    /// 0 = 未完了、1 = 完了
    #[arg(short = 's', long, default_value = "0")]
    pub status: String,
}

#[derive(Args)]
pub struct UuidOptions {
    /// ID of the todo to mark as done.
    pub uuid: String,
}

#[derive(Debug, clap::Parser)]
pub struct EditOptions {
    /// ID of the todo to mark as done.
    pub uuid: String,

    /// Title
    #[arg(short = 't', long)]
    pub title: Option<String>,

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
    #[arg(long)]
    pub time: Option<String>,

    /// Description of the todo (optional).
    #[arg(short, long)]
    pub description: Option<String>,

    /// URL of the todo (optional).
    #[arg(short, long)]
    pub url: Option<String>,
}
