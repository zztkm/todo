use rusqlite::{OpenFlags, Result};
use std::path::Path;

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
