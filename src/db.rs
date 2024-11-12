use chrono::prelude::*;
use rusqlite::{OpenFlags, Result};
use std::path::Path;

pub struct Todo {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub done: i64,
    pub title: String,
    pub start_date: Option<DateTime<Utc>>,
    pub start_time: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub url: Option<String>,
}

impl std::fmt::Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "id: {}, created_at: {}, updated_at: {}, title: {}, done: {}, description: {:?}, url: {:?}, due_date: {:?}",
            self.id, self.created_at.to_rfc3339(), self.updated_at.to_rfc3339(), self.title, self.done, self.description, self.url, self.start_date
        )
    }
}

pub fn open_db(p: &Path) -> Result<rusqlite::Connection> {
    // p のパスにデータベースファイルが存在しない場合は新規作成する
    if !p.exists() {
        // db ファイルが存在しない場合は初回起動とみなし、テーブルを作成する
        let db = rusqlite::Connection::open_with_flags(
            p,
            OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
        )?;
        db.execute(
            "CREATE TABLE todos (
                id INTEGER PRIMARY KEY,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                done INTEGER NOT NULL,
                title TEXT NOT NULL,
                start_date TEXT,
                start_time TEXT,
                description TEXT,
                url TEXT
            )",
            (),
        )?;
        return Ok(db);
    }
    rusqlite::Connection::open_with_flags(p, OpenFlags::SQLITE_OPEN_READ_WRITE)
}

pub fn add_todo(conn: &rusqlite::Connection, todo: &Todo) -> Result<()> {
    conn.execute(
        "INSERT INTO todos (created_at, updated_at, done, title, start_date, start_time, description, url)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (
            todo.created_at.to_rfc3339(),
            todo.updated_at.to_rfc3339(),
            todo.done,
            &todo.title,
            todo.start_date.map(|d| d.to_rfc3339()).as_deref(),
            todo.start_time.map(|d| d.to_rfc3339()).as_deref(),
            todo.description.as_deref(),
            todo.url.as_deref(),
        ),
    )?;
    Ok(())
}

pub fn list(conn: &rusqlite::Connection) -> Result<Vec<Todo>> {
    let mut stmt = conn.prepare(
        "SELECT id, created_at, updated_at, done, title, due_date, description, url
        FROM todos ORDER BY created_at DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        // Index 1, 2 は String として row.get し、Local.datetime_from_str で DateTime に変換する
        // Index 7 は Option<String> として row.get し、Option<DateTime> に変換する
        let created_at_str: String = row.get(1)?;
        let updated_at_str: String = row.get(2)?;
        let due_date_str: Option<String> = row.get(5)?;
        let due_date_time_str: Option<String> = row.get(6)?;

        Ok(Todo {
            id: row.get(0)?,
            created_at: DateTime::parse_from_rfc3339(&created_at_str)
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                .unwrap()
                .with_timezone(&Utc),
            done: row.get(3)?,
            title: row.get(4)?,
            start_date: due_date_str.map(|d| {
                DateTime::parse_from_rfc3339(&d)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
            start_time: due_date_time_str.map(|d| {
                DateTime::parse_from_rfc3339(&d)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
            description: row.get(7)?,
            url: row.get(8)?,
        })
    })?;
    let mut todos = Vec::new();
    for todo in rows {
        todos.push(todo?);
    }
    Ok(todos)
}
