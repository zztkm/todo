use crate::model::Todo;
use crate::{util, EditOptions};
use crate::{AddOptions, ListOptions, UuidOptions};
use chrono::{DateTime, Utc};
use rusqlite::Result;
use uuid::Uuid;

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
            uuid: Uuid::new_v4(),
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
            "INSERT INTO todos (uuid, created_at, updated_at, done, title, start_date, start_time, description, url)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                todo.uuid.to_string(), // ハイフンありの文字列に変換
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

    pub fn done(&self, options: &UuidOptions) -> Result<()> {
        // uuid 文字列は - ありを期待しているが 仮に - なしの文字列が来ても問題ないように Uuid に変換している
        let uuid = Uuid::parse_str(&options.uuid).unwrap();
        self.conn.execute(
            "UPDATE todos SET done = 1 WHERE uuid = ?1",
            [uuid.to_string()],
        )?;
        Ok(())
    }

    pub fn undone(&self, options: &UuidOptions) -> Result<()> {
        let uuid = Uuid::parse_str(&options.uuid).unwrap();
        self.conn.execute(
            "UPDATE todos SET done = 0 WHERE uuid = ?1",
            [uuid.to_string()],
        )?;
        Ok(())
    }

    pub fn list_todos(&self, options: &ListOptions) -> Result<Vec<Todo>> {
        let query = format!(
            "
            SELECT
                id
                ,created_at
                , updated_at
                , done
                , title
                , start_date
                , start_time
                , description
                , url
                , uuid
            FROM todos
            WHERE done = {}
            ORDER BY created_at asc
        ",
            options.status
        );
        let mut stmt = self.conn.prepare(&query)?;
        let rows = stmt.query_map([], |row| {
            // Index 1, 2 は String として row.get し、Local.datetime_from_str で DateTime に変換する
            // Index 5,6 は それぞれ開始日時と時刻を表し Option<String> として row.get し、Option<DateTime> に変換する
            let created_at_str: String = row.get(1)?;
            let updated_at_str: String = row.get(2)?;
            let start_date_str: Option<String> = row.get(5)?;
            let start_time_str: Option<String> = row.get(6)?;
            let uuid_str: String = row.get(9)?;

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
                uuid: Uuid::parse_str(&uuid_str).unwrap(),
            })
        })?;
        let mut todos = Vec::new();
        for todo in rows {
            todos.push(todo?);
        }
        Ok(todos)
    }

    pub fn edit(&self, options: &EditOptions) -> Result<()> {
        let current_time = Utc::now();
        let prev_todo = self.get(options.uuid.clone())?;

        let start_date_time =
            util::parse_start_date_time(options.date.clone(), options.time.clone());
        let todo = Todo {
            id: prev_todo.id,
            uuid: prev_todo.uuid,
            created_at: prev_todo.created_at,
            updated_at: current_time,
            done: prev_todo.done,
            title: options.title.clone().unwrap_or(prev_todo.title),
            start_date: options
                .date
                .as_ref()
                .map(|_| start_date_time)
                .or(prev_todo.start_date),
            start_time: options
                .time
                .as_ref()
                .map(|_| start_date_time)
                .or(prev_todo.start_time),
            description: options.description.clone().or(prev_todo.description),
            url: options.url.clone().or(prev_todo.url),
        };
        let query = "
            UPDATE todos
            SET
                updated_at = ?1
                , title = ?2
                , start_date = ?3
                , start_time = ?4
                , description = ?5
                , url = ?6
            WHERE uuid = ?7
        ";
        let mut stmt = self.conn.prepare(query)?;
        stmt.execute((
            todo.updated_at.to_rfc3339(),
            todo.title,
            todo.start_date.map(|d| d.to_rfc3339()).as_deref(),
            todo.start_time.map(|d| d.to_rfc3339()).as_deref(),
            todo.description,
            todo.url,
            prev_todo.uuid.to_string(),
        ))?;
        Ok(())
    }

    fn get(&self, uuid: String) -> Result<Todo> {
        let query = "
            SELECT
                id
                ,created_at
                , updated_at
                , done
                , title
                , start_date
                , start_time
                , description
                , url
                , uuid
            FROM todos
            WHERE uuid = ?1
        ";
        let mut stmt = self.conn.prepare(query)?;
        let todo = stmt.query_row([uuid], |row| {
            // Index 1, 2 は String として row.get し、Local.datetime_from_str で DateTime に変換する
            // Index 5,6 は それぞれ開始日時と時刻を表し Option<String> として row.get し、Option<DateTime> に変換する
            let created_at_str: String = row.get(1)?;
            let updated_at_str: String = row.get(2)?;
            let start_date_str: Option<String> = row.get(5)?;
            let start_time_str: Option<String> = row.get(6)?;
            let uuid_str: String = row.get(9)?;

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
                uuid: Uuid::parse_str(&uuid_str).unwrap(),
            })
        })?;
        Ok(todo)
    }
}
