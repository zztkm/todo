use clap::Parser;
use todo::commands::{AddTodoArgs, Cli, Commands, ListArgs};
use todo::db;
use todo::util;

use chrono::Utc;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add(args) => {
            add(args);
        }
        Commands::List(args) => {
            list(args);
        }
    }
}

fn add(args: &AddTodoArgs) {
    let db_path = util::get_app_db_path();
    let conn = db::open_db(&db_path).unwrap();

    let current_time = Utc::now();

    // 開始日時のパース
    // start_date なしで、start_time を指定した場合は current_time の日付を使う
    // start_date & start_time ありの場合は、start_date と start_time を結合して DateTime にする
    let start_date = if args.date.is_some() && args.time.is_some() {
        let date = args.date.as_ref().unwrap();
        let time = args.time.as_ref().unwrap();
        util::parse_ymd_hms(date, time)
    } else if args.date.is_some() && args.time.is_none() {
        let date = args.date.as_ref().unwrap();
        let time = "00:00:00"; // 時刻が指定されていない場合は 00:00:00 固定
        util::parse_ymd_hms(date, time)
    } else if args.date.is_none() && args.time.is_some() {
        let date = &current_time.format("%Y-%m-%d").to_string();
        let time = args.time.as_ref().unwrap();
        util::parse_ymd_hms(date, time)
    } else {
        // 両方なしの場合は Todo 初期化時に無視されるが、都合上 current_time を返す
        current_time
    };

    // start_date と start_time は CLI 引数で指定された場合のみ設定する
    // None はつまり指定されていないことを意味する
    // これは 例えば、start_date が 2024-12-31 で start_time が指定されていない場合、
    // 保持する DateTime は 2024-12-31 00:00:00 となるが、クライアントには開始日を 2024-12-31、開始時刻を設定なしとして表示するためである
    let todo = db::Todo {
        id: 0,
        created_at: current_time,
        updated_at: current_time,
        done: 0,
        title: args.title.clone(),
        start_date: args.date.as_ref().map(|_| start_date),
        start_time: args.time.as_ref().map(|_| start_date),
        description: args.description.clone(),
        url: args.url.clone(),
    };

    db::add_todo(&conn, &todo).unwrap();
}

fn list(args: &ListArgs) {
    let db_path = util::get_app_db_path();
    let conn = db::open_db(&db_path).unwrap();

    let todos = db::list(&conn).unwrap();

    if todos.is_empty() {
        println!("No todos found");
        return;
    }
    for todo in todos {
        println!("{}", todo);
    }
}
