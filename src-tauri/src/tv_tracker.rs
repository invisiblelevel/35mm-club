use crate::db::Database;
use crate::models::EpisodeWithStatus;
use tauri::State;

#[tauri::command]
pub async fn get_tv_seasons(
    db: State<'_, Database>, tmdb_id: i64, api_key: Option<String>, locale: String,
) -> Result<Vec<crate::tmdb::TvSeason>, String> {
    let cached: Option<Vec<crate::tmdb::TvSeason>> = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let title_id_opt: Option<i64> = conn.query_row(
            "SELECT id FROM titles WHERE tmdb_id = ?1 AND type = 'tv' LIMIT 1",
            [tmdb_id], |r| r.get(0)
        ).ok();

        match title_id_opt {
            Some(title_id) => {
                let count: i64 = conn.query_row("SELECT COUNT(*) FROM seasons WHERE title_id = ?1", [title_id], |r| r.get(0)).unwrap_or(0);
                if count > 0 {
                    let mut stmt = conn.prepare(
                        "SELECT s.tmdb_season_number, s.name, s.episode_count, s.poster_path,
                         (SELECT COUNT(*) FROM user_episodes ue JOIN episodes e ON e.id = ue.episode_id WHERE e.season_id = s.id) as watched_count
                         FROM seasons s WHERE s.title_id = ?1 ORDER BY s.tmdb_season_number ASC"
                    ).map_err(|e| e.to_string())?;
                    
                    let seasons = stmt.query_map([title_id], |row| Ok(crate::tmdb::TvSeason {
                        season_number: row.get(0)?,
                        name: row.get(1)?,
                        episode_count: row.get(2)?,
                        poster_url: row.get(3)?,
                        watched_count: row.get(4)?,
                    })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
                    Some(seasons)
                } else {
                    None
                }
            }
            None => return Err("Тайтл не найден в библиотеке".to_string()),
        }
    };

    if let Some(seasons) = cached {
        return Ok(seasons);
    }

    let mut api_seasons = crate::tmdb::get_tv_seasons(tmdb_id, api_key, &locale).await?;

    {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let title_id: i64 = conn.query_row(
            "SELECT id FROM titles WHERE tmdb_id = ?1 AND type = 'tv' LIMIT 1",
            [tmdb_id], |r| r.get(0)
        ).map_err(|_| "Тайтл не найден".to_string())?;

        for s in &api_seasons {
            conn.execute(
                "INSERT INTO seasons (title_id, tmdb_season_number, name, episode_count, poster_path) VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(title_id, tmdb_season_number) DO UPDATE SET name = excluded.name, episode_count = excluded.episode_count, poster_path = excluded.poster_path",
                rusqlite::params![title_id, s.season_number, s.name, s.episode_count, s.poster_url]
            ).map_err(|e| e.to_string())?;
        }
    }

    for s in &mut api_seasons {
        s.watched_count = 0;
    }

    Ok(api_seasons)
}

#[tauri::command]
pub async fn get_season_episodes(
    db: State<'_, Database>, tmdb_id: i64, season_number: i64, api_key: Option<String>, locale: String,
) -> Result<Vec<EpisodeWithStatus>, String> {
    let (season_id, cached): (i64, Option<Vec<EpisodeWithStatus>>) = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let title_id: i64 = conn.query_row("SELECT id FROM titles WHERE tmdb_id = ?1 AND type = 'tv' LIMIT 1", [tmdb_id], |r| r.get(0)).map_err(|_| "Тайтл не найден".to_string())?;
        let season_id: i64 = conn.query_row("SELECT id FROM seasons WHERE title_id = ?1 AND tmdb_season_number = ?2 LIMIT 1", rusqlite::params![title_id, season_number], |r| r.get(0)).map_err(|_| "Сезон не найден".to_string())?;
        let ep_count: i64 = conn.query_row("SELECT COUNT(*) FROM episodes WHERE season_id = ?1", [season_id], |r| r.get(0)).unwrap_or(0);

        if ep_count > 0 {
            let mut stmt = conn.prepare(
                "SELECT e.id, e.tmdb_episode_number, e.name, e.overview, e.runtime, e.air_date, e.still_path,
                        CASE WHEN ue.episode_id IS NOT NULL THEN 1 ELSE 0 END as watched
                 FROM episodes e LEFT JOIN user_episodes ue ON ue.episode_id = e.id
                 WHERE e.season_id = ?1 ORDER BY e.tmdb_episode_number ASC"
            ).map_err(|e| e.to_string())?;
            let eps = stmt.query_map([season_id], |row| Ok(EpisodeWithStatus {
                id: row.get(0)?, episode_number: row.get(1)?, name: row.get(2)?,
                overview: row.get(3)?, runtime: row.get(4)?, air_date: row.get(5)?,
                still_url: row.get(6)?, watched: row.get::<_, i64>(7)? != 0,
            })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
            (season_id, Some(eps))
        } else {
            (season_id, None)
        }
    };

    if let Some(eps) = cached { return Ok(eps); }

    let api_eps = crate::tmdb::get_season_details(tmdb_id, season_number, api_key, &locale).await?;

    {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        for e in &api_eps {
            conn.execute(
                "INSERT INTO episodes (season_id, tmdb_episode_number, name, overview, runtime, air_date, still_path)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                 ON CONFLICT(season_id, tmdb_episode_number) DO UPDATE SET name = excluded.name, overview = excluded.overview, runtime = excluded.runtime, air_date = excluded.air_date, still_path = excluded.still_path",
                rusqlite::params![season_id, e.episode_number, e.name, e.overview, e.runtime, e.air_date, e.still_url]
            ).map_err(|e| e.to_string())?;
        }
    }

    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare(
        "SELECT e.id, e.tmdb_episode_number, e.name, e.overview, e.runtime, e.air_date, e.still_path,
                CASE WHEN ue.episode_id IS NOT NULL THEN 1 ELSE 0 END as watched
         FROM episodes e LEFT JOIN user_episodes ue ON ue.episode_id = e.id
         WHERE e.season_id = ?1 ORDER BY e.tmdb_episode_number ASC"
    ).map_err(|e| e.to_string())?;
    let eps = stmt.query_map([season_id], |row| Ok(EpisodeWithStatus {
        id: row.get(0)?, episode_number: row.get(1)?, name: row.get(2)?,
        overview: row.get(3)?, runtime: row.get(4)?, air_date: row.get(5)?,
        still_url: row.get(6)?, watched: row.get::<_, i64>(7)? != 0,
    })).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    Ok(eps)
}

#[tauri::command]
pub fn mark_episode_watched(db: State<Database>, episode_id: i64, watched: bool) -> Result<Vec<crate::models::Achievement>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // XP за эпизод: +2 при отметке, 0 при снятии
    let xp_delta = if watched { 2 } else { 0 };

    if watched {
        conn.execute("INSERT OR IGNORE INTO user_episodes (episode_id) VALUES (?1)", [episode_id]).map_err(|e| e.to_string())?;
    } else {
        conn.execute("DELETE FROM user_episodes WHERE episode_id = ?1", [episode_id]).map_err(|e| e.to_string())?;
    }

    let season_id: i64 = conn.query_row("SELECT season_id FROM episodes WHERE id = ?1", [episode_id], |r| r.get(0)).map_err(|e| e.to_string())?;
    let title_id: i64 = conn.query_row("SELECT title_id FROM seasons WHERE id = ?1", [season_id], |r| r.get(0)).map_err(|e| e.to_string())?;
    let user_title_id: Option<i64> = conn.query_row("SELECT id FROM user_titles WHERE title_id = ?1", [title_id], |r| r.get(0)).ok();

    if let Some(ut_id) = user_title_id {
        let total_eps: i64 = conn.query_row("SELECT COUNT(*) FROM episodes e JOIN seasons s ON s.id = e.season_id WHERE s.title_id = ?1", [title_id], |r| r.get(0)).unwrap_or(0);
        let watched_eps: i64 = conn.query_row("SELECT COUNT(*) FROM user_episodes ue JOIN episodes e ON e.id = ue.episode_id JOIN seasons s ON s.id = e.season_id WHERE s.title_id = ?1", [title_id], |r| r.get(0)).unwrap_or(0);

        if total_eps > 0 && watched_eps == total_eps {
            conn.execute("UPDATE user_titles SET status = 'completed', updated_at = datetime('now') WHERE id = ?1", [ut_id]).map_err(|e| e.to_string())?;
        } else if watched_eps > 0 {
            conn.execute("UPDATE user_titles SET status = 'watching', updated_at = datetime('now') WHERE id = ?1", [ut_id]).map_err(|e| e.to_string())?;
        } else {
            conn.execute("UPDATE user_titles SET status = 'planned', updated_at = datetime('now') WHERE id = ?1", [ut_id]).map_err(|e| e.to_string())?;
        }
    }

    // Начисляем XP за эпизод
    if xp_delta > 0 {
        crate::achievements::award_xp_internal(&conn, xp_delta)?;
    }

    // Проверяем ачивки
    crate::achievements::check_achievements_internal(&conn)
}

pub async fn mark_all_episodes_watched_internal(
    db: &Database,
    tmdb_id: i64,
    api_key: Option<String>,
    locale: String,
) -> Result<Vec<crate::models::Achievement>, String> {
    let title_id: i64 = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        conn.query_row(
            "SELECT id FROM titles WHERE tmdb_id = ?1 AND type = 'tv' LIMIT 1",
            [tmdb_id], |r| r.get(0)
        ).map_err(|_| "Тайтл не найден".to_string())?
    };

    let seasons_count: i64 = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        conn.query_row("SELECT COUNT(*) FROM seasons WHERE title_id = ?1", [title_id], |r| r.get(0)).unwrap_or(0)
    };

    if seasons_count == 0 {
        let api_seasons = crate::tmdb::get_tv_seasons(tmdb_id, api_key.clone(), &locale).await?;
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        for s in &api_seasons {
            conn.execute(
                "INSERT INTO seasons (title_id, tmdb_season_number, name, episode_count, poster_path) VALUES (?1, ?2, ?3, ?4, ?5)
                 ON CONFLICT(title_id, tmdb_season_number) DO UPDATE SET name = excluded.name, episode_count = excluded.episode_count, poster_path = excluded.poster_path",
                rusqlite::params![title_id, s.season_number, s.name, s.episode_count, s.poster_url]
            ).map_err(|e| e.to_string())?;
        }
    }

    let season_numbers: Vec<i64> = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        let mut stmt = conn.prepare("SELECT tmdb_season_number FROM seasons WHERE title_id = ?1").map_err(|e| e.to_string())?;
        stmt.query_map([title_id], |r| r.get(0)).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?
    };

    for season_number in season_numbers {
        let season_id: i64 = {
            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            conn.query_row(
                "SELECT id FROM seasons WHERE title_id = ?1 AND tmdb_season_number = ?2 LIMIT 1",
                rusqlite::params![title_id, season_number], |r| r.get(0)
            ).map_err(|e| e.to_string())?
        };

        let ep_count: i64 = {
            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            conn.query_row("SELECT COUNT(*) FROM episodes WHERE season_id = ?1", [season_id], |r| r.get(0)).unwrap_or(0)
        };

        if ep_count == 0 {
            let api_eps = crate::tmdb::get_season_details(tmdb_id, season_number, api_key.clone(), &locale).await?;
            let conn = db.conn.lock().map_err(|e| e.to_string())?;
            for e in &api_eps {
                conn.execute(
                    "INSERT INTO episodes (season_id, tmdb_episode_number, name, overview, runtime, air_date, still_path)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
                     ON CONFLICT(season_id, tmdb_episode_number) DO UPDATE SET name = excluded.name, overview = excluded.overview, runtime = excluded.runtime, air_date = excluded.air_date, still_path = excluded.still_path",
                    rusqlite::params![season_id, e.episode_number, e.name, e.overview, e.runtime, e.air_date, e.still_url]
                ).map_err(|e| e.to_string())?;
            }
        }
    }

    let unlocked = {
        let conn = db.conn.lock().map_err(|e| e.to_string())?;
        
        // Считаем, сколько новых эпизодов будет отмечено (для XP)
        let already_watched: i64 = conn.query_row(
            "SELECT COUNT(*) FROM user_episodes ue JOIN episodes e ON e.id = ue.episode_id JOIN seasons s ON s.id = e.season_id WHERE s.title_id = ?1",
            [title_id], |r| r.get(0)
        ).unwrap_or(0);
        
        let total_eps: i64 = conn.query_row(
            "SELECT COUNT(*) FROM episodes e JOIN seasons s ON s.id = e.season_id WHERE s.title_id = ?1",
            [title_id], |r| r.get(0)
        ).unwrap_or(0);
        
        let new_eps = total_eps - already_watched;
        let xp_gained = new_eps * 2;

        conn.execute(
            "INSERT OR IGNORE INTO user_episodes (episode_id)
             SELECT e.id FROM episodes e
             JOIN seasons s ON s.id = e.season_id
             WHERE s.title_id = ?1",
            [title_id],
        ).map_err(|e| e.to_string())?;
        
        // Начисляем XP за новые эпизоды
        if xp_gained > 0 {
            crate::achievements::award_xp_internal(&conn, xp_gained)?;
        }
        
        crate::achievements::check_achievements_internal(&conn)?
    };

    Ok(unlocked)
}

#[tauri::command]
pub async fn mark_all_episodes_watched(
    db: State<'_, Database>,
    tmdb_id: i64,
    api_key: Option<String>,
    locale: String,
) -> Result<Vec<crate::models::Achievement>, String> {
    mark_all_episodes_watched_internal(&db, tmdb_id, api_key, locale).await
}