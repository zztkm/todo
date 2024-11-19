use rusqlite::Result;

pub fn init(conn: &rusqlite::Connection) -> Result<()> {
    conn.execute(
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
    Ok(())
}
