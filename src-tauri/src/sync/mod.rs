pub mod android_multicast;
pub mod commands;
pub mod crdt;
pub mod discovery;
pub mod identity;
pub mod log;
pub mod transport;
pub mod types;

use crate::db::Database;

/// Возвращает текущую дату в формате SQLite (`YYYY-MM-DD HH:MM:SS`).
/// Использует тот же формат, что и `datetime('now')`, чтобы LWW-сравнения были корректны.
pub fn now_sqlite_datetime(db: &Database) -> Option<String> {
    let conn = db.conn.lock().ok()?;
    conn.query_row("SELECT datetime('now')", [], |r| r.get::<_, String>(0))
        .ok()
}