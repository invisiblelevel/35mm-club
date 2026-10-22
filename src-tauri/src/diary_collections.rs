use crate::db::Database;
use crate::models::{DiaryEntry, Collection, CollectionItem, LibraryDiaryEntry, RewatchEntry};
use crate::sync::log;
use tauri::State;
use std::sync::Arc;

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
pub fn get_diary(db: State<'_, Arc<Database>>) -> Result<Vec<DiaryEntry>, String> {
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
pub fn get_library_diary(db: State<'_, Arc<Database>>, status: Option<String>) -> Result<Vec<LibraryDiaryEntry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let sql_base = "SELECT ut.id, t.title, t.poster_path, t.type, t.tmdb_id, ut.status, ut.rating, ut.notes, ut.added_at
                    FROM user_titles ut
                    JOIN titles t ON t.id = ut.title_id";

    let map_row = |row: &rusqlite::Row| -> rusqlite::Result<LibraryDiaryEntry> {
        Ok(LibraryDiaryEntry {
            user_title_id: row.get(0)?,
            title: row.get(1)?,
            poster_url: resolve_poster_url(row.get::<_, Option<String>>(2)?),
            type_field: row.get(3)?,
            tmdb_id: row.get(4)?,
            status: row.get(5)?,
            rating: row.get(6)?,
            notes: row.get(7)?,
            added_at: row.get(8)?,
        })
    };

    let entries = if let Some(ref s) = status {
        let sql = format!("{} WHERE ut.status = ?1 ORDER BY ut.updated_at DESC", sql_base);
        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
        stmt.query_map([s], map_row).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    } else {
        let sql = format!("{} ORDER BY ut.updated_at DESC", sql_base);
        let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
        stmt.query_map([], map_row).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    };

    Ok(entries)
}

// ==========================================
// REWATCH TRACKER
// ==========================================

#[tauri::command]
pub fn get_rewatches(
    db: State<'_, Arc<Database>>,
    user_title_id: i64,
) -> Result<Vec<RewatchEntry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT id, watched_at, rating, notes
         FROM watch_entries
         WHERE user_title_id = ?1 AND is_rewatch = 1
         ORDER BY watched_at DESC"
    ).map_err(|e| e.to_string())?;

    let entries = stmt.query_map([user_title_id], |row| Ok(RewatchEntry {
        id: row.get(0)?,
        watched_at: row.get(1)?,
        rating: row.get(2)?,
        notes: row.get(3)?,
    })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    Ok(entries)
}

#[tauri::command]
pub fn add_rewatch(
    db: State<'_, Arc<Database>>,
    user_title_id: i64,
    rating: Option<f64>,
    notes: Option<String>,
) -> Result<Vec<crate::models::Achievement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Проверяем, что user_title существует
    let _: i64 = conn.query_row(
        "SELECT id FROM user_titles WHERE id = ?1",
        [user_title_id],
        |r| r.get(0),
    ).map_err(|_| "Тайтл не найден в библиотеке".to_string())?;

    // Вставляем запись о пересмотре
    conn.execute(
        "INSERT INTO watch_entries (user_title_id, watched_at, rating, notes, is_rewatch)
         VALUES (?1, datetime('now'), ?2, ?3, 1)",
        rusqlite::params![user_title_id, rating, notes],
    ).map_err(|e| e.to_string())?;

    let rewatch_id = conn.last_insert_rowid();

    // Логируем для синка
    let _ = log::record_watch_entry(&conn, rewatch_id);

    // XP за пересмотр (5 XP — по желанию, можно 0)
    let _ = crate::achievements::award_xp_internal(&conn, 5);
    let _ = log::record_profile(&conn);

    // Проверяем ачивки
    crate::achievements::check_achievements_internal(&conn)
}

#[tauri::command]
pub fn delete_rewatch(
    db: State<'_, Arc<Database>>,
    rewatch_id: i64,
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Узнаём user_title_id и watched_at перед удалением (для лога)
    let (user_title_id, watched_at, type_field, tmdb_id): (i64, String, String, Option<i64>) = conn.query_row(
        "SELECT we.user_title_id, we.watched_at, t.type, t.tmdb_id
         FROM watch_entries we
         JOIN user_titles ut ON ut.id = we.user_title_id
         JOIN titles t ON t.id = ut.title_id
         WHERE we.id = ?1",
        [rewatch_id],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
    ).map_err(|_| "Пересмотр не найден".to_string())?;

    conn.execute("DELETE FROM watch_entries WHERE id = ?1 AND is_rewatch = 1", [rewatch_id])
        .map_err(|e| e.to_string())?;

    // Логируем удаление (если нужно — пока пропускаем, watch_entry delete не поддерживается в crdt)
    let _ = (user_title_id, watched_at, type_field, tmdb_id);

    Ok(())
}

#[tauri::command]
pub fn get_collections(db: State<'_, Arc<Database>>) -> Result<Vec<Collection>, String> {
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
pub fn create_collection(db: State<'_, Arc<Database>>, name: String, description: Option<String>) -> Result<i64, String> {
    if name.trim().is_empty() { return Err("Название не может быть пустым".to_string()); }
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO collections (uuid, name, description) VALUES (lower(hex(randomblob(16))), ?1, ?2)", rusqlite::params![name.trim(), description]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    let _ = log::record_collection(&conn, id);
    Ok(id)
}

#[tauri::command]
pub fn delete_collection(db: State<'_, Arc<Database>>, collection_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let uuid: Option<String> = conn.query_row(
        "SELECT uuid FROM collections WHERE id = ?1",
        [collection_id],
        |r| r.get(0),
    ).ok().flatten();
    conn.execute("DELETE FROM collections WHERE id = ?1", [collection_id]).map_err(|e| e.to_string())?;
    if let Some(u) = uuid {
        let _ = log::record_collection_deleted(&conn, &u);
    }
    Ok(())
}

#[tauri::command]
pub fn get_collection_items(db: State<'_, Arc<Database>>, collection_id: i64) -> Result<Vec<CollectionItem>, String> {
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
pub fn add_to_collection(db: State<'_, Arc<Database>>, collection_id: i64, user_title_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("INSERT OR IGNORE INTO collection_items (collection_id, user_title_id) VALUES (?1, ?2)", rusqlite::params![collection_id, user_title_id]).map_err(|e| e.to_string())?;
    let _ = log::record_collection_item(&conn, collection_id, user_title_id);
    Ok(())
}

#[tauri::command]
pub fn remove_from_collection(db: State<'_, Arc<Database>>, collection_id: i64, user_title_id: i64) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let ids: Option<(String, String, i64)> = conn.query_row(
        "SELECT c.uuid, t.type, t.tmdb_id
         FROM collections c, user_titles ut
         JOIN titles t ON t.id = ut.title_id
         WHERE c.id = ?1 AND ut.id = ?2",
        rusqlite::params![collection_id, user_title_id],
        |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)),
    ).ok();

    conn.execute("DELETE FROM collection_items WHERE collection_id = ?1 AND user_title_id = ?2", rusqlite::params![collection_id, user_title_id]).map_err(|e| e.to_string())?;

    if let Some((uuid, type_field, tmdb_id)) = ids {
        let title_key = format!("{}:{}", type_field, tmdb_id);
        let _ = log::record_collection_item_deleted(&conn, &uuid, &title_key);
    }

    Ok(())
}

#[tauri::command]
pub fn get_user_collections_for_title(db: State<'_, Arc<Database>>, user_title_id: i64) -> Result<Vec<i64>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT collection_id FROM collection_items WHERE user_title_id = ?1").map_err(|e| e.to_string())?;
    let ids = stmt.query_map([user_title_id], |row| row.get(0)).map_err(|e| e.to_string())?.collect::<Result<Vec<i64>, _>>().map_err(|e| e.to_string())?;
    Ok(ids)
}

#[tauri::command]
pub fn get_all_rewatches(db: State<'_, Arc<Database>>) -> Result<Vec<crate::models::RewatchDiaryEntry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT we.id, we.watched_at, we.rating, we.notes,
                t.title, t.poster_path, t.type, t.tmdb_id
         FROM watch_entries we
         JOIN user_titles ut ON ut.id = we.user_title_id
         JOIN titles t ON t.id = ut.title_id
         WHERE we.is_rewatch = 1
         ORDER BY we.watched_at DESC"
    ).map_err(|e| e.to_string())?;

    let entries = stmt.query_map([], |row| Ok(crate::models::RewatchDiaryEntry {
        id: row.get(0)?,
        watched_at: row.get(1)?,
        rating: row.get(2)?,
        notes: row.get(3)?,
        title: row.get(4)?,
        poster_url: resolve_poster_url(row.get::<_, Option<String>>(5)?),
        type_field: row.get(6)?,
        tmdb_id: row.get(7)?,
    })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    Ok(entries)
}