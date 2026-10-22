use rusqlite::Connection;
use crate::models::Achievement;

pub fn xp_threshold_for_level(level: i32) -> i64 {
    let l = level as i64;
    100 * l + 50 * l * l
}

pub fn calculate_level(total_xp: i64) -> i32 {
    let mut level = 1;
    loop {
        let next_threshold = xp_threshold_for_level(level);
        if total_xp < next_threshold { return level; }
        level += 1;
        if level > 1000 { return level; }
    }
}

pub fn rank_name_for_level(level: i32) -> &'static str {
    match level {
        1..=2 => "ranks.private",
        3..=5 => "ranks.corporal",
        6..=8 => "ranks.sergeant",
        9..=11 => "ranks.starshina",
        12..=14 => "ranks.lieutenant",
        15..=19 => "ranks.captain",
        20..=26 => "ranks.major",
        27..=33 => "ranks.colonel",
        34..=45 => "ranks.full_colonel",
        _ => "ranks.general",
    }
}

pub fn award_xp_internal(conn: &Connection, amount: i64) -> Result<i64, String> {
    if amount == 0 { return Ok(0); }
    conn.execute("UPDATE user_profile SET xp = xp + ?1, updated_at = datetime('now') WHERE id = 1", [amount]).map_err(|e| e.to_string())?;
    let new_xp: i64 = conn.query_row("SELECT xp FROM user_profile WHERE id = 1", [], |row| row.get(0)).map_err(|e| e.to_string())?;
    let new_level = calculate_level(new_xp);
    conn.execute("UPDATE user_profile SET level = ?1 WHERE id = 1", [new_level]).map_err(|e| e.to_string())?;
    Ok(new_xp)
}

pub fn add_watch_minutes(conn: &Connection, minutes: i64) -> Result<(), String> {
    if minutes <= 0 { return Ok(()); }
    conn.execute("UPDATE user_profile SET total_watch_minutes = total_watch_minutes + ?1, updated_at = datetime('now') WHERE id = 1", [minutes]).map_err(|e| e.to_string())?;
    Ok(())
}

fn count_person_in_library(conn: &Connection, person_id: i64, field: &str) -> Result<i64, String> {
    match field {
        "cast_ids" | "director_ids" | "producer_ids" => {}
        _ => return Ok(0),
    }
    let sql = format!("SELECT DISTINCT tm.title_id, tm.{} FROM title_metadata tm JOIN user_titles ut ON ut.title_id = tm.title_id WHERE tm.{} IS NOT NULL AND ut.status = 'completed'", field, field);
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows: Vec<(i64, String)> = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?))).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    let mut count = 0;
    for (_title_id, json_str) in rows {
        if let Ok(ids) = serde_json::from_str::<Vec<i64>>(&json_str) {
            if ids.contains(&person_id) { count += 1; }
        }
    }
    Ok(count)
}

fn count_id_in_json_field(conn: &Connection, target_id: i64, field: &str) -> Result<i64, String> {
    match field {
        "keyword_ids" | "production_company_ids" => {}
        _ => return Ok(0),
    }
    let sql = format!("SELECT tm.{} FROM title_metadata tm JOIN user_titles ut ON ut.title_id = tm.title_id WHERE tm.{} IS NOT NULL AND ut.status = 'completed'", field, field);
    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let rows: Vec<String> = stmt.query_map([], |row| row.get(0)).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;
    let mut count = 0;
    for json_str in rows {
        if let Ok(ids) = serde_json::from_str::<Vec<i64>>(&json_str) {
            if ids.contains(&target_id) { count += 1; }
        }
    }
    Ok(count)
}

pub fn check_achievements_internal(conn: &Connection) -> Result<Vec<Achievement>, String> {
    let mut newly_unlocked = Vec::new();
    let total_count: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles", [], |row| row.get(0)).unwrap_or(0);
    let movies_count: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.type = 'movie'", [], |row| row.get(0)).unwrap_or(0);
    let tv_count: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.type = 'tv'", [], |row| row.get(0)).unwrap_or(0);
    let anime_count: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.type = 'anime'", [], |row| row.get(0)).unwrap_or(0);
	let status_planned: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles WHERE status = 'planned'", [], |row| row.get(0)).unwrap_or(0);
    let status_on_hold: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles WHERE status = 'on_hold'", [], |row| row.get(0)).unwrap_or(0);
    let status_dropped: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles WHERE status = 'dropped'", [], |row| row.get(0)).unwrap_or(0);

    let mut stmt = conn.prepare(
        "SELECT a.id, a.code, a.category, a.icon, a.xp_reward, a.scope, a.tier, a.is_secret, a.title_key, a.description_key, a.condition_type, a.condition_value, a.required_count
         FROM achievements a LEFT JOIN user_achievements ua ON ua.achievement_id = a.id WHERE ua.achievement_id IS NULL"
    ).map_err(|e| e.to_string())?;

    #[allow(clippy::type_complexity)]
    let pending: Vec<(i64, String, String, Option<String>, i64, String, String, i64, Option<String>, Option<String>, String, Option<i64>, i64)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?, row.get(9)?, row.get(10)?, row.get::<_, Option<i64>>(11)?, row.get::<_, i64>(12)?)))
        .map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?;

    for (id, code, category, icon, xp_reward, scope, tier, is_secret, title_key, description_key, cond_type, cond_value_opt, required) in pending {
        let cond_value = cond_value_opt.unwrap_or(0);
        let is_counter = matches!(cond_type.as_str(), "total_count" | "movies_count" | "tv_count" | "anime_count" | "status_planned" | "status_on_hold" | "status_dropped" | "tv_seasons_completed" | "anime_episodes_watched" | "rewatch_count" | "rewatch_same_title");
        let no_id_type = matches!(cond_type.as_str(), "ratings_1" | "night_watch" | "user_rating_with_low_tmdb" | "daily_completed" | "tv_episodes_completed" | "anime_episodes_completed" | "anime_franchise_completed");
        if cond_value == 0 && !is_counter && !no_id_type { continue; }

        let satisfied = match cond_type.as_str() {
            "total_count" => total_count >= cond_value,
            "movies_count" => movies_count >= cond_value,
            "tv_count" => tv_count >= cond_value,
            "anime_count" => anime_count >= cond_value,
			"status_planned" => status_planned >= cond_value,
            "status_on_hold" => status_on_hold >= cond_value,
            "status_dropped" => status_dropped >= cond_value,
            "movie_id" => { let c: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.tmdb_id = ?1 AND ut.status = 'completed'", [cond_value], |r| r.get(0)).unwrap_or(0); c > 0 }
            "cast_contains" => count_person_in_library(conn, cond_value, "cast_ids")? >= required,
            "director_contains" => count_person_in_library(conn, cond_value, "director_ids")? >= required,
            "cast_or_producer_contains" => { let a = count_person_in_library(conn, cond_value, "cast_ids")?; let b = count_person_in_library(conn, cond_value, "producer_ids")?; (a + b) >= required }
            "runtime_gt" => { let c: i64 = conn.query_row("SELECT COUNT(*) FROM titles t JOIN user_titles ut ON ut.title_id = t.id WHERE t.runtime >= ?1 AND ut.status = 'completed'", [cond_value], |r| r.get(0)).unwrap_or(0); c > 0 }
            "runtime_lt" => { let c: i64 = conn.query_row("SELECT COUNT(*) FROM titles t JOIN user_titles ut ON ut.title_id = t.id WHERE t.runtime <= ?1 AND ut.status = 'completed'", [cond_value], |r| r.get(0)).unwrap_or(0); c > 0 }
            "year_lt" => { let c: i64 = conn.query_row("SELECT COUNT(*) FROM titles t JOIN user_titles ut ON ut.title_id = t.id WHERE t.year < ?1 AND ut.status = 'completed'", [cond_value], |r| r.get(0)).unwrap_or(0); c > 0 }
            "collection_id" => { let c: i64 = conn.query_row("SELECT COUNT(DISTINCT ut.title_id) FROM title_metadata tm JOIN user_titles ut ON ut.title_id = tm.title_id WHERE tm.collection_id = ?1 AND ut.status = 'completed'", [cond_value], |r| r.get(0)).unwrap_or(0); c >= required }
            "keyword_or_company" => { let k = count_id_in_json_field(conn, cond_value, "keyword_ids")?; let c = count_id_in_json_field(conn, 420, "production_company_ids")?; k.max(c) >= required }
            "user_rating_with_low_tmdb" => { let c: i64 = conn.query_row("SELECT COUNT(*) FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE ut.rating >= 8.0 AND t.imdb_rating < 4.0", [], |r| r.get(0)).unwrap_or(0); c >= 1 }
            "ratings_1" => { let mut s2 = conn.prepare("SELECT rating FROM user_titles WHERE rating IS NOT NULL ORDER BY updated_at DESC LIMIT ?1").map_err(|e| e.to_string())?; let r: Vec<f64> = s2.query_map([required], |r| r.get(0)).map_err(|e| e.to_string())?.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())?; r.len() as i64 >= required && r.iter().all(|v| (*v - 1.0).abs() < 0.001) }
            "daily_completed" => {
                let c: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM (
                        SELECT date(we.watched_at) AS d, COUNT(DISTINCT ut.title_id) AS n
                        FROM watch_entries we
                        JOIN user_titles ut ON ut.id = we.user_title_id
                        JOIN titles t ON t.id = ut.title_id
                        WHERE t.type = 'movie' AND we.is_rewatch = 0
                        GROUP BY date(we.watched_at)
                        HAVING n >= ?1
                    )",
                    [cond_value], |r| r.get(0)
                ).unwrap_or(0);
                c > 0
            }
            "night_watch" => { let c: i64 = conn.query_row("SELECT COUNT(*) FROM watch_entries WHERE CAST(strftime('%H', watched_at) AS INTEGER) >= 2 AND CAST(strftime('%H', watched_at) AS INTEGER) <= 5", [], |r| r.get(0)).unwrap_or(0); c >= 1 }
            "tv_episodes_completed" => {
                let total: i64 = conn.query_row("SELECT COUNT(*) FROM episodes e JOIN seasons s ON s.id = e.season_id JOIN titles t ON t.id = s.title_id WHERE t.tmdb_id = ?1 AND t.type = 'tv'", [cond_value], |r| r.get(0)).unwrap_or(0);
                let watched: i64 = conn.query_row("SELECT COUNT(*) FROM user_episodes ue JOIN episodes e ON e.id = ue.episode_id JOIN seasons s ON s.id = e.season_id JOIN titles t ON t.id = s.title_id WHERE t.tmdb_id = ?1 AND t.type = 'tv'", [cond_value], |r| r.get(0)).unwrap_or(0);
                total > 0 && watched == total
            }
            "tv_seasons_completed" => {
                // Считаем сезоны, в которых ВСЕ эпизоды отмечены.
                // Работает для долгоиграющих сериалов (Симпсоны, South Park, Criminal Minds),
                // которые ещё выходят: ачивка выдаётся за N полностью просмотренных сезонов,
                // а не за весь сериал целиком.
                let completed_seasons: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM (
                        SELECT s.id
                        FROM seasons s
                        JOIN titles t ON t.id = s.title_id
                        WHERE t.tmdb_id = ?1 AND t.type = 'tv'
                        GROUP BY s.id
                        HAVING (SELECT COUNT(*) FROM episodes e WHERE e.season_id = s.id) > 0
                           AND (SELECT COUNT(*) FROM episodes e WHERE e.season_id = s.id) =
                               (SELECT COUNT(*) FROM user_episodes ue JOIN episodes e ON e.id = ue.episode_id WHERE e.season_id = s.id)
                    )",
                    [cond_value],
                    |r| r.get(0),
                ).unwrap_or(0);
                completed_seasons >= required
            },
            "anime_episodes_completed" => {
                let row: Option<(i64, Option<i64>)> = conn.query_row("SELECT ut.anime_episodes_watched, t.episodes_total FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.tmdb_id = ?1 AND t.type = 'anime' LIMIT 1", [cond_value], |r| Ok((r.get(0)?, r.get(1)?))).ok();
                if let Some((watched, total_opt)) = row { if let Some(total) = total_opt { total > 0 && watched >= total } else { false } } else { false }
            }
            "anime_episodes_watched" => {
                // Для долгоиграющих аниме (One Piece, Detective Conan, Naruto):
                // ачивка за N просмотренных серий, а не за весь тайтл целиком.
                let watched: i64 = conn.query_row(
                    "SELECT ut.anime_episodes_watched FROM user_titles ut
                     JOIN titles t ON t.id = ut.title_id
                     WHERE t.tmdb_id = ?1 AND t.type = 'anime' LIMIT 1",
                    [cond_value],
                    |r| r.get(0),
                ).unwrap_or(0);
                watched >= required
            },
            "anime_franchise_completed" => {
                let mal_ids: Vec<i64> = match code.as_str() {
                    "anime_konosuba_party" => vec![30831, 32937, 49458],
                    "anime_prison_school" => vec![30240, 32485],
                    "anime_black_lagoon" => vec![889, 1519],
                    "anime_naruto_legend" => vec![20, 1735],
                    "anime_made_in_abyss" => vec![34599, 41084],
                    "anime_mushoku_tensei" => vec![39535, 45576],
					"anime_ghibli_miyazaki" => vec![199, 431, 164, 523],
					"anime_rezero" => vec![31240, 39587, 42203, 54857, 61316],
                    "anime_slime" => vec![37430, 39551, 41487, 53580, 59970],
                    "anime_shield_hero" => vec![35790, 40356, 40357, 57907],
                    "anime_realist_hero" => vec![41710, 49930],
                    "anime_emminence_shadow" => vec![48316, 54595],
                    "anime_slime_300" => vec![40586, 50738],
                    "anime_campfire_cooking" => vec![53446, 57025],
                    "anime_k_on" => vec![5680, 7791],
                    "anime_clannad" => vec![2167, 4181],
                    "anime_umaru" => vec![28825, 35376],
                    "anime_rent_a_girlfriend" => vec![40839, 42963, 53050, 59277, 62485],
                    "anime_kaguya" => vec![37999, 40591, 43608],
                    "anime_tokyo_ghoul" => vec![22319, 27899, 36511, 37799],
                    "anime_psycho_pass" => vec![13601, 23281, 39491],
                    "anime_terra_formars" => vec![22687, 31430],
					"anime_highschool_dxd" => vec![11617, 15451, 24703, 34281],
                    "anime_to_love_ru" => vec![3455, 9181, 13663, 28979],
                    "anime_demon_lord" => vec![37210, 41623],
                    "anime_uzaki" => vec![41226, 42962],
                    "anime_testament" => vec![23233, 30363],
                    _ => vec![],
                };
                if mal_ids.is_empty() { false }
                else {
                    let mut all_completed = true;
                    for mal_id in mal_ids {
                        let row: Option<(i64, Option<i64>)> = conn.query_row("SELECT ut.anime_episodes_watched, t.episodes_total FROM user_titles ut JOIN titles t ON t.id = ut.title_id WHERE t.tmdb_id = ?1 AND t.type = 'anime' LIMIT 1", [mal_id], |r| Ok((r.get(0)?, r.get(1)?))).ok();
                        match row { Some((watched, Some(total))) if total > 0 && watched >= total => {}, _ => { all_completed = false; break; } }
                    }
                    all_completed
                }
            },
            "rewatch_count" => {
                let c: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM watch_entries WHERE is_rewatch = 1",
                    [],
                    |r| r.get(0),
                ).unwrap_or(0);
                c >= required
            }
            "rewatch_same_title" => {
                let c: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM (
                        SELECT user_title_id FROM watch_entries
                        WHERE is_rewatch = 1
                        GROUP BY user_title_id
                        HAVING COUNT(*) >= ?1
                    )",
                    [required],
                    |r| r.get(0),
                ).unwrap_or(0);
                c > 0
            }
            _ => false,
        };

        if satisfied {
            conn.execute("INSERT INTO user_achievements (achievement_id) VALUES (?1)", [id]).map_err(|e| e.to_string())?;
			let _ = crate::sync::log::record_user_achievement(conn, id);
            award_xp_internal(conn, xp_reward)?;
            newly_unlocked.push(Achievement { id, code, category, icon, xp_reward, scope, tier, is_secret: is_secret != 0, title_key, description_key, unlocked: true, unlocked_at: None });
        }
    }
    Ok(newly_unlocked)
}