use crate::db::Database;
use crate::models::{DiaryEntry, Collection, CollectionItem};
use tauri::State;

fn resolve_poster_url(path: Option<String>) -> Option<String> {
    path.map(|p| {
        if p.starts_with("http") {
            p
        } else {
            format!("https://image.tmdb.org/t/p/w200{}", p)
        }
    })
}

#[tauri::command]
pub fn get_diary(db: State<Database>) -> Result<Vec<DiaryEntry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT we.id, we.watched_at, COALESCE(we.rating, ut.rating) as rating, we.notes, we.is_rewatch,
                t.title, t.poster_path, t.type, t.tmdb_id
         FROM watch_entries we
         JOIN user_titles ut ON ut.id = we.user_title_id
         JOIN titles t ON t.id = ut.title_id
         ORDER BY we.watched_at DESC"
    ).map_err(|e| e.to_string())?;

    let entries = stmt.query_map([], |row| Ok(DiaryEntry {
        id: row.get(0)?,
        watched_at: row.get(1)?,
        rating: row.get(2)?,
        notes: row.get(3)?,
        is_rewatch: row.get::<_, i64>(4)? != 0,
        title: row.get(5)?,
        poster_url: resolve_poster_url(row.get::<_, Option<String>>(6)?),
        type_field: row.get(7)?,
        tmdb_id: row.get(8)?,
    })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    Ok(entries)
}

#[tauri::command]
pub fn get_collections(db: State<Database>) -> Result<Vec<Collection>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn.prepare(
        "SELECT c.id, c.name, c.description, c.created_at,
         (SELECT COUNT(*) FROM collection_items ci WHERE ci.collection_id = c.id) as item_count
         FROM collections c ORDER BY c.created_at DESC"
    ).map_err(|e| e.to_string())?;

    let base_collections: Vec<(i64, String, Option<String>, String, i64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    let mut result = Vec::new();

    for (id, name, description, created_at, item_count) in base_collections {
        let mut pstmt = conn.prepare(
            "SELECT t.poster_path FROM collection_items ci
             JOIN user_titles ut ON ut.id = ci.user_title_id
             JOIN titles t ON t.id = ut.title_id
             WHERE ci.collection_id = ?1 AND t.poster_path IS NOT NULL
             ORDER BY ci.added_at DESC LIMIT 4"
        ).map_err(|e| e.to_string())?;

        let preview: Vec<String> = pstmt
            .query_map([id], |row| row.get::<_, String>(0))
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .map(|p| resolve_poster_url(Some(p)).unwrap_or_default())
            .collect();

        result.push(Collection {
            id, name, description, created_at, item_count,
            preview_posters: preview,
        });
    }

    Ok(result)
}

#[tauri::command]
pub fn create_collection(db: State<Database>, name: String, description: Option<String>) -> Result<i64, String> {
    if name.trim().is_empty() { return Err("Название не может быть пустым".to_string()); }
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO collections (name, description) VALUES (?1, ?2)", rusqlite::params![name.trim(), description]).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn delete_collection(db: State<Database>, collection_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM collections WHERE id = ?1", [collection_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_collection_items(db: State<Database>, collection_id: i64) -> Result<Vec<CollectionItem>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT ut.id, t.title, t.poster_path, t.type, t.tmdb_id, t.year, ut.status
         FROM collection_items ci
         JOIN user_titles ut ON ut.id = ci.user_title_id
         JOIN titles t ON t.id = ut.title_id
         WHERE ci.collection_id = ?1 ORDER BY ci.added_at DESC"
    ).map_err(|e| e.to_string())?;

    let items = stmt.query_map([collection_id], |row| Ok(CollectionItem {
        user_title_id: row.get(0)?,
        title: row.get(1)?,
        poster_url: resolve_poster_url(row.get::<_, Option<String>>(2)?),
        type_field: row.get(3)?,
        tmdb_id: row.get(4)?,
        year: row.get(5)?,
        status: row.get(6)?,
    })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    Ok(items)
}

#[tauri::command]
pub fn add_to_collection(db: State<Database>, collection_id: i64, user_title_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("INSERT OR IGNORE INTO collection_items (collection_id, user_title_id) VALUES (?1, ?2)", rusqlite::params![collection_id, user_title_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn remove_from_collection(db: State<Database>, collection_id: i64, user_title_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM collection_items WHERE collection_id = ?1 AND user_title_id = ?2", rusqlite::params![collection_id, user_title_id]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_user_collections_for_title(db: State<Database>, user_title_id: i64) -> Result<Vec<i64>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT collection_id FROM collection_items WHERE user_title_id = ?1").map_err(|e| e.to_string())?;
    let ids = stmt.query_map([user_title_id], |row| row.get(0)).map_err(|e| e.to_string())?.collect::<Result<Vec<i64>, _>>().map_err(|e| e.to_string())?;
    Ok(ids)
}