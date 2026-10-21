use rusqlite::Connection;

use super::identity;

/// Записывает изменение в sync_log.
/// `entity_type` — тип сущности (см. ниже).
/// `entity_key` — натуральный ключ (например, "movie:550" или "achievement:first_step").
/// `action` — "upsert" | "delete".
/// `payload` — JSON с полями (для delete можно None).
pub fn record_change(
    conn: &Connection,
    entity_type: &str,
    entity_key: &str,
    action: &str,
    payload: Option<&str>,
    updated_at: &str,
) -> Result<(), String> {
    let device_id: String = conn
        .query_row("SELECT device_id FROM device_identity WHERE id = 1", [], |r| {
            r.get(0)
        })
        .map_err(|e| format!("device_identity не инициализирована: {}", e))?;

    conn.execute(
        "INSERT INTO sync_log (device_id, entity_type, entity_key, action, payload, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![device_id, entity_type, entity_key, action, payload, updated_at],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

// ============================================================
// Хелперы для конкретных сущностей
// ============================================================

/// profile — одна строка, ключ всегда "profile:1".
pub fn record_profile(conn: &Connection) -> Result<(), String> {
    let (username, avatar_path, xp, total_watch_minutes, updated_at): (
        String,
        Option<String>,
        i64,
        i64,
        String,
    ) = conn
        .query_row(
            "SELECT username, avatar_path, xp, total_watch_minutes, updated_at
             FROM user_profile WHERE id = 1",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )
        .map_err(|e| e.to_string())?;

    let payload = serde_json::json!({
        "username": username,
        "avatar_path": avatar_path,
        "xp": xp,
        "total_watch_minutes": total_watch_minutes,
    })
    .to_string();

    record_change(conn, "profile", "profile:1", "upsert", Some(&payload), &updated_at)
}

/// title — справочник. Ключ "movie:550" / "tv:1399" / "anime:20".
pub fn record_title(conn: &Connection, title_id: i64) -> Result<(), String> {
    let (tmdb_id, type_field, title, original_title, year, poster_path, overview, imdb_rating, runtime, language, episodes_total, anime_format, anime_status, studio, created_at): (
        Option<i64>, String, String, Option<String>, Option<i64>, Option<String>,
        Option<String>, Option<f64>, Option<i64>, Option<String>, Option<i64>,
        Option<String>, Option<String>, Option<String>, String,
    ) = conn
        .query_row(
            "SELECT tmdb_id, type, title, original_title, year, poster_path, overview,
                    imdb_rating, runtime, language, episodes_total, anime_format,
                    anime_status, studio, created_at
             FROM titles WHERE id = ?1",
            [title_id],
            |r| Ok((
                r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?,
                r.get(6)?, r.get(7)?, r.get(8)?, r.get(9)?, r.get(10)?, r.get(11)?,
                r.get(12)?, r.get(13)?, r.get(14)?,
            )),
        )
        .map_err(|e| e.to_string())?;

    let tmdb = tmdb_id.ok_or_else(|| "title без tmdb_id".to_string())?;
    let key = format!("{}:{}", type_field, tmdb);

    let payload = serde_json::json!({
        "tmdb_id": tmdb,
        "type": type_field,
        "title": title,
        "original_title": original_title,
        "year": year,
        "poster_path": poster_path,
        "overview": overview,
        "imdb_rating": imdb_rating,
        "runtime": runtime,
        "language": language,
        "episodes_total": episodes_total,
        "anime_format": anime_format,
        "anime_status": anime_status,
        "studio": studio,
    })
    .to_string();

    record_change(conn, "title", &key, "upsert", Some(&payload), &created_at)
}

/// title_metadata — справочник, LWW по release_year (по факту иммутабельно).
pub fn record_title_metadata(conn: &Connection, title_id: i64) -> Result<(), String> {
    let (type_field, tmdb_id, cast_ids, director_ids, producer_ids, production_company_ids, keyword_ids, runtime, release_year, collection_id): (
        String, Option<i64>, Option<String>, Option<String>, Option<String>,
        Option<String>, Option<String>, Option<i64>, Option<i64>, Option<i64>,
    ) = conn
        .query_row(
            "SELECT t.type, t.tmdb_id, tm.cast_ids, tm.director_ids, tm.producer_ids,
                    tm.production_company_ids, tm.keyword_ids, tm.runtime, tm.release_year, tm.collection_id
             FROM title_metadata tm JOIN titles t ON t.id = tm.title_id
             WHERE tm.title_id = ?1",
            [title_id],
            |r| Ok((
                r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?,
                r.get(5)?, r.get(6)?, r.get(7)?, r.get(8)?, r.get(9)?,
            )),
        )
        .map_err(|e| e.to_string())?;

    let tmdb = tmdb_id.ok_or_else(|| "title_metadata без tmdb_id".to_string())?;
    let key = format!("{}:{}", type_field, tmdb);

    let payload = serde_json::json!({
        "cast_ids": cast_ids,
        "director_ids": director_ids,
        "producer_ids": producer_ids,
        "production_company_ids": production_company_ids,
        "keyword_ids": keyword_ids,
        "runtime": runtime,
        "release_year": release_year,
        "collection_id": collection_id,
    })
    .to_string();

    let now: String = conn
        .query_row("SELECT datetime('now')", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;

    record_change(conn, "title_metadata", &key, "upsert", Some(&payload), &now)
}

/// user_title — ключ "movie:550" / "tv:1399" / "anime:20".
pub fn record_user_title(conn: &Connection, user_title_id: i64) -> Result<(), String> {
    let (type_field, tmdb_id, status, rating, notes, started_at, completed_at, anime_episodes_watched, added_at, updated_at): (
        String, Option<i64>, String, Option<f64>, Option<String>, Option<String>,
        Option<String>, i64, String, String,
    ) = conn
        .query_row(
            "SELECT t.type, t.tmdb_id, ut.status, ut.rating, ut.notes, ut.started_at,
                    ut.completed_at, ut.anime_episodes_watched, ut.added_at, ut.updated_at
             FROM user_titles ut JOIN titles t ON t.id = ut.title_id
             WHERE ut.id = ?1",
            [user_title_id],
            |r| Ok((
                r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?,
                r.get(6)?, r.get(7)?, r.get(8)?, r.get(9)?,
            )),
        )
        .map_err(|e| e.to_string())?;

    let tmdb = tmdb_id.ok_or_else(|| "user_title без tmdb_id".to_string())?;
    let key = format!("{}:{}", type_field, tmdb);

    let payload = serde_json::json!({
        "type": type_field,
        "tmdb_id": tmdb,
        "status": status,
        "rating": rating,
        "notes": notes,
        "started_at": started_at,
        "completed_at": completed_at,
        "anime_episodes_watched": anime_episodes_watched,
        "added_at": added_at,
    })
    .to_string();

    record_change(conn, "user_title", &key, "upsert", Some(&payload), &updated_at)
}

/// user_title — удаление.
pub fn record_user_title_deleted(conn: &Connection, key: &str) -> Result<(), String> {
    let now: String = conn
        .query_row("SELECT datetime('now')", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    record_change(conn, "user_title", key, "delete", None, &now)
}

/// watch_entry — ключ "movie:550|2026-10-21 12:34:56|0" (title_key|watched_at|is_rewatch).
/// LWW по watched_at (он же updated_at).
pub fn record_watch_entry(conn: &Connection, watch_entry_id: i64) -> Result<(), String> {
    let (type_field, tmdb_id, watched_at, rating, notes, is_rewatch): (
        String, Option<i64>, String, Option<f64>, Option<String>, i64,
    ) = conn
        .query_row(
            "SELECT t.type, t.tmdb_id, we.watched_at, we.rating, we.notes, we.is_rewatch
             FROM watch_entries we
             JOIN user_titles ut ON ut.id = we.user_title_id
             JOIN titles t ON t.id = ut.title_id
             WHERE we.id = ?1",
            [watch_entry_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?, r.get(5)?)),
        )
        .map_err(|e| e.to_string())?;

    let tmdb = tmdb_id.ok_or_else(|| "watch_entry без tmdb_id".to_string())?;
    let key = format!("{}:{}|{}|{}", type_field, tmdb, watched_at, is_rewatch);

    let payload = serde_json::json!({
        "type": type_field,
        "tmdb_id": tmdb,
        "watched_at": watched_at,
        "rating": rating,
        "notes": notes,
        "is_rewatch": is_rewatch != 0,
    })
    .to_string();

    record_change(conn, "watch_entry", &key, "upsert", Some(&payload), &watched_at)
}

/// user_episode — ключ "tv:1399|1|5" (title_key|season|episode).
/// LWW по watched_at.
pub fn record_user_episode(conn: &Connection, episode_id: i64) -> Result<(), String> {
    let (type_field, tmdb_id, season_num, ep_num, watched_at): (
        String, Option<i64>, i64, i64, String,
    ) = conn
        .query_row(
            "SELECT t.type, t.tmdb_id, s.tmdb_season_number, e.tmdb_episode_number, ue.watched_at
             FROM user_episodes ue
             JOIN episodes e ON e.id = ue.episode_id
             JOIN seasons s ON s.id = e.season_id
             JOIN titles t ON t.id = s.title_id
             WHERE ue.episode_id = ?1",
            [episode_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )
        .map_err(|e| e.to_string())?;

    let tmdb = tmdb_id.ok_or_else(|| "user_episode без tmdb_id".to_string())?;
    let key = format!("{}:{}|{}|{}", type_field, tmdb, season_num, ep_num);

    let payload = serde_json::json!({
        "type": type_field,
        "tmdb_id": tmdb,
        "season": season_num,
        "episode": ep_num,
        "watched_at": watched_at,
    })
    .to_string();

    record_change(conn, "user_episode", &key, "upsert", Some(&payload), &watched_at)
}

/// user_episode — удаление (снятие отметки).
pub fn record_user_episode_deleted(
    conn: &Connection,
    type_field: &str,
    tmdb_id: i64,
    season: i64,
    episode: i64,
) -> Result<(), String> {
    let key = format!("{}:{}|{}|{}", type_field, tmdb_id, season, episode);
    let now: String = conn
        .query_row("SELECT datetime('now')", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    record_change(conn, "user_episode", &key, "delete", None, &now)
}

/// user_achievement — ключ "achievement:<code>".
/// LWW по unlocked_at.
pub fn record_user_achievement(conn: &Connection, achievement_id: i64) -> Result<(), String> {
    let (code, unlocked_at): (String, String) = conn
        .query_row(
            "SELECT a.code, ua.unlocked_at
             FROM user_achievements ua JOIN achievements a ON a.id = ua.achievement_id
             WHERE ua.achievement_id = ?1",
            [achievement_id],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|e| e.to_string())?;

    let key = format!("achievement:{}", code);

    let payload = serde_json::json!({
        "code": code,
        "unlocked_at": unlocked_at,
    })
    .to_string();

    record_change(conn, "user_achievement", &key, "upsert", Some(&payload), &unlocked_at)
}

/// collection — ключ "collection:<uuid>".
pub fn record_collection(conn: &Connection, collection_id: i64) -> Result<(), String> {
    let (uuid, name, description, created_at): (Option<String>, String, Option<String>, String) = conn
        .query_row(
            "SELECT uuid, name, description, created_at FROM collections WHERE id = ?1",
            [collection_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .map_err(|e| e.to_string())?;

    let uuid = uuid.ok_or_else(|| "collection без uuid".to_string())?;
    let key = format!("collection:{}", uuid);

    let payload = serde_json::json!({
        "uuid": uuid,
        "name": name,
        "description": description,
    })
    .to_string();

    record_change(conn, "collection", &key, "upsert", Some(&payload), &created_at)
}

pub fn record_collection_deleted(conn: &Connection, uuid: &str) -> Result<(), String> {
    let key = format!("collection:{}", uuid);
    let now: String = conn
        .query_row("SELECT datetime('now')", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    record_change(conn, "collection", &key, "delete", None, &now)
}

/// collection_item — ключ "collection_item:<uuid>|<title_key>".
pub fn record_collection_item(
    conn: &Connection,
    collection_id: i64,
    user_title_id: i64,
) -> Result<(), String> {
    let (coll_uuid, type_field, tmdb_id, added_at): (Option<String>, String, Option<i64>, String) = conn
        .query_row(
            "SELECT c.uuid, t.type, t.tmdb_id, ci.added_at
             FROM collection_items ci
             JOIN collections c ON c.id = ci.collection_id
             JOIN user_titles ut ON ut.id = ci.user_title_id
             JOIN titles t ON t.id = ut.title_id
             WHERE ci.collection_id = ?1 AND ci.user_title_id = ?2",
            rusqlite::params![collection_id, user_title_id],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?)),
        )
        .map_err(|e| e.to_string())?;

    let coll_uuid = coll_uuid.ok_or_else(|| "collection без uuid".to_string())?;
    let tmdb = tmdb_id.ok_or_else(|| "collection_item без tmdb_id".to_string())?;
    let title_key = format!("{}:{}", type_field, tmdb);
    let key = format!("collection_item:{}|{}", coll_uuid, title_key);

    let payload = serde_json::json!({
        "collection_uuid": coll_uuid,
        "title_key": title_key,
    })
    .to_string();

    record_change(conn, "collection_item", &key, "upsert", Some(&payload), &added_at)
}

pub fn record_collection_item_deleted(
    conn: &Connection,
    coll_uuid: &str,
    title_key: &str,
) -> Result<(), String> {
    let key = format!("collection_item:{}|{}", coll_uuid, title_key);
    let now: String = conn
        .query_row("SELECT datetime('now')", [], |r| r.get(0))
        .map_err(|e| e.to_string())?;
    record_change(conn, "collection_item", &key, "delete", None, &now)
}

/// Проверяет, что identity инициализирована. Если нет — no-op (синк выключен).
pub fn is_ready(conn: &Connection) -> bool {
    identity::current_identity(conn).is_ok()
}