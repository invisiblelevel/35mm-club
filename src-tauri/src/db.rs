use rusqlite::{Connection, Result, params};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Включаем WAL для производительности и защиты от блокировок
        conn.execute_batch("
            PRAGMA journal_mode = WAL;
            PRAGMA foreign_keys = ON;
        ")?;
        
        Ok(Self { conn: Mutex::new(conn) })
    }
}