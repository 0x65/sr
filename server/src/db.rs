use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

const DB: &str = "main.db";

pub fn connect() -> Result<Connection> {
    Connection::open(DB)
}

pub fn setup_db() -> Result<()> {
    let conn = connect()?;

    // TODO: move to separate schema files
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email VARCHAR(64) NOT NULL
        )",
        NO_PARAMS,
    )?;

    Ok(())
}

// TODO: reuse connection
pub mod user {
    use crate::db;
    use rusqlite::Result;

    pub struct User {
        pub id: i64,
        pub email: String,
    }

    pub fn insert(email: &str) -> Result<i64> {
        let conn = db::connect()?;
        conn.execute("INSERT INTO users (email) VALUES (?1)", &[email])?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_by_email(email: &str) -> Result<User> {
        let conn = db::connect()?;
        let mut stmt = conn.prepare("SELECT id, email FROM users WHERE email = ?1")?;
        stmt.query_row(&[email], |row| {
            Ok(User {
                id: row.get(0)?,
                email: row.get(1)?,
            })
        })
    }
}
