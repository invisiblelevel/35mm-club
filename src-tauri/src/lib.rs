mod commands;
mod db;
mod tmdb;
mod models;
mod achievements;
mod tv_tracker;
mod diary_collections;
mod anilist;
mod shikimori;

use db::Database;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn apply_pending_migrations(conn: &rusqlite::Connection) -> Result<(), String> {
    conn.execute_batch(include_str!("../migrations/001_initial.sql")).map_err(|e| format!("001: {}", e))?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let app_dir = app.path().app_data_dir().expect("failed to get app data dir");
            std::fs::create_dir_all(&app_dir).expect("failed to create app data dir");
            let db_path = app_dir.join("35mm_club.db");
            let database = Database::new(db_path.to_str().unwrap()).expect("failed to open database");
            {
                let conn = database.conn.lock().expect("failed to lock db");
                apply_pending_migrations(&conn).expect("failed to apply pending migrations");
            }
            app.manage(database);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            commands::get_profile,
            commands::update_profile,
            commands::award_xp,
            commands::reset_all_data,
            commands::get_achievements,
            commands::get_library,
            commands::search_local_library,
            commands::add_to_library,
            commands::search_movies,
            commands::get_title_details,
            commands::get_user_title_data,
            commands::update_rating,
            commands::set_avatar,
            commands::refresh_metadata,
            commands::refresh_titles_language,
            commands::ensure_title_language,
            commands::search_anime,
            commands::get_anime_details,
            commands::add_anime_to_library,
            commands::get_anime_progress,
            commands::update_anime_progress,
            tv_tracker::get_tv_seasons,
            tv_tracker::get_season_episodes,
            tv_tracker::mark_episode_watched,
            tv_tracker::mark_all_episodes_watched,
            diary_collections::get_diary,
            diary_collections::get_collections,
            diary_collections::create_collection,
            diary_collections::delete_collection,
            diary_collections::get_collection_items,
            diary_collections::add_to_collection,
            diary_collections::remove_from_collection,
            diary_collections::get_user_collections_for_title
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}