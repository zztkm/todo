use rusqlite::{OpenFlags, Result};
use std::path::Path;

pub fn open_db(p: &Path) -> Result<rusqlite::Connection> {
    rusqlite::Connection::open_with_flags(
        p,
        OpenFlags::SQLITE_OPEN_CREATE | OpenFlags::SQLITE_OPEN_READ_WRITE,
    )
}
