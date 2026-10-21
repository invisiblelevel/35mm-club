use rusqlite::Connection;

use super::types::SyncRecord;

/// Применяет входящую запись из sync_log пира.
/// Возвращает true, если изменение реально применилось (было новее локального).
/// Возвращает false, если локальная версия новее (LWW) или запись идентична.
pub fn apply_remote_change(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    match record.entity_type.as_str() {
        "profile" => apply_profile(conn, record),
        "title" => apply_title(conn, record),
        "title_metadata" => apply_title_metadata(conn, record),
        "user_title" => apply_user_title(conn, record),
        "watch_entry" => apply_watch_entry(conn, record),
        "user_episode" => apply_user_episode(conn, record),
        "user_achievement" => apply_user_achievement(conn, record),
        "collection" => apply_collection(conn, record),
        "collection_item" => apply_collection_item(conn, record),
        other => Err(format!("неизвестный entity_type: {}", other)),
    }
}

// ============================================================
// profile
// ============================================================

fn apply_profile(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    if record.action == "delete" {
        return Ok(false); // профиль не удаляем
    }

    let payload: serde_json::Value = serde_json::from_str(
        record.payload.as_deref().ok_or("profile: пустой payload")?,
    )
    .map_err(|e| e.to_string())?;

    let local_updated: String = conn
        .query_row("SELECT updated_at FROM user_profile WHERE id = 1", [], |r| r.get(0))
        .unwrap_or_else(|_| "1970-01-01 00:00:00".to_string());

    if record.updated_at <= local_updated {
        return Ok(false); // локальная версия новее или равна
    }

    let username = payload.get("username").and_then(|v| v.as_str()).unwrap_or("Киноман");
    let avatar_path = payload.get("avatar_path").and_then(|v| v.as_str());
    let xp = payload.get("xp").and_then(|v| v.as_i64()).unwrap_or(0);
    let total_watch_minutes = payload
        .get("total_watch_minutes")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    conn.execute(
        "UPDATE user_profile
         SET username = ?1, avatar_path = ?2, xp = ?3, total_watch_minutes = ?4, updated_at = ?5
         WHERE id = 1",
        rusqlite::params![username, avatar_path, xp, total_watch_minutes, record.updated_at],
    )
    .map_err(|e| e.to_string())?;

    Ok(true)
}

// ============================================================
// title
// ============================================================

fn apply_title(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    if record.action == "delete" {
        return Ok(false);
    }

    let payload: serde_json::Value =
        serde_json::from_str(record.payload.as_deref().ok_or("title: пустой payload")?)
            .map_err(|e| e.to_string())?;

    let tmdb_id = payload.get("tmdb_id").and_then(|v| v.as_i64()).ok_or("title: нет tmdb_id")?;
    let type_field = payload.get("type").and_then(|v| v.as_str()).ok_or("title: нет type")?;

    // Ищем существующий title по (tmdb_id, type). Берём MIN(id) на случай дубликатов.
    let existing: Option<i64> = conn
        .query_row(
            "SELECT MIN(id) FROM titles WHERE tmdb_id = ?1 AND type = ?2",
            rusqlite::params![tmdb_id, type_field],
            |r| r.get(0),
        )
        .ok()
        .flatten();

    let title = payload.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let original_title = payload.get("original_title").and_then(|v| v.as_str());
    let year = payload.get("year").and_then(|v| v.as_i64());
    let poster_path = payload.get("poster_path").and_then(|v| v.as_str());
    let overview = payload.get("overview").and_then(|v| v.as_str());
    let imdb_rating = payload.get("imdb_rating").and_then(|v| v.as_f64());
    let runtime = payload.get("runtime").and_then(|v| v.as_i64());
    let language = payload.get("language").and_then(|v| v.as_str());
    let episodes_total = payload.get("episodes_total").and_then(|v| v.as_i64());
    let anime_format = payload.get("anime_format").and_then(|v| v.as_str());
    let anime_status = payload.get("anime_status").and_then(|v| v.as_str());
    let studio = payload.get("studio").and_then(|v| v.as_str());

    match existing {
        Some(id) => {
            // LWW: если локальная запись новее — скип.
            let local_created: String = conn
                .query_row("SELECT created_at FROM titles WHERE id = ?1", [id], |r| r.get(0))
                .unwrap_or_default();
            if record.updated_at <= local_created {
                return Ok(false);
            }

            conn.execute(
                "UPDATE titles SET
                    title = ?1,
                    original_title = ?2,
                    year = ?3,
                    poster_path = ?4,
                    overview = ?5,
                    imdb_rating = ?6,
                    runtime = ?7,
                    language = ?8,
                    episodes_total = ?9,
                    anime_format = ?10,
                    anime_status = ?11,
                    studio = ?12
                 WHERE id = ?13",
                rusqlite::params![
                    title, original_title, year, poster_path, overview, imdb_rating,
                    runtime, language, episodes_total, anime_format, anime_status, studio, id
                ],
            )
            .map_err(|e| e.to_string())?;
        }
        None => {
            conn.execute(
                "INSERT INTO titles (tmdb_id, type, title, original_title, year, poster_path,
                                    overview, imdb_rating, runtime, language, episodes_total,
                                    anime_format, anime_status, studio, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                rusqlite::params![
                    tmdb_id, type_field, title, original_title, year, poster_path,
                    overview, imdb_rating, runtime, language, episodes_total,
                    anime_format, anime_status, studio, record.updated_at
                ],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(true)
}

// ============================================================
// title_metadata
// ============================================================

fn apply_title_metadata(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    if record.action == "delete" {
        return Ok(false);
    }

    let payload: serde_json::Value =
        serde_json::from_str(record.payload.as_deref().ok_or("title_metadata: пустой payload")?)
            .map_err(|e| e.to_string())?;

    // entity_key = "movie:550"
    let (type_field, tmdb_id_str) = record
        .entity_key
        .split_once(':')
        .ok_or("title_metadata: плохой entity_key")?;
    let tmdb_id: i64 = tmdb_id_str.parse().map_err(|_| "title_metadata: плохой tmdb_id")?;

    let title_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM titles WHERE tmdb_id = ?1 AND type = ?2",
            rusqlite::params![tmdb_id, type_field],
            |r| r.get(0),
        )
        .ok();

    let title_id = match title_id {
        Some(id) => id,
        None => return Ok(false), // тайтла нет — пропускаем (придёт в следующем батче)
    };

    let cast_ids = payload.get("cast_ids").and_then(|v| v.as_str());
    let director_ids = payload.get("director_ids").and_then(|v| v.as_str());
    let producer_ids = payload.get("producer_ids").and_then(|v| v.as_str());
    let production_company_ids = payload.get("production_company_ids").and_then(|v| v.as_str());
    let keyword_ids = payload.get("keyword_ids").and_then(|v| v.as_str());
    let runtime = payload.get("runtime").and_then(|v| v.as_i64());
    let release_year = payload.get("release_year").and_then(|v| v.as_i64());
    let collection_id = payload.get("collection_id").and_then(|v| v.as_i64());

    conn.execute(
        "INSERT INTO title_metadata (title_id, cast_ids, director_ids, producer_ids,
                                    production_company_ids, keyword_ids, runtime,
                                    release_year, collection_id)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
         ON CONFLICT(title_id) DO UPDATE SET
            cast_ids = excluded.cast_ids,
            director_ids = excluded.director_ids,
            producer_ids = excluded.producer_ids,
            production_company_ids = excluded.production_company_ids,
            keyword_ids = excluded.keyword_ids,
            runtime = excluded.runtime,
            release_year = excluded.release_year,
            collection_id = COALESCE(excluded.collection_id, title_metadata.collection_id)",
        rusqlite::params![
            title_id, cast_ids, director_ids, producer_ids, production_company_ids,
            keyword_ids, runtime, release_year, collection_id
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(true)
}

// ============================================================
// user_title
// ============================================================

fn apply_user_title(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    // entity_key = "movie:550"
    let (type_field, tmdb_id_str) = record
        .entity_key
        .split_once(':')
        .ok_or("user_title: плохой entity_key")?;
    let tmdb_id: i64 = tmdb_id_str.parse().map_err(|_| "user_title: плохой tmdb_id")?;

    if record.action == "delete" {
        // Найти title_id и удалить user_titles
        let title_id: Option<i64> = conn
            .query_row(
                "SELECT id FROM titles WHERE tmdb_id = ?1 AND type = ?2",
                rusqlite::params![tmdb_id, type_field],
                |r| r.get(0),
            )
            .ok();
        if let Some(tid) = title_id {
            conn.execute("DELETE FROM user_titles WHERE title_id = ?1", [tid])
                .map_err(|e| e.to_string())?;
            return Ok(true);
        }
        return Ok(false);
    }

    let payload: serde_json::Value =
        serde_json::from_str(record.payload.as_deref().ok_or("user_title: пустой payload")?)
            .map_err(|e| e.to_string())?;

    // Локальный title_id — должен существовать (приходит раньше через title)
    let title_id: Option<i64> = conn
        .query_row(
            "SELECT id FROM titles WHERE tmdb_id = ?1 AND type = ?2",
            rusqlite::params![tmdb_id, type_field],
            |r| r.get(0),
        )
        .ok();

    let title_id = match title_id {
        Some(id) => id,
        None => return Ok(false), // тайтла нет — пропускаем, придёт в следующем батче
    };

    // LWW
    let local_updated: Option<String> = conn
        .query_row(
            "SELECT updated_at FROM user_titles WHERE title_id = ?1",
            [title_id],
            |r| r.get(0),
        )
        .ok();

    if let Some(local) = local_updated {
        if record.updated_at <= local {
            return Ok(false);
        }
    }

    let status = payload.get("status").and_then(|v| v.as_str()).unwrap_or("planned");
    let rating = payload.get("rating").and_then(|v| v.as_f64());
    let notes = payload.get("notes").and_then(|v| v.as_str());
    let started_at = payload.get("started_at").and_then(|v| v.as_str());
    let completed_at = payload.get("completed_at").and_then(|v| v.as_str());
    let anime_episodes_watched = payload
        .get("anime_episodes_watched")
        .and_then(|v| v.as_i64())
        .unwrap_or(0);
    let added_at = payload
        .get("added_at")
        .and_then(|v| v.as_str())
        .unwrap_or("2026-01-01 00:00:00");

    conn.execute(
        "INSERT INTO user_titles (title_id, status, rating, notes, started_at, completed_at,
                                 anime_episodes_watched, added_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
         ON CONFLICT(title_id) DO UPDATE SET
            status = excluded.status,
            rating = excluded.rating,
            notes = excluded.notes,
            started_at = excluded.started_at,
            completed_at = excluded.completed_at,
            anime_episodes_watched = excluded.anime_episodes_watched,
            updated_at = excluded.updated_at",
        rusqlite::params![
            title_id, status, rating, notes, started_at, completed_at,
            anime_episodes_watched, added_at, record.updated_at
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(true)
}

// ============================================================
// watch_entry
// ============================================================

fn apply_watch_entry(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    if record.action == "delete" {
        return Ok(false); // пока не поддерживаем удаление записей дневника через синк
    }

    let payload: serde_json::Value =
        serde_json::from_str(record.payload.as_deref().ok_or("watch_entry: пустой payload")?)
            .map_err(|e| e.to_string())?;

    let type_field = payload.get("type").and_then(|v| v.as_str()).ok_or("watch_entry: нет type")?;
    let tmdb_id = payload.get("tmdb_id").and_then(|v| v.as_i64()).ok_or("watch_entry: нет tmdb_id")?;

    // Найти user_title_id
    let user_title_id: Option<i64> = conn
        .query_row(
            "SELECT ut.id FROM user_titles ut
             JOIN titles t ON t.id = ut.title_id
             WHERE t.tmdb_id = ?1 AND t.type = ?2",
            rusqlite::params![tmdb_id, type_field],
            |r| r.get(0),
        )
        .ok();

    let user_title_id = match user_title_id {
        Some(id) => id,
        None => return Ok(false),
    };

    // Натуральный ключ: (user_title_id, watched_at, is_rewatch)
    let watched_at = payload
        .get("watched_at")
        .and_then(|v| v.as_str())
        .ok_or("watch_entry: нет watched_at")?;
    let is_rewatch = payload
        .get("is_rewatch")
        .and_then(|v| v.as_bool())
        .unwrap_or(false) as i64;
    let rating = payload.get("rating").and_then(|v| v.as_f64());
    let notes = payload.get("notes").and_then(|v| v.as_str());

    // LWW: ищем существующую запись по натуральному ключу
    let existing: Option<(i64, String)> = conn
        .query_row(
            "SELECT id, watched_at FROM watch_entries
             WHERE user_title_id = ?1 AND watched_at = ?2 AND is_rewatch = ?3",
            rusqlite::params![user_title_id, watched_at, is_rewatch],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .ok();

    match existing {
        Some((id, local_watched_at)) => {
            if record.updated_at <= local_watched_at {
                return Ok(false);
            }
            conn.execute(
                "UPDATE watch_entries SET rating = ?1, notes = ?2 WHERE id = ?3",
                rusqlite::params![rating, notes, id],
            )
            .map_err(|e| e.to_string())?;
        }
        None => {
            conn.execute(
                "INSERT INTO watch_entries (user_title_id, watched_at, rating, notes, is_rewatch)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![user_title_id, watched_at, rating, notes, is_rewatch],
            )
            .map_err(|e| e.to_string())?;
        }
    }

    Ok(true)
}

// ============================================================
// user_episode
// ============================================================

fn apply_user_episode(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    // entity_key = "tv:1399|1|5"
    let parts: Vec<&str> = record.entity_key.split('|').collect();
    if parts.len() != 3 {
        return Err("user_episode: плохой entity_key".to_string());
    }
    let title_parts: Vec<&str> = parts[0].splitn(2, ':').collect();
    if title_parts.len() != 2 {
        return Err("user_episode: плохой title_key".to_string());
    }
    let type_field = title_parts[0];
    let tmdb_id: i64 = title_parts[1].parse().map_err(|_| "user_episode: плохой tmdb_id")?;
    let season: i64 = parts[1].parse().map_err(|_| "user_episode: плохой season")?;
    let episode: i64 = parts[2].parse().map_err(|_| "user_episode: плохой episode")?;

    // Найти episode_id локально
    let episode_id: Option<i64> = conn
        .query_row(
            "SELECT e.id FROM episodes e
             JOIN seasons s ON s.id = e.season_id
             JOIN titles t ON t.id = s.title_id
             WHERE t.tmdb_id = ?1 AND t.type = ?2
               AND s.tmdb_season_number = ?3 AND e.tmdb_episode_number = ?4",
            rusqlite::params![tmdb_id, type_field, season, episode],
            |r| r.get(0),
        )
        .ok();

    let episode_id = match episode_id {
        Some(id) => id,
        None => return Ok(false), // серии нет локально — пропускаем
    };

    if record.action == "delete" {
        conn.execute("DELETE FROM user_episodes WHERE episode_id = ?1", [episode_id])
            .map_err(|e| e.to_string())?;
        return Ok(true);
    }

    // LWW по watched_at
    let local_watched: Option<String> = conn
        .query_row(
            "SELECT watched_at FROM user_episodes WHERE episode_id = ?1",
            [episode_id],
            |r| r.get(0),
        )
        .ok();

    if let Some(local) = local_watched {
        if record.updated_at <= local {
            return Ok(false);
        }
    }

    conn.execute(
        "INSERT INTO user_episodes (episode_id, watched_at) VALUES (?1, ?2)
         ON CONFLICT(episode_id) DO UPDATE SET watched_at = excluded.watched_at",
        rusqlite::params![episode_id, record.updated_at],
    )
    .map_err(|e| e.to_string())?;

    Ok(true)
}

// ============================================================
// user_achievement
// ============================================================

fn apply_user_achievement(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    if record.action == "delete" {
        return Ok(false); // ачивки не отзываем
    }

    let payload: serde_json::Value =
        serde_json::from_str(record.payload.as_deref().ok_or("user_achievement: пустой payload")?)
            .map_err(|e| e.to_string())?;

    let code = payload.get("code").and_then(|v| v.as_str()).ok_or("user_achievement: нет code")?;

    let achievement_id: Option<i64> = conn
        .query_row("SELECT id FROM achievements WHERE code = ?1", [code], |r| r.get(0))
        .ok();

    let achievement_id = match achievement_id {
        Some(id) => id,
        None => return Ok(false), // нет такой ачивки — пропускаем
    };

    let unlocked_at = payload
        .get("unlocked_at")
        .and_then(|v| v.as_str())
        .unwrap_or(&record.updated_at);

    // Ачивка либо есть, либо нет — LWW по unlocked_at
    let local_unlocked: Option<String> = conn
        .query_row(
            "SELECT unlocked_at FROM user_achievements WHERE achievement_id = ?1",
            [achievement_id],
            |r| r.get(0),
        )
        .ok();

    if let Some(local) = local_unlocked {
        if record.updated_at <= local {
            return Ok(false);
        }
    }

    conn.execute(
        "INSERT INTO user_achievements (achievement_id, unlocked_at, updated_at)
         VALUES (?1, ?2, ?3)
         ON CONFLICT(achievement_id) DO UPDATE SET
            unlocked_at = excluded.unlocked_at,
            updated_at = excluded.updated_at",
        rusqlite::params![achievement_id, unlocked_at, record.updated_at],
    )
    .map_err(|e| e.to_string())?;

    Ok(true)
}

// ============================================================
// collection
// ============================================================

fn apply_collection(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    // entity_key = "collection:<uuid>"
    let uuid = record
        .entity_key
        .strip_prefix("collection:")
        .ok_or("collection: плохой entity_key")?;

    if record.action == "delete" {
        conn.execute("DELETE FROM collections WHERE uuid = ?1", [uuid])
            .map_err(|e| e.to_string())?;
        return Ok(true);
    }

    let payload: serde_json::Value =
        serde_json::from_str(record.payload.as_deref().ok_or("collection: пустой payload")?)
            .map_err(|e| e.to_string())?;

    let name = payload.get("name").and_then(|v| v.as_str()).ok_or("collection: нет name")?;
    let description = payload.get("description").and_then(|v| v.as_str());
    let created_at = payload
        .get("created_at")
        .and_then(|v| v.as_str())
        .unwrap_or(&record.updated_at);

    conn.execute(
        "INSERT INTO collections (uuid, name, description, created_at)
         VALUES (?1, ?2, ?3, ?4)
         ON CONFLICT(uuid) DO UPDATE SET
            name = excluded.name,
            description = excluded.description",
        rusqlite::params![uuid, name, description, created_at],
    )
    .map_err(|e| e.to_string())?;

    Ok(true)
}

// ============================================================
// collection_item
// ============================================================

fn apply_collection_item(conn: &Connection, record: &SyncRecord) -> Result<bool, String> {
    // entity_key = "collection_item:<uuid>|<title_key>"
    let inner = record
        .entity_key
        .strip_prefix("collection_item:")
        .ok_or("collection_item: плохой entity_key")?;
    let (coll_uuid, title_key) = inner
        .split_once('|')
        .ok_or("collection_item: плохой entity_key")?;

    let collection_id: Option<i64> = conn
        .query_row("SELECT id FROM collections WHERE uuid = ?1", [coll_uuid], |r| r.get(0))
        .ok();
    let collection_id = match collection_id {
        Some(id) => id,
        None => return Ok(false),
    };

    let title_parts: Vec<&str> = title_key.splitn(2, ':').collect();
    if title_parts.len() != 2 {
        return Ok(false);
    }
    let type_field = title_parts[0];
    let tmdb_id: i64 = match title_parts[1].parse() {
        Ok(v) => v,
        Err(_) => return Ok(false),
    };

    let user_title_id: Option<i64> = conn
        .query_row(
            "SELECT ut.id FROM user_titles ut
             JOIN titles t ON t.id = ut.title_id
             WHERE t.tmdb_id = ?1 AND t.type = ?2",
            rusqlite::params![tmdb_id, type_field],
            |r| r.get(0),
        )
        .ok();
    let user_title_id = match user_title_id {
        Some(id) => id,
        None => return Ok(false),
    };

    if record.action == "delete" {
        conn.execute(
            "DELETE FROM collection_items WHERE collection_id = ?1 AND user_title_id = ?2",
            rusqlite::params![collection_id, user_title_id],
        )
        .map_err(|e| e.to_string())?;
        return Ok(true);
    }

    let added_at = payload_added_at(record).unwrap_or_else(|| record.updated_at.clone());

    conn.execute(
        "INSERT OR IGNORE INTO collection_items (collection_id, user_title_id, added_at)
         VALUES (?1, ?2, ?3)",
        rusqlite::params![collection_id, user_title_id, added_at],
    )
    .map_err(|e| e.to_string())?;

    Ok(true)
}

fn payload_added_at(record: &SyncRecord) -> Option<String> {
    let p = record.payload.as_deref()?;
    let v: serde_json::Value = serde_json::from_str(p).ok()?;
    v.get("added_at").and_then(|x| x.as_str()).map(|s| s.to_string())
}