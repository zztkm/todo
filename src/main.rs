mod app_dir;
mod commands;
mod db;

use clap::Parser;
use commands::{Cli, Commands, ListArgs};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::List(args) => {
            list(args);
        }
    }
}

fn list(args: &ListArgs) {
    let db_path = app_dir::get_app_db_path();
    let db = db::open_db(&db_path).unwrap();
    let _ = db.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        (),
    );
    println!("db path: {:?}", db_path);
    println!("List command");
    println!("Filter: {:?}", args.filter);
    println!("Sort: {:?}", args.sort);
    println!("Reverse: {:?}", args.reverse);
    println!("Limit: {:?}", args.limit);
    println!("Output: {:?}", args.output);
}
