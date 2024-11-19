use crate::model::Todo;
use crate::util;
use crate::AddOptions;
use chrono::{DateTime, Utc};
use rusqlite::Result;

pub struct TodoController {
    conn: rusqlite::Connection,
}

impl TodoController {
    pub fn new(conn: rusqlite::Connection) -> Self {
        Self { conn }
    }

    pub fn add_todo(&self, options: &AddOptions) -> Result<()> {
        let current_time = Utc::now();
        let start_date_time =
            util::parse_start_date_time(options.date.clone(), options.time.clone());
        let todo = Todo {
            id: 0, // 0 を入れているが、実際には autoincrement で自動採番される
            created_at: current_time,
            updated_at: current_time,
            done: 0,
            title: options.title.clone(),
            start_date: options.date.as_ref().map(|_| start_date_time),
            start_time: options.time.as_ref().map(|_| start_date_time),
            description: options.description.clone(),
            url: options.url.clone(),
        };
        self.conn.execute(
            "INSERT INTO todos (created_at, updated_at, done, title, start_date, start_time, description, url)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
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

    pub fn list_todos(&self) -> Result<Vec<Todo>> {
        // TODO(zztkm): 条件による絞り込みを実装する
        let mut stmt = self.conn.prepare(
            "SELECT id, created_at, updated_at, done, title, start_date, start_time, description, url
        FROM todos ORDER BY created_at DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            // Index 1, 2 は String として row.get し、Local.datetime_from_str で DateTime に変換する
            // Index 5,6 は それぞれ開始日時と時刻を表し Option<String> として row.get し、Option<DateTime> に変換する
            let created_at_str: String = row.get(1)?;
            let updated_at_str: String = row.get(2)?;
            let start_date_str: Option<String> = row.get(5)?;
            let start_time_str: Option<String> = row.get(6)?;

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
                start_date: start_date_str.map(|d| {
                    DateTime::parse_from_rfc3339(&d)
                        .unwrap()
                        .with_timezone(&Utc)
                }),
                start_time: start_time_str.map(|d| {
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
}
