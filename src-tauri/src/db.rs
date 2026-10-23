use rusqlite::{Connection, Result};
use std::sync::Mutex;

pub struct Database {
    pub conn: Mutex<Connection>,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;

        // WAL + foreign keys
        conn.execute_batch(
            "
            PRAGMA journal_mode = WAL;
            PRAGMA foreign_keys = ON;
            ",
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
}

/// Применяет миграции.
///
/// 001 — всегда, при каждом старте. Она идемпотентна
/// (CREATE IF NOT EXISTS + INSERT ... ON CONFLICT DO UPDATE),
/// поэтому новые ачивки, дописанные в 001_initial.sql, подтягиваются
/// на всех устройствах при следующем запуске. Данные пользователя не трогаются.
///
/// 002 — только один раз, для устройств, где ещё не применялась
/// (проверяем по PRAGMA user_version). После применения ставит user_version = 2.
pub fn apply_migrations(conn: &Connection) -> std::result::Result<(), String> {
    // 001 — ВСЕГДА
    conn.execute_batch(include_str!("../migrations/001_initial.sql"))
        .map_err(|e| format!("001: {}", e))?;

    let version: i64 = conn
        .query_row("PRAGMA user_version", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;

    if version < 2 {
        conn.execute_batch(include_str!("../migrations/002_sync.sql"))
            .map_err(|e| format!("002: {}", e))?;
    }

    Ok(())
}