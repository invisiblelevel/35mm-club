use crate::db::Database;
use crate::models::*;
use crate::achievements::{award_xp_internal, add_watch_minutes, check_achievements_internal, calculate_level, xp_threshold_for_level, rank_name_for_level};
use crate::sync::log;
use tauri::State;
use std::sync::Arc;

#[tauri::command]
pub fn get_profile(db: State<'_, Arc<Database>>) -> Result<UserProfile, String> {
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
pub fn update_profile(db: State<'_, Arc<Database>>, username: Option<String>, avatar_path: Option<String>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    if let Some(name) = username { conn.execute("UPDATE user_profile SET username = ?1, updated_at = datetime('now') WHERE id = 1", [name]).map_err(|e| e.to_string())?; }
    if let Some(path) = avatar_path { conn.execute("UPDATE user_profile SET avatar_path = ?1, updated_at = datetime('now') WHERE id = 1", [path]).map_err(|e| e.to_string())?; }
    let _ = log::record_profile(&conn);
    Ok(())
}

#[tauri::command]
pub fn award_xp(db: State<'_, Arc<Database>>, amount: i64) -> Result<i64, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result = award_xp_internal(&conn, amount);
    let _ = log::record_profile(&conn);
    result
}

#[tauri::command]
pub fn reset_all_data(db: State<'_, Arc<Database>>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute_batch("DELETE FROM user_episodes; DELETE FROM episodes; DELETE FROM seasons; DELETE FROM user_showcase; DELETE FROM user_achievements; DELETE FROM user_ribbons; DELETE FROM watch_entries; DELETE FROM collection_items; DELETE FROM collections; DELETE FROM user_titles; DELETE FROM titles; UPDATE user_profile SET xp = 0, level = 1, total_watch_minutes = 0, avatar_path = NULL, updated_at = datetime('now') WHERE id = 1;").map_err(|e| e.to_string())?;
    let _ = log::record_profile(&conn);
    Ok(())
}

#[tauri::command]
pub fn get_library(db: State<'_, Arc<Database>>, content_type: String) -> Result<Vec<Title>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT t.id, t.tmdb_id, t.title, t.type, t.year, t.poster_path, ut.status, ut.rating, ut.added_at FROM titles t JOIN user_titles ut ON ut.title_id = t.id WHERE t.type = ?1 ORDER BY ut.added_at DESC").map_err(|e| e.to_string())?;
    let titles = stmt.query_map([content_type], |row| Ok(Title { id: row.get(0)?, tmdb_id: row.get(1)?, title: row.get(2)?, type_field: row.get(3)?, year: row.get(4)?, poster_path: row.get(5)?, status: row.get(6)?, rating: row.get(7)?, added_at: row.get(8)? })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    Ok(titles)
}

#[tauri::command]
pub fn search_local_library(db: State<'_, Arc<Database>>, query: String, content_type: Option<String>) -> Result<Vec<Title>, String> {
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
pub fn get_achievements(db: State<'_, Arc<Database>>) -> Result<Vec<Achievement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT a.id, a.code, a.category, a.icon, a.xp_reward, a.scope, a.tier, a.is_secret, a.title_key, a.description_key, ua.unlocked_at FROM achievements a LEFT JOIN user_achievements ua ON ua.achievement_id = a.id ORDER BY ua.unlocked_at DESC NULLS LAST, a.id ASC").map_err(|e| e.to_string())?;
    let achievements = stmt.query_map([], |row| {
        let unlocked_at: Option<String> = row.get(10)?;
        Ok(Achievement { id: row.get(0)?, code: row.get(1)?, category: row.get(2)?, icon: row.get(3)?, xp_reward: row.get(4)?, scope: row.get(5)?, tier: row.get(6)?, is_secret: row.get::<_, i64>(7)? != 0, title_key: row.get(8)?, description_key: row.get(9)?, unlocked: unlocked_at.is_some(), unlocked_at })
    }).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    Ok(achievements)
}

#[tauri::command]
pub fn get_user_title_data(db: State<'_, Arc<Database>>, tmdb_id: i64, type_field: String) -> Result<UserTitleData, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result = conn.query_row(
        "SELECT ut.id, ut.status, ut.rating, ut.notes, ut.added_at FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.tmdb_id = ?1 AND t.type = ?2 LIMIT 1",
        rusqlite::params![tmdb_id, type_field],
        |row| Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, Option<f64>>(2)?,
            row.get::<_, Option<String>>(3)?,
            row.get::<_, String>(4)?,
        ))
    ).ok();

    match result {
        Some((user_title_id, status, rating, notes, added_at)) => {
            let rewatch_count: i64 = conn.query_row(
                "SELECT COUNT(*) FROM watch_entries WHERE user_title_id = ?1 AND is_rewatch = 1",
                [user_title_id],
                |r| r.get(0),
            ).unwrap_or(0);
            Ok(UserTitleData {
                user_title_id: Some(user_title_id),
                status: Some(status),
                rating,
                notes,
                added_at: Some(added_at),
                rewatch_count,
            })
        }
        None => Ok(UserTitleData {
            user_title_id: None,
            status: None,
            rating: None,
            notes: None,
            added_at: None,
            rewatch_count: 0,
        }),
    }
}

#[tauri::command]
pub fn update_rating(db: State<'_, Arc<Database>>, user_title_id: i64, rating: Option<f64>) -> Result<Vec<Achievement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("UPDATE user_titles SET rating = ?1, updated_at = datetime('now') WHERE id = ?2", rusqlite::params![rating, user_title_id]).map_err(|e| e.to_string())?;
    let _ = log::record_user_title(&conn, user_title_id);
    let _ = log::record_profile(&conn);
    check_achievements_internal(&conn)
}

#[tauri::command]
pub fn add_to_library(db: State<'_, Arc<Database>>, payload: AddToLibraryPayload) -> Result<Vec<Achievement>, String> {
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

    let _ = log::record_title(&conn, title_id);
    let _ = log::record_title_metadata(&conn, title_id);
    let _ = log::record_user_title(&conn, user_title_id);
    let _ = log::record_profile(&conn);

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
pub fn set_avatar(app: tauri::AppHandle, db: State<'_, Arc<Database>>, image_data: String) -> Result<String, String> {
    use base64::{Engine as _, engine::general_purpose};
    use std::fs;
    use tauri::Manager;

    let bytes = general_purpose::STANDARD.decode(&image_data)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    let app_dir = app.path().app_local_data_dir().map_err(|e| e.to_string())?;
    let avatars_dir = app_dir.join("avatars");
    fs::create_dir_all(&avatars_dir).map_err(|e| e.to_string())?;

    let dest = avatars_dir.join("avatar.png");
    fs::write(&dest, &bytes).map_err(|e| format!("Write error: {}", e))?;

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE user_profile SET avatar_path = ?1, updated_at = datetime('now') WHERE id = 1",
        [dest.to_string_lossy().to_string()]
    ).map_err(|e| e.to_string())?;
    let _ = log::record_profile(&conn);

    Ok(dest.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn refresh_metadata(
    db: State<'_, Arc<Database>>,
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
    db: State<'_, Arc<Database>>,
    api_key: Option<String>,
    locale: String,
) -> Result<usize, String> {
    if locale == "zh-CN" {
        return Ok(0);
    }

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
    db: State<'_, Arc<Database>>,
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
// SHIKIMORI / ANILIST / BANGUMI (АНИМЕ)
// ==========================================

#[tauri::command]
pub async fn search_anime(query: String, locale: String) -> Result<Vec<crate::shikimori::ShikiSearchItem>, String> {
    if locale == "zh-CN" {
        return search_anime_bangumi(query).await;
    }

    match crate::shikimori::search_anime(&query, 20, &locale).await {
        Ok(mut results) if !results.is_empty() => {
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

async fn search_anime_bangumi(query: String) -> Result<Vec<crate::shikimori::ShikiSearchItem>, String> {
    let items = crate::bangumi::search_anime(&query, 20).await?;

    if items.is_empty() {
        return Ok(vec![]);
    }

    let mut set = tokio::task::JoinSet::new();
    for (idx, item) in items.iter().enumerate() {
        let title = item.title_original.clone().unwrap_or_else(|| item.title.clone());
        let bangumi_id = item.bangumi_id;
        set.spawn(async move {
            let mal_id = crate::bangumi::find_mal_id(bangumi_id, &title).await;
            (idx, mal_id)
        });
    }

    let mut mal_ids: Vec<Option<i64>> = vec![None; items.len()];
    while let Some(joined) = set.join_next().await {
        if let Ok((idx, mal_id)) = joined {
            mal_ids[idx] = mal_id;
        }
    }

    let mut mapped = Vec::new();
    for (item, mal_id_opt) in items.into_iter().zip(mal_ids.into_iter()) {
        if let Some(mal_id) = mal_id_opt {
            mapped.push(crate::shikimori::ShikiSearchItem {
                mal_id,
                title: item.title,
                title_original: item.title_original,
                description: item.description,
                cover_url: item.cover_url,
                episodes: item.episodes,
                average_score: item.average_score.map(|s| s / 10.0),
                format: item.format,
                year: item.year,
            });
        }
    }

    Ok(mapped)
}

#[tauri::command]
pub async fn get_anime_details(
    db: State<'_, Arc<Database>>,
    mal_id: i64,
    locale: String,
) -> Result<crate::shikimori::ShikiAnimeDetails, String> {
    let from_db: Option<crate::shikimori::ShikiAnimeDetails> = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        conn.query_row(
            "SELECT title, original_title, overview, poster_path, episodes_total,
                    anime_format, anime_status, studio, year
             FROM titles WHERE tmdb_id = ?1 AND type = 'anime' LIMIT 1",
            [mal_id],
            |r| {
                let title: String = r.get(0)?;
                let original_title: Option<String> = r.get(1)?;
                let overview: Option<String> = r.get(2)?;
                let poster_path: Option<String> = r.get(3)?;
                let episodes: Option<i64> = r.get(4)?;
                let format: Option<String> = r.get(5)?;
                let status: Option<String> = r.get(6)?;
                let studio: Option<String> = r.get(7)?;
                let year: Option<i64> = r.get(8)?;

                let cover_url = poster_path.map(|p| {
                    if p.starts_with("http") { p }
                    else { format!("https://image.tmdb.org/t/p/w500{}", p) }
                });

                Ok(crate::shikimori::ShikiAnimeDetails {
                    mal_id,
                    title,
                    title_original: original_title,
                    description: overview,
                    cover_url,
                    episodes,
                    duration: None,
                    average_score: None,
                    format,
                    status,
                    genres: vec![],
                    studio,
                    year,
                })
            },
        ).ok()
    };

    if let Some(details) = from_db {
        return Ok(details);
    }

    let mut details = crate::shikimori::get_anime_details(mal_id, &locale).await?;

    if locale != "ru" {
        if let Some(en_desc) = crate::anilist::get_description_by_mal_id(mal_id).await {
            details.description = Some(en_desc);
        }
    }

    Ok(details)
}

#[tauri::command]
pub async fn add_anime_to_library(
    db: State<'_, Arc<Database>>,
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

    let _ = log::record_title(&conn, title_id);
    if let Ok(ut_id) = conn.query_row(
        "SELECT id FROM user_titles WHERE title_id = ?1",
        [title_id],
        |r| r.get::<_, i64>(0),
    ) {
        let _ = log::record_user_title(&conn, ut_id);
    }
    let _ = log::record_profile(&conn);

    check_achievements_internal(&conn)
}

#[tauri::command]
pub fn get_anime_progress(db: State<'_, Arc<Database>>, mal_id: i64) -> Result<i64, String> {
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
    db: State<'_, Arc<Database>>,
    mal_id: i64,
    watched: i64,
) -> Result<Vec<Achievement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let (title_id, episodes_total): (i64, Option<i64>) = conn.query_row(
        "SELECT id, episodes_total FROM titles WHERE tmdb_id = ?1 AND type = 'anime' LIMIT 1",
        [mal_id], |r| Ok((r.get(0)?, r.get(1)?))
    ).map_err(|_| "Аниме не найдено в базе".to_string())?;

    let old_watched: Option<i64> = conn.query_row(
        "SELECT anime_episodes_watched FROM user_titles WHERE title_id = ?1",
        [title_id], |r| r.get(0)
    ).ok();

    let old_watched = old_watched.unwrap_or(0);

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

    let _ = log::record_user_title(&conn, title_id);
    let _ = log::record_profile(&conn);

    check_achievements_internal(&conn)
}

use serde_json::{json, Value};

#[tauri::command]
pub fn export_data(db: State<'_, Arc<Database>>) -> Result<String, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut data = serde_json::Map::new();

    let profile: Value = conn.query_row(
        "SELECT username, level, xp, total_watch_minutes, created_at FROM user_profile WHERE id = 1",
        [],
        |r| Ok(json!({
            "username": r.get::<_, String>(0)?,
            "level": r.get::<_, i64>(1)?,
            "xp": r.get::<_, i64>(2)?,
            "total_watch_minutes": r.get::<_, i64>(3)?,
            "created_at": r.get::<_, String>(4)?,
        }))
    ).map_err(|e| e.to_string())?;
    data.insert("profile".to_string(), profile);

    let mut stmt = conn.prepare(
        "SELECT t.tmdb_id, t.anilist_id, t.type, t.title, t.original_title, t.year, t.poster_path,
                t.overview, t.imdb_rating, t.runtime, t.language, t.episodes_total, t.anime_format,
                t.anime_status, t.studio, ut.status, ut.rating, ut.anime_episodes_watched, ut.added_at
         FROM titles t JOIN user_titles ut ON ut.title_id = t.id"
    ).map_err(|e| e.to_string())?;
    let titles: Vec<Value> = stmt.query_map([], |r| Ok(json!({
        "tmdb_id": r.get::<_, Option<i64>>(0)?,
        "anilist_id": r.get::<_, Option<i64>>(1)?,
        "type": r.get::<_, String>(2)?,
        "title": r.get::<_, String>(3)?,
        "original_title": r.get::<_, Option<String>>(4)?,
        "year": r.get::<_, Option<i64>>(5)?,
        "poster_path": r.get::<_, Option<String>>(6)?,
        "overview": r.get::<_, Option<String>>(7)?,
        "imdb_rating": r.get::<_, Option<f64>>(8)?,
        "runtime": r.get::<_, Option<i64>>(9)?,
        "language": r.get::<_, Option<String>>(10)?,
        "episodes_total": r.get::<_, Option<i64>>(11)?,
        "anime_format": r.get::<_, Option<String>>(12)?,
        "anime_status": r.get::<_, Option<String>>(13)?,
        "studio": r.get::<_, Option<String>>(14)?,
        "status": r.get::<_, String>(15)?,
        "rating": r.get::<_, Option<f64>>(16)?,
        "anime_episodes_watched": r.get::<_, i64>(17)?,
        "added_at": r.get::<_, String>(18)?,
    }))).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    data.insert("titles".to_string(), json!(titles));

    let mut stmt = conn.prepare(
        "SELECT a.code, ua.unlocked_at FROM user_achievements ua JOIN achievements a ON a.id = ua.achievement_id"
    ).map_err(|e| e.to_string())?;
    let achievements: Vec<Value> = stmt.query_map([], |r| Ok(json!({
        "code": r.get::<_, String>(0)?,
        "unlocked_at": r.get::<_, String>(1)?,
    }))).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    data.insert("achievements".to_string(), json!(achievements));

    let mut stmt = conn.prepare(
        "SELECT we.watched_at, we.rating, we.notes, we.is_rewatch, t.tmdb_id
         FROM watch_entries we
         JOIN user_titles ut ON ut.id = we.user_title_id
         JOIN titles t ON t.id = ut.title_id"
    ).map_err(|e| e.to_string())?;
    let diary: Vec<Value> = stmt.query_map([], |r| Ok(json!({
        "watched_at": r.get::<_, String>(0)?,
        "rating": r.get::<_, Option<f64>>(1)?,
        "notes": r.get::<_, Option<String>>(2)?,
        "is_rewatch": r.get::<_, i64>(3)? != 0,
        "tmdb_id": r.get::<_, Option<i64>>(4)?,
    }))).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    data.insert("diary".to_string(), json!(diary));

    let mut stmt = conn.prepare("SELECT id, name, description FROM collections").map_err(|e| e.to_string())?;
    let collections_raw: Vec<(i64, String, Option<String>)> = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))
        .map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    let mut collections: Vec<Value> = Vec::new();
    for (id, name, desc) in collections_raw {
        let mut stmt = conn.prepare(
            "SELECT t.tmdb_id FROM collection_items ci
             JOIN user_titles ut ON ut.id = ci.user_title_id
             JOIN titles t ON t.id = ut.title_id
             WHERE ci.collection_id = ?1"
        ).map_err(|e| e.to_string())?;
        let tmdb_ids: Vec<i64> = stmt.query_map([id], |r| r.get::<_, Option<i64>>(0))
            .map_err(|e| e.to_string())?
            .filter_map(|r| r.ok().flatten())
            .collect();
        collections.push(json!({
            "name": name,
            "description": desc,
            "tmdb_ids": tmdb_ids,
        }));
    }
    data.insert("collections".to_string(), json!(collections));

    let json_str = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    Ok(json_str)
}

#[tauri::command]
pub fn import_data(db: State<'_, Arc<Database>>, json_str: String) -> Result<(), String> {
    let data: Value = serde_json::from_str(&json_str).map_err(|e| format!("Invalid JSON: {}", e))?;

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute_batch(
        "DELETE FROM user_episodes;
         DELETE FROM episodes;
         DELETE FROM seasons;
         DELETE FROM user_showcase;
         DELETE FROM user_achievements;
         DELETE FROM user_ribbons;
         DELETE FROM watch_entries;
         DELETE FROM collection_items;
         DELETE FROM collections;
         DELETE FROM user_titles;
         DELETE FROM titles;
         UPDATE user_profile SET xp = 0, level = 1, total_watch_minutes = 0 WHERE id = 1;"
    ).map_err(|e| e.to_string())?;

    if let Some(profile) = data.get("profile") {
        conn.execute(
            "UPDATE user_profile SET username = ?1, xp = ?2, level = ?3, total_watch_minutes = ?4 WHERE id = 1",
            rusqlite::params![
                profile.get("username").and_then(|v| v.as_str()).unwrap_or("Киноман"),
                profile.get("xp").and_then(|v| v.as_i64()).unwrap_or(0),
                profile.get("level").and_then(|v| v.as_i64()).unwrap_or(1),
                profile.get("total_watch_minutes").and_then(|v| v.as_i64()).unwrap_or(0),
            ]
        ).map_err(|e| e.to_string())?;
    }

    let mut title_map: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
    if let Some(titles) = data.get("titles").and_then(|v| v.as_array()) {
        for t in titles {
            let tmdb_id = t.get("tmdb_id").and_then(|v| v.as_i64());
            if tmdb_id.is_none() { continue; }
            let tmdb_id = tmdb_id.unwrap();

            let existing: Option<i64> = conn.query_row(
                "SELECT id FROM titles WHERE tmdb_id = ?1 AND type = ?2",
                rusqlite::params![tmdb_id, t.get("type").and_then(|v| v.as_str()).unwrap_or("movie")],
                |r| r.get(0)
            ).ok();

            let title_id = match existing {
                Some(id) => id,
                None => {
                    conn.execute(
                        "INSERT INTO titles (tmdb_id, anilist_id, type, title, original_title, year, poster_path, overview, imdb_rating, runtime, language, episodes_total, anime_format, anime_status, studio)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                        rusqlite::params![
                            tmdb_id,
                            t.get("anilist_id").and_then(|v| v.as_i64()),
                            t.get("type").and_then(|v| v.as_str()).unwrap_or("movie"),
                            t.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                            t.get("original_title").and_then(|v| v.as_str()),
                            t.get("year").and_then(|v| v.as_i64()),
                            t.get("poster_path").and_then(|v| v.as_str()),
                            t.get("overview").and_then(|v| v.as_str()),
                            t.get("imdb_rating").and_then(|v| v.as_f64()),
                            t.get("runtime").and_then(|v| v.as_i64()),
                            t.get("language").and_then(|v| v.as_str()),
                            t.get("episodes_total").and_then(|v| v.as_i64()),
                            t.get("anime_format").and_then(|v| v.as_str()),
                            t.get("anime_status").and_then(|v| v.as_str()),
                            t.get("studio").and_then(|v| v.as_str()),
                        ]
                    ).map_err(|e| e.to_string())?;
                    conn.last_insert_rowid()
                }
            };
            title_map.insert(tmdb_id, title_id);

            conn.execute(
                "INSERT INTO user_titles (title_id, status, rating, anime_episodes_watched, added_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![
                    title_id,
                    t.get("status").and_then(|v| v.as_str()).unwrap_or("planned"),
                    t.get("rating").and_then(|v| v.as_f64()),
                    t.get("anime_episodes_watched").and_then(|v| v.as_i64()).unwrap_or(0),
                    t.get("added_at").and_then(|v| v.as_str()).unwrap_or("2026-01-01 00:00:00"),
                ]
            ).map_err(|e| e.to_string())?;
        }
    }

    if let Some(achievements) = data.get("achievements").and_then(|v| v.as_array()) {
        for a in achievements {
            if let Some(code) = a.get("code").and_then(|v| v.as_str()) {
                if let Ok(ach_id) = conn.query_row("SELECT id FROM achievements WHERE code = ?1", [code], |r| r.get::<_, i64>(0)) {
                    conn.execute(
                        "INSERT OR IGNORE INTO user_achievements (achievement_id, unlocked_at) VALUES (?1, ?2)",
                        rusqlite::params![ach_id, a.get("unlocked_at").and_then(|v| v.as_str()).unwrap_or("2026-01-01 00:00:00")]
                    ).map_err(|e| e.to_string())?;
                }
            }
        }
    }

    if let Some(diary) = data.get("diary").and_then(|v| v.as_array()) {
        for d in diary {
            if let Some(tmdb_id) = d.get("tmdb_id").and_then(|v| v.as_i64()) {
                if let Some(&title_id) = title_map.get(&tmdb_id) {
                    let ut_id: Option<i64> = conn.query_row("SELECT id FROM user_titles WHERE title_id = ?1", [title_id], |r| r.get(0)).ok();
                    if let Some(ut_id) = ut_id {
                        conn.execute(
                            "INSERT INTO watch_entries (user_title_id, watched_at, rating, notes, is_rewatch) VALUES (?1, ?2, ?3, ?4, ?5)",
                            rusqlite::params![
                                ut_id,
                                d.get("watched_at").and_then(|v| v.as_str()).unwrap_or("2026-01-01 00:00:00"),
                                d.get("rating").and_then(|v| v.as_f64()),
                                d.get("notes").and_then(|v| v.as_str()),
                                d.get("is_rewatch").and_then(|v| v.as_bool()).unwrap_or(false) as i64,
                            ]
                        ).map_err(|e| e.to_string())?;
                    }
                }
            }
        }
    }

    if let Some(collections) = data.get("collections").and_then(|v| v.as_array()) {
        for c in collections {
            if let Some(name) = c.get("name").and_then(|v| v.as_str()) {
                conn.execute(
                    "INSERT INTO collections (uuid, name, description) VALUES (lower(hex(randomblob(16))), ?1, ?2)",
                    rusqlite::params![name, c.get("description").and_then(|v| v.as_str())]
                ).map_err(|e| e.to_string())?;
                let coll_id = conn.last_insert_rowid();
                if let Some(tmdb_ids) = c.get("tmdb_ids").and_then(|v| v.as_array()) {
                    for tmdb_val in tmdb_ids {
                        if let Some(tmdb_id) = tmdb_val.as_i64() {
                            if let Some(&title_id) = title_map.get(&tmdb_id) {
                                if let Ok(ut_id) = conn.query_row("SELECT id FROM user_titles WHERE title_id = ?1", [title_id], |r| r.get::<_, i64>(0)) {
                                    conn.execute(
                                        "INSERT OR IGNORE INTO collection_items (collection_id, user_title_id) VALUES (?1, ?2)",
                                        rusqlite::params![coll_id, ut_id]
                                    ).map_err(|e| e.to_string())?;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub fn update_title_notes(db: State<'_, Arc<Database>>, user_title_id: i64, notes: Option<String>) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "UPDATE user_titles SET notes = ?1, updated_at = datetime('now') WHERE id = ?2",
        rusqlite::params![notes, user_title_id]
    ).map_err(|e| e.to_string())?;
    let _ = log::record_user_title(&conn, user_title_id);
    Ok(())
}

// ==========================================
// СТАТИСТИКА
// ==========================================

#[tauri::command]
pub fn get_stats(db: State<'_, Arc<Database>>) -> Result<crate::models::StatsFull, String> {
    use crate::models::*;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // ---------- OVERVIEW ----------
    let total_titles: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles", [], |r| r.get(0)).unwrap_or(0);
    let total_movies: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.type = 'movie'", [], |r| r.get(0)).unwrap_or(0);
    let total_tv: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.type = 'tv'", [], |r| r.get(0)).unwrap_or(0);
    let total_anime: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.type = 'anime'", [], |r| r.get(0)).unwrap_or(0);
    let total_completed: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles WHERE status = 'completed'", [], |r| r.get(0)).unwrap_or(0);
    let total_watching: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles WHERE status = 'watching'", [], |r| r.get(0)).unwrap_or(0);
    let total_planned: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles WHERE status = 'planned'", [], |r| r.get(0)).unwrap_or(0);
    let total_on_hold: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles WHERE status = 'on_hold'", [], |r| r.get(0)).unwrap_or(0);
    let total_dropped: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles WHERE status = 'dropped'", [], |r| r.get(0)).unwrap_or(0);
    let total_minutes: i64 = conn.query_row("SELECT total_watch_minutes FROM user_profile WHERE id = 1", [], |r| r.get(0)).unwrap_or(0);

    // Эпизоды: отмеченные сериалы + anime_episodes_watched
    let tv_episodes: i64 = conn.query_row(
        "SELECT COUNT(*) FROM user_episodes ue
         JOIN episodes e ON e.id = ue.episode_id
         JOIN seasons s ON s.id = e.season_id
         JOIN titles t ON t.id = s.title_id
         WHERE t.type = 'tv'", [], |r| r.get(0)
    ).unwrap_or(0);
    let anime_episodes: i64 = conn.query_row(
        "SELECT COALESCE(SUM(anime_episodes_watched), 0) FROM user_titles ut
         JOIN titles t ON t.id = ut.title_id
         WHERE t.type = 'anime'", [], |r| r.get(0)
    ).unwrap_or(0);
    let total_episodes = tv_episodes + anime_episodes;

    let total_rewatches: i64 = conn.query_row("SELECT COUNT(*) FROM watch_entries WHERE is_rewatch = 1", [], |r| r.get(0)).unwrap_or(0);

    let (average_rating, rated_count): (Option<f64>, i64) = conn.query_row(
        "SELECT AVG(rating), COUNT(*) FROM user_titles WHERE rating IS NOT NULL",
        [], |r| Ok((r.get(0)?, r.get(1)?))
    ).unwrap_or((None, 0));

    let overview = StatsOverview {
        total_titles, total_movies, total_tv, total_anime,
        total_completed, total_watching, total_planned, total_on_hold, total_dropped,
        total_minutes, total_episodes, total_rewatches, average_rating, rated_count,
    };

    // ---------- GENRES ----------
    // Жанры не хранятся в БД отдельной таблицей. Используем keyword_ids/company? Нет.
    // У нас в titles нет genre_ids. Есть только overview. Поэтому жанры достанем из titles.overview? Нет.
    // Проще: пропускаем жанры на первом этапе ИЛИ считаем по keyword? Нет.
    // Вариант: считать по type + year, а жанры убрать. Либо добавить в title_metadata поле genre_ids.
    // Пока genres = пустой массив (можно расширить позже).
    let genres: Vec<StatsGenre> = vec![];

    // ---------- YEARS ----------
    let mut stmt = conn.prepare(
        "SELECT t.year, COUNT(*) FROM titles t
         JOIN user_titles ut ON ut.title_id = t.id
         WHERE t.year IS NOT NULL
         GROUP BY t.year
         ORDER BY t.year ASC"
    ).map_err(|e| e.to_string())?;
    let years: Vec<StatsYear> = stmt.query_map([], |r| Ok(StatsYear { year: r.get(0)?, count: r.get(1)? }))
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    // ---------- DECADES ----------
    let mut stmt = conn.prepare(
        "SELECT (t.year / 10) * 10 AS decade, COUNT(*) FROM titles t
         JOIN user_titles ut ON ut.title_id = t.id
         WHERE t.year IS NOT NULL
         GROUP BY decade
         ORDER BY decade ASC"
    ).map_err(|e| e.to_string())?;
    let decades: Vec<StatsDecade> = stmt.query_map([], |r| {
        let d: i64 = r.get(0)?;
        Ok(StatsDecade { decade: format!("{}s", d), count: r.get(1)? })
    }).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    // ---------- RATINGS ----------
    let mut stmt = conn.prepare(
        "SELECT CAST(rating AS INTEGER) AS r, COUNT(*) FROM user_titles
         WHERE rating IS NOT NULL
         GROUP BY r ORDER BY r ASC"
    ).map_err(|e| e.to_string())?;
    let mut rating_map: std::collections::HashMap<i64, i64> = std::collections::HashMap::new();
    for row in stmt.query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, i64>(1)?))).map_err(|e| e.to_string())? {
        let (k, v) = row.map_err(|e| e.to_string())?;
        rating_map.insert(k, v);
    }
    let ratings: Vec<StatsRatingBucket> = (0..10).map(|i| StatsRatingBucket {
        bucket: format!("{}-{}", i, i + 1),
        count: *rating_map.get(&i).unwrap_or(&0),
    }).collect();

    // ---------- MONTHLY ----------
    let mut stmt = conn.prepare(
        "SELECT strftime('%Y-%m', watched_at) AS m, COUNT(*) FROM watch_entries
         WHERE watched_at IS NOT NULL
         GROUP BY m ORDER BY m ASC"
    ).map_err(|e| e.to_string())?;
    let monthly: Vec<StatsMonthly> = stmt.query_map([], |r| Ok(StatsMonthly { month: r.get(0)?, count: r.get(1)? }))
        .map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    // ---------- TOP RATED ----------
    let mut stmt = conn.prepare(
        "SELECT t.title, t.poster_path, t.type, t.tmdb_id, ut.rating,
                (SELECT COUNT(*) FROM watch_entries we WHERE we.user_title_id = ut.id AND we.is_rewatch = 1) AS rw
         FROM user_titles ut JOIN titles t ON t.id = ut.title_id
         WHERE ut.rating IS NOT NULL
         ORDER BY ut.rating DESC, rw DESC
         LIMIT 10"
    ).map_err(|e| e.to_string())?;
    let top_rated: Vec<StatsTopTitle> = stmt.query_map([], |r| {
        let poster_path: Option<String> = r.get(1)?;
        Ok(StatsTopTitle {
            title: r.get(0)?,
            poster_url: poster_path.map(|p| if p.starts_with("http") { p } else { format!("https://image.tmdb.org/t/p/w200{}", p) }),
            type_field: r.get(2)?,
            tmdb_id: r.get(3)?,
            rating: r.get(4)?,
            rewatch_count: r.get(5)?,
        })
    }).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    // ---------- MOST REWATCHED ----------
    let mut stmt = conn.prepare(
        "SELECT t.title, t.poster_path, t.type, t.tmdb_id, ut.rating,
                COUNT(*) AS rw
         FROM watch_entries we
         JOIN user_titles ut ON ut.id = we.user_title_id
         JOIN titles t ON t.id = ut.title_id
         WHERE we.is_rewatch = 1
         GROUP BY ut.id
         ORDER BY rw DESC
         LIMIT 10"
    ).map_err(|e| e.to_string())?;
    let most_rewatched: Vec<StatsTopTitle> = stmt.query_map([], |r| {
        let poster_path: Option<String> = r.get(1)?;
        Ok(StatsTopTitle {
            title: r.get(0)?,
            poster_url: poster_path.map(|p| if p.starts_with("http") { p } else { format!("https://image.tmdb.org/t/p/w200{}", p) }),
            type_field: r.get(2)?,
            tmdb_id: r.get(3)?,
            rating: r.get(4)?,
            rewatch_count: r.get(5)?,
        })
    }).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    // ---------- WEEKDAYS ----------
    // strftime('%w') → 0=Sunday, 1=Monday, ..., 6=Saturday
    let mut stmt = conn.prepare(
        "SELECT strftime('%w', watched_at) AS w, COUNT(*) FROM watch_entries
         WHERE watched_at IS NOT NULL
         GROUP BY w ORDER BY w ASC"
    ).map_err(|e| e.to_string())?;
    let weekdays: Vec<StatsWeekday> = stmt.query_map([], |r| {
        Ok(StatsWeekday { weekday: r.get(0)?, count: r.get(1)? })
    }).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
	
    Ok(StatsFull { overview, genres, years, decades, ratings, monthly, weekdays, top_rated, most_rewatched })
}