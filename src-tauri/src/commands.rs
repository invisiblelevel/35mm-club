use crate::db::Database;
use crate::models::*;
use crate::achievements::{award_xp_internal, add_watch_minutes, check_achievements_internal, calculate_level, xp_threshold_for_level, rank_name_for_level};
use tauri::State;

#[tauri::command]
pub fn get_profile(db: State<Database>) -> Result<UserProfile, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let (username, avatar_path, remote_id, xp, total_watch_minutes, created_at) = conn.query_row("SELECT username, avatar_path, remote_id, xp, total_watch_minutes, created_at FROM user_profile WHERE id = 1", [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?))).map_err(|e| e.to_string())?;
    let level = calculate_level(xp);
    let current_threshold = xp_threshold_for_level(level);
    let next_threshold = xp_threshold_for_level(level + 1);
    let xp_in_current_level = if level == 1 { xp } else { xp - current_threshold };
    let xp_to_next_level = next_threshold - xp;
    Ok(UserProfile { username, avatar_path, remote_id, level, xp, total_watch_minutes, created_at, xp_to_next_level, xp_in_current_level, rank_name: rank_name_for_level(level).to_string() })
}

#[tauri::command]
pub fn update_profile(db: State<Database>, username: Option<String>, avatar_path: Option<String>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    if let Some(name) = username { conn.execute("UPDATE user_profile SET username = ?1 WHERE id = 1", [name]).map_err(|e| e.to_string())?; }
    if let Some(path) = avatar_path { conn.execute("UPDATE user_profile SET avatar_path = ?1 WHERE id = 1", [path]).map_err(|e| e.to_string())?; }
    Ok(())
}

#[tauri::command]
pub fn award_xp(db: State<Database>, amount: i64) -> Result<i64, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    award_xp_internal(&conn, amount)
}

#[tauri::command]
pub fn reset_all_data(db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute_batch("DELETE FROM user_episodes; DELETE FROM episodes; DELETE FROM seasons; DELETE FROM user_showcase; DELETE FROM user_achievements; DELETE FROM user_ribbons; DELETE FROM watch_entries; DELETE FROM collection_items; DELETE FROM collections; DELETE FROM user_titles; DELETE FROM titles; UPDATE user_profile SET xp = 0, level = 1, total_watch_minutes = 0, avatar_path = NULL WHERE id = 1;").map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_library(db: State<Database>, content_type: String) -> Result<Vec<Title>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT t.id, t.tmdb_id, t.title, t.type, t.year, t.poster_path, ut.status, ut.rating, ut.added_at FROM titles t JOIN user_titles ut ON ut.title_id = t.id WHERE t.type = ?1 ORDER BY ut.added_at DESC").map_err(|e| e.to_string())?;
    let titles = stmt.query_map([content_type], |row| Ok(Title { id: row.get(0)?, tmdb_id: row.get(1)?, title: row.get(2)?, type_field: row.get(3)?, year: row.get(4)?, poster_path: row.get(5)?, status: row.get(6)?, rating: row.get(7)?, added_at: row.get(8)? })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    Ok(titles)
}

#[tauri::command]
pub fn search_local_library(db: State<Database>, query: String, content_type: Option<String>) -> Result<Vec<Title>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let pattern = format!("%{}%", query);
    let (sql, params_vec): (&str, Vec<Box<dyn rusqlite::ToSql>>) = match content_type {
        Some(ref ct) if !ct.is_empty() && ct != "multi" => ("SELECT t.id, t.tmdb_id, t.title, t.type, t.year, t.poster_path, ut.status, ut.rating, ut.added_at FROM titles t JOIN user_titles ut ON ut.title_id = t.id WHERE t.type = ?1 AND t.title LIKE ?2 COLLATE NOCASE ORDER BY ut.added_at DESC LIMIT 50", vec![Box::new(ct.clone()), Box::new(pattern)]),
        _ => ("SELECT t.id, t.tmdb_id, t.title, t.type, t.year, t.poster_path, ut.status, ut.rating, ut.added_at FROM titles t JOIN user_titles ut ON ut.title_id = t.id WHERE t.title LIKE ?1 COLLATE NOCASE ORDER BY ut.added_at DESC LIMIT 50", vec![Box::new(pattern)]),
    };
    let mut stmt = conn.prepare(sql).map_err(|e| e.to_string())?;
    let params_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|p| p.as_ref()).collect();
    let titles = stmt.query_map(params_refs.as_slice(), |row| Ok(Title { id: row.get(0)?, tmdb_id: row.get(1)?, title: row.get(2)?, type_field: row.get(3)?, year: row.get(4)?, poster_path: row.get(5)?, status: row.get(6)?, rating: row.get(7)?, added_at: row.get(8)? })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    Ok(titles)
}

#[tauri::command]
pub fn get_achievements(db: State<Database>) -> Result<Vec<Achievement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT a.id, a.code, a.category, a.icon, a.xp_reward, a.scope, a.tier, a.is_secret, a.title_key, a.description_key, ua.unlocked_at FROM achievements a LEFT JOIN user_achievements ua ON ua.achievement_id = a.id ORDER BY ua.unlocked_at DESC NULLS LAST, a.id ASC").map_err(|e| e.to_string())?;
    let achievements = stmt.query_map([], |row| {
        let unlocked_at: Option<String> = row.get(10)?;
        Ok(Achievement { id: row.get(0)?, code: row.get(1)?, category: row.get(2)?, icon: row.get(3)?, xp_reward: row.get(4)?, scope: row.get(5)?, tier: row.get(6)?, is_secret: row.get::<_, i64>(7)? != 0, title_key: row.get(8)?, description_key: row.get(9)?, unlocked: unlocked_at.is_some(), unlocked_at })
    }).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    Ok(achievements)
}

#[tauri::command]
pub fn get_user_title_data(db: State<Database>, tmdb_id: i64, type_field: String) -> Result<UserTitleData, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result = conn.query_row("SELECT ut.id, ut.status, ut.rating, ut.added_at FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.tmdb_id = ?1 AND t.type = ?2 LIMIT 1", rusqlite::params![tmdb_id, type_field], |row| Ok(UserTitleData { user_title_id: Some(row.get(0)?), status: Some(row.get(1)?), rating: row.get(2)?, added_at: row.get(3)? })).ok();
    Ok(result.unwrap_or(UserTitleData { user_title_id: None, status: None, rating: None, added_at: None }))
}

#[tauri::command]
pub fn update_rating(db: State<Database>, user_title_id: i64, rating: Option<f64>) -> Result<Vec<Achievement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE user_titles SET rating = ?1, updated_at = datetime('now') WHERE id = ?2", rusqlite::params![rating, user_title_id]).map_err(|e| e.to_string())?;
    check_achievements_internal(&conn)
}

#[tauri::command]
pub fn add_to_library(db: State<Database>, payload: AddToLibraryPayload) -> Result<Vec<Achievement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let existing: Option<i64> = conn.query_row("SELECT id FROM titles WHERE tmdb_id = ?1 AND type = ?2 AND title = ?3", rusqlite::params![payload.tmdb_id, payload.type_field, payload.title], |row| row.get(0)).ok();

    let cast_json = serde_json::to_string(&payload.cast_ids.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());
    let dir_json = serde_json::to_string(&payload.director_ids.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());
    let producer_json = serde_json::to_string(&payload.producer_ids.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());
    let company_json = serde_json::to_string(&payload.production_company_ids.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());
    let keyword_json = serde_json::to_string(&payload.keyword_ids.clone().unwrap_or_default()).unwrap_or_else(|_| "[]".to_string());

    let title_id = match existing {
        Some(id) => {
            conn.execute("INSERT INTO title_metadata (title_id, cast_ids, director_ids, runtime, release_year, collection_id, producer_ids, production_company_ids, keyword_ids) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9) ON CONFLICT(title_id) DO UPDATE SET cast_ids = excluded.cast_ids, director_ids = excluded.director_ids, runtime = excluded.runtime, release_year = excluded.release_year, collection_id = COALESCE(excluded.collection_id, title_metadata.collection_id), producer_ids = excluded.producer_ids, production_company_ids = excluded.production_company_ids, keyword_ids = excluded.keyword_ids", rusqlite::params![id, cast_json, dir_json, payload.runtime, payload.year, payload.collection_id, producer_json, company_json, keyword_json]).map_err(|e| e.to_string())?;
            id
        }
        None => {
            let locale = payload.locale.clone().unwrap_or_else(|| "ru".to_string());
            conn.execute("INSERT INTO titles (tmdb_id, type, title, original_title, year, poster_path, overview, imdb_rating, runtime, language) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)", rusqlite::params![payload.tmdb_id, payload.type_field, payload.title, payload.original_title, payload.year, payload.poster_path, payload.overview, payload.imdb_rating, payload.runtime, locale]).map_err(|e| e.to_string())?;
            let new_id = conn.last_insert_rowid();
            conn.execute("INSERT INTO title_metadata (title_id, cast_ids, director_ids, runtime, release_year, collection_id, producer_ids, production_company_ids, keyword_ids) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", rusqlite::params![new_id, cast_json, dir_json, payload.runtime, payload.year, payload.collection_id, producer_json, company_json, keyword_json]).map_err(|e| e.to_string())?;
            new_id
        }
    };

    let already: Option<i64> = conn.query_row("SELECT id FROM user_titles WHERE title_id = ?1", [title_id], |row| row.get(0)).ok();
    let mut xp_delta = 0; let mut minutes_delta = 0;
    match payload.status.as_str() {
        "watching" => xp_delta = 5,
        "planned" => xp_delta = 1,
        "completed" => { xp_delta = match payload.type_field.as_str() { "movie" => 15, "tv" => 40, "anime" => 25, _ => 15 }; if payload.type_field == "movie" { minutes_delta = payload.runtime.unwrap_or(0); } }
        _ => {}
    }

    let user_title_id = if let Some(user_title_id) = already {
        let old_status: String = conn.query_row("SELECT status FROM user_titles WHERE id = ?1", [user_title_id], |row| row.get(0)).map_err(|e| e.to_string())?;
        conn.execute("UPDATE user_titles SET status = ?1, updated_at = datetime('now') WHERE id = ?2", rusqlite::params![payload.status, user_title_id]).map_err(|e| e.to_string())?;
        if old_status != "completed" && payload.status == "completed" {
            award_xp_internal(&conn, xp_delta)?;
            add_watch_minutes(&conn, minutes_delta)?;
            conn.execute("INSERT INTO watch_entries (user_title_id, watched_at, is_rewatch) VALUES (?1, datetime('now'), 0)", [user_title_id]).map_err(|e| e.to_string())?;
        } else if old_status != "completed" {
            award_xp_internal(&conn, xp_delta)?;
            add_watch_minutes(&conn, minutes_delta)?;
        }
        user_title_id
    } else {
        conn.execute("INSERT INTO user_titles (title_id, status) VALUES (?1, ?2)", rusqlite::params![title_id, payload.status]).map_err(|e| e.to_string())?;
        let new_id = conn.last_insert_rowid();
        award_xp_internal(&conn, xp_delta)?;
        add_watch_minutes(&conn, minutes_delta)?;
        if payload.status == "completed" {
            conn.execute("INSERT INTO watch_entries (user_title_id, watched_at, is_rewatch) VALUES (?1, datetime('now'), 0)", [new_id]).map_err(|e| e.to_string())?;
        }
        new_id
    };
    let _ = user_title_id;
    check_achievements_internal(&conn)
}

#[tauri::command]
pub async fn search_movies(query: String, media_type: String, api_key: Option<String>, locale: String) -> Result<Vec<crate::tmdb::MovieSearchItem>, String> {
    crate::tmdb::search_movies(&query, &media_type, api_key, &locale).await
}

#[tauri::command]
pub async fn get_title_details(tmdb_id: i64, media_type: String, api_key: Option<String>, locale: String) -> Result<crate::tmdb::TitleDetails, String> {
    crate::tmdb::get_title_details(tmdb_id, &media_type, api_key, &locale).await
}

#[tauri::command]
pub fn set_avatar(app: tauri::AppHandle, db: State<Database>, source_path: String) -> Result<String, String> {
    use std::fs; use std::path::Path; use tauri::Manager;
    let src = Path::new(&source_path);
    if !src.exists() { return Err("Файл не найден".to_string()); }
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let avatars_dir = app_dir.join("avatars");
    fs::create_dir_all(&avatars_dir).map_err(|e| e.to_string())?;
    let ext = src.extension().and_then(|e| e.to_str()).unwrap_or("png");
    let dest = avatars_dir.join(format!("avatar.{}", ext));
    fs::copy(src, &dest).map_err(|e| format!("Не удалось скопировать: {}", e))?;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE user_profile SET avatar_path = ?1 WHERE id = 1", [dest.to_string_lossy().to_string()]).map_err(|e| e.to_string())?;
    Ok(dest.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn refresh_metadata(
    db: State<'_, Database>,
    api_key: Option<String>,
    locale: String,
) -> Result<usize, String> {
    let titles: Vec<(i64, i64, String)> = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT id, tmdb_id, type FROM titles WHERE tmdb_id IS NOT NULL AND type != 'anime'").map_err(|e| e.to_string())?;
        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?
    };

    let mut updated = 0;

    for (title_id, tmdb_id, type_field) in titles {
        if let Ok(details) = crate::tmdb::get_title_details(tmdb_id, &type_field, api_key.clone(), &locale).await {
            let cast_json = serde_json::to_string(&details.cast_ids).unwrap_or_else(|_| "[]".to_string());
            let dir_json = serde_json::to_string(&details.director_ids).unwrap_or_else(|_| "[]".to_string());
            let producer_json = serde_json::to_string(&details.producer_ids).unwrap_or_else(|_| "[]".to_string());
            let company_json = serde_json::to_string(&details.production_company_ids).unwrap_or_else(|_| "[]".to_string());
            let keyword_json = serde_json::to_string(&details.keyword_ids).unwrap_or_else(|_| "[]".to_string());

            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            conn.execute(
                "INSERT INTO title_metadata (title_id, cast_ids, director_ids, runtime, release_year, collection_id, producer_ids, production_company_ids, keyword_ids)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
                 ON CONFLICT(title_id) DO UPDATE SET
                    cast_ids = excluded.cast_ids,
                    director_ids = excluded.director_ids,
                    runtime = excluded.runtime,
                    release_year = excluded.release_year,
                    collection_id = COALESCE(excluded.collection_id, title_metadata.collection_id),
                    producer_ids = excluded.producer_ids,
                    production_company_ids = excluded.production_company_ids,
                    keyword_ids = excluded.keyword_ids",
                rusqlite::params![title_id, cast_json, dir_json, details.runtime, details.year, details.collection_id, producer_json, company_json, keyword_json]
            ).map_err(|e| e.to_string())?;
            updated += 1;
        }
    }

    Ok(updated)
}

#[tauri::command]
pub async fn refresh_titles_language(
    db: State<'_, Database>,
    api_key: Option<String>,
    locale: String,
) -> Result<usize, String> {
    let titles: Vec<(i64, i64, String)> = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT id, tmdb_id, type FROM titles WHERE tmdb_id IS NOT NULL AND type != 'anime'").map_err(|e| e.to_string())?;
        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .map_err(|e| e.to_string())?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| e.to_string())?
    };

    let mut updated = 0;

    for (title_id, tmdb_id, type_field) in titles {
        if let Ok(details) = crate::tmdb::get_title_details(tmdb_id, &type_field, api_key.clone(), &locale).await {
            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            let poster_path = details.poster_url.map(|p| p.replace("https://image.tmdb.org/t/p/w500", ""));
            conn.execute(
                "UPDATE titles SET title = ?1, overview = ?2, poster_path = ?3, language = ?4 WHERE id = ?5",
                rusqlite::params![details.title, details.overview, poster_path, locale, title_id]
            ).map_err(|e| e.to_string())?;
            updated += 1;
        }
    }

    Ok(updated)
}

#[tauri::command]
pub async fn ensure_title_language(
    db: State<'_, Database>,
    tmdb_id: i64,
    type_field: String,
    api_key: Option<String>,
    locale: String,
) -> Result<crate::tmdb::TitleDetails, String> {
    let details = crate::tmdb::get_title_details(tmdb_id, &type_field, api_key, &locale).await?;

    {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let poster_path = details.poster_url.clone().map(|p| p.replace("https://image.tmdb.org/t/p/w500", ""));
        conn.execute(
            "UPDATE titles SET title = ?1, overview = ?2, poster_path = ?3, language = ?4 WHERE tmdb_id = ?5 AND type = ?6",
            rusqlite::params![details.title, details.overview, poster_path, locale, tmdb_id, type_field]
        ).map_err(|e| e.to_string())?;
    }

    Ok(details)
}

// ==========================================
// SHIKIMORI / ANILIST (АНИМЕ)
// ==========================================

#[tauri::command]
pub async fn search_anime(query: String, locale: String) -> Result<Vec<crate::shikimori::ShikiSearchItem>, String> {
    // 1. Shikimori (основной источник, учитывает локаль)
    match crate::shikimori::search_anime(&query, 20, &locale).await {
        Ok(mut results) if !results.is_empty() => {
            // Если локаль не русская — подтягиваем английские описания из AniList
            if locale != "ru" {
                for item in results.iter_mut().take(10) {
                    if let Some(en_desc) = crate::anilist::get_description_by_mal_id(item.mal_id).await {
                        item.description = Some(en_desc);
                    }
                }
            }
            return Ok(results);
        }
        _ => {}
    }

    // 2. Fallback: AniList
    let anilist_results = crate::anilist::search_anime(&query, 1, 20).await.unwrap_or_default();
    Ok(anilist_results.into_iter().map(|a| {
        crate::shikimori::ShikiSearchItem {
            mal_id: a.id_mal.unwrap_or(a.anilist_id),
            title: a.title,
            title_original: a.title_romaji,
            description: a.description,
            cover_url: a.cover_url,
            episodes: a.episodes,
            average_score: a.average_score.map(|s| s as f64 / 10.0),
            format: a.format,
            year: a.year,
        }
    }).collect())
}

#[tauri::command]
pub async fn get_anime_details(
    mal_id: i64,
    locale: String,
) -> Result<crate::shikimori::ShikiAnimeDetails, String> {
    let mut details = crate::shikimori::get_anime_details(mal_id, &locale).await?;

    // Если локаль не русская, пробуем взять английское описание из AniList
    if locale != "ru" {
        if let Some(en_desc) = crate::anilist::get_description_by_mal_id(mal_id).await {
            details.description = Some(en_desc);
        }
    }

    Ok(details)
}

#[tauri::command]
pub async fn add_anime_to_library(
    db: State<'_, Database>,
    payload: AddAnimePayload,
) -> Result<Vec<Achievement>, String> {
    let mal_id = payload.mal_id.ok_or_else(|| "mal_id обязателен".to_string())?;

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let existing: Option<i64> = conn.query_row(
        "SELECT id FROM titles WHERE tmdb_id = ?1 AND type = 'anime'",
        [mal_id], |r| r.get(0)
    ).ok();

    let title_id = match existing {
        Some(id) => id,
        None => {
            conn.execute(
                "INSERT INTO titles (anilist_id, tmdb_id, type, title, overview, poster_path, episodes_total, anime_format, anime_status, studio, year)
                 VALUES (?1, ?1, 'anime', ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                rusqlite::params![
                    mal_id,
                    payload.title,
                    payload.description,
                    payload.cover_url,
                    payload.episodes,
                    payload.format,
                    payload.status,
                    payload.studio,
                    payload.year
                ]
            ).map_err(|e| e.to_string())?;
            conn.last_insert_rowid()
        }
    };

    let already: Option<i64> = conn.query_row("SELECT id FROM user_titles WHERE title_id = ?1", [title_id], |r| r.get(0)).ok();

    let xp_delta = match payload.status.as_str() {
        "watching" => 5,
        "planned" => 1,
        "completed" => 25,
        _ => 0,
    };

    // ФИКС: если completed — берём episodes_total из titles, а не из payload (payload может быть null).
    let total_from_db: Option<i64> = conn.query_row(
        "SELECT episodes_total FROM titles WHERE id = ?1", [title_id], |r| r.get(0)
    ).ok().flatten();

    let watched = if payload.status == "completed" {
        total_from_db.or(payload.episodes).unwrap_or(0)
    } else {
        0
    };

    let anime_minutes = if payload.status == "completed" {
        watched * 24
    } else {
        0
    };

    if already.is_none() {
        conn.execute(
            "INSERT INTO user_titles (title_id, status, anime_episodes_watched) VALUES (?1, ?2, ?3)",
            rusqlite::params![title_id, payload.status, watched]
        ).map_err(|e| e.to_string())?;
        let new_id = conn.last_insert_rowid();
        award_xp_internal(&conn, xp_delta)?;
        add_watch_minutes(&conn, anime_minutes)?;
        if payload.status == "completed" {
            conn.execute(
                "INSERT INTO watch_entries (user_title_id, watched_at, is_rewatch) VALUES (?1, datetime('now'), 0)",
                [new_id]
            ).map_err(|e| e.to_string())?;
        }
    } else {
        let user_title_id = already.unwrap();
        let old_status: String = conn.query_row("SELECT status FROM user_titles WHERE id = ?1", [user_title_id], |r| r.get(0)).map_err(|e| e.to_string())?;
        conn.execute(
            "UPDATE user_titles
             SET status = ?1,
                 anime_episodes_watched = CASE WHEN ?2 > anime_episodes_watched THEN ?2 ELSE anime_episodes_watched END,
                 updated_at = datetime('now')
             WHERE title_id = ?3",
            rusqlite::params![payload.status, watched, title_id]
        ).map_err(|e| e.to_string())?;
        if old_status != "completed" {
            award_xp_internal(&conn, xp_delta)?;
            add_watch_minutes(&conn, anime_minutes)?;
            if payload.status == "completed" {
                conn.execute(
                    "INSERT INTO watch_entries (user_title_id, watched_at, is_rewatch) VALUES (?1, datetime('now'), 0)",
                    [user_title_id]
                ).map_err(|e| e.to_string())?;
            }
        }
    }

    check_achievements_internal(&conn)
}

#[tauri::command]
pub fn get_anime_progress(db: State<Database>, mal_id: i64) -> Result<i64, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result: Option<i64> = conn.query_row(
        "SELECT ut.anime_episodes_watched
         FROM user_titles ut
         JOIN titles t ON t.id = ut.title_id
         WHERE t.tmdb_id = ?1 AND t.type = 'anime'
         LIMIT 1",
        [mal_id], |r| r.get(0)
    ).ok();
    Ok(result.unwrap_or(0))
}

#[tauri::command]
pub fn update_anime_progress(
    db: State<Database>,
    mal_id: i64,
    watched: i64,
) -> Result<Vec<Achievement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // 1. Ищем само аниме (без JOIN с user_titles)
    let (title_id, episodes_total): (i64, Option<i64>) = conn.query_row(
        "SELECT id, episodes_total FROM titles WHERE tmdb_id = ?1 AND type = 'anime' LIMIT 1",
        [mal_id], |r| Ok((r.get(0)?, r.get(1)?))
    ).map_err(|_| "Аниме не найдено в базе".to_string())?;

    // 2. Проверяем, есть ли оно в user_titles
    let old_watched: Option<i64> = conn.query_row(
        "SELECT anime_episodes_watched FROM user_titles WHERE title_id = ?1",
        [title_id], |r| r.get(0)
    ).ok();

    let old_watched = old_watched.unwrap_or(0);

    // 3. Если записи нет — создаем её
    if old_watched == 0 {
        conn.execute(
            "INSERT OR IGNORE INTO user_titles (title_id, status, anime_episodes_watched) VALUES (?1, 'watching', 0)",
            [title_id]
        ).map_err(|e| e.to_string())?;
    }

    let new_eps = (watched - old_watched).max(0);
    let xp_delta = new_eps * 2;

    conn.execute(
        "UPDATE user_titles SET anime_episodes_watched = ?1, updated_at = datetime('now') WHERE title_id = ?2",
        rusqlite::params![watched, title_id]
    ).map_err(|e| e.to_string())?;

    let mut completed_now = false;
    if let Some(total) = episodes_total {
        if total > 0 && watched >= total {
            let old_status: String = conn.query_row("SELECT status FROM user_titles WHERE title_id = ?1", [title_id], |r| r.get(0)).unwrap_or_else(|_| "planned".to_string());
            if old_status != "completed" {
                completed_now = true;
            }
            conn.execute(
                "UPDATE user_titles SET status = 'completed' WHERE title_id = ?1",
                [title_id]
            ).map_err(|e| e.to_string())?;
        } else if watched > 0 {
            conn.execute(
                "UPDATE user_titles SET status = 'watching' WHERE title_id = ?1",
                [title_id]
            ).map_err(|e| e.to_string())?;
        }
    }

    if xp_delta > 0 {
        crate::achievements::award_xp_internal(&conn, xp_delta)?;
    }
    if completed_now {
        crate::achievements::award_xp_internal(&conn, 25)?;
    }

    check_achievements_internal(&conn)
}