mod achievements;
mod anilist;
mod commands;
mod db;
mod diary_collections;
mod models;
mod shikimori;
mod sync;
mod tmdb;
mod tv_tracker;

use db::Database;
use std::sync::Arc;
use sync::commands::SyncState;
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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

            let database = Database::new(db_path.to_str().unwrap())
                .expect("failed to open database");

            // Миграции
            {
                let conn = database.conn.lock().expect("failed to lock db");
                db::apply_migrations(&conn).expect("failed to apply migrations");
            }

            // Инициализация identity (генерация ключа при первом запуске)
            let identity = {
                let conn = database.conn.lock().expect("failed to lock db");
                sync::identity::init_identity(&app_dir, &conn)
                    .expect("failed to init identity")
            };

            let database = Arc::new(database);
            app.manage(database.clone());

            // SyncState + listener
            let sync_state = SyncState::new(0); // порт узнаем после bind
            app.manage(sync_state);

            // Запускаем listener и mDNS в фоне
            let app_handle = app.handle().clone();
            let db_clone = database.clone();
            let identity_clone = identity.clone();
            let app_dir_clone = app_dir.clone();

            tauri::async_runtime::spawn(async move {
                // 1. Находим свободный порт
                let port = match sync::discovery::find_free_port() {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("[sync] find_free_port: {}", e);
                        return;
                    }
                };

                // 2. Обновляем порт в SyncState
                // (SyncState::new(0) → перезаписываем)
                // Проще: сделать SyncState.port через Arc<AtomicU16>, но здесь
                // просто запустим listener на найденном порту, а state будет знать.
                // Так как SyncState хранит u16, а он уже создан — пересоздаём.
                // Хак: используем listener_port как есть — пересоздать нельзя.
                // Поэтому: используем порт из state, но state создали с 0...
                //
                // РЕШЕНИЕ: SyncState.listener_port — не нужен после старта.
                // Используем локальную переменную port.

                // 3. Запускаем TCP listener
                match sync::transport::start_listener(
                    db_clone.clone(),
                    identity_clone.device_id.clone(),
                    port,
                ).await {
                    Ok(actual_port) => {
                        println!("[sync] listener on port {}", actual_port);

                        // 4. Регистрируем mDNS
                        match sync::discovery::register_service(
                            &identity_clone.device_id,
                            &identity_clone.device_name,
                            actual_port,
                        ).await {
                            Ok(mdns) => {
                                let state = app_handle.state::<SyncState>();
                                let mut guard = state.mdns.lock().await;
                                *guard = Some(mdns);
                                println!("[sync] mDNS registered");
                            }
                            Err(e) => eprintln!("[sync] mDNS register: {}", e),
                        }
                    }
                    Err(e) => eprintln!("[sync] listener start: {}", e),
                }

                // 5. Слушаем событие закрытия приложения
                // (опционально, позже)
                let _ = (&app_handle, &app_dir_clone);
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // Существующие команды
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
            commands::update_title_notes,
            commands::set_avatar,
            commands::refresh_metadata,
            commands::refresh_titles_language,
            commands::ensure_title_language,
            commands::search_anime,
            commands::get_anime_details,
            commands::add_anime_to_library,
            commands::get_anime_progress,
            commands::update_anime_progress,
            commands::export_data,
            commands::import_data,
            tv_tracker::get_tv_seasons,
            tv_tracker::get_season_episodes,
            tv_tracker::mark_episode_watched,
            tv_tracker::mark_all_episodes_watched,
            diary_collections::get_diary,
            diary_collections::get_library_diary,
            diary_collections::get_collections,
            diary_collections::create_collection,
            diary_collections::delete_collection,
            diary_collections::get_collection_items,
            diary_collections::add_to_collection,
            diary_collections::remove_from_collection,
            diary_collections::get_user_collections_for_title,
            // Синк
            sync::commands::sync_get_identity,
            sync::commands::sync_rename_device,
            sync::commands::sync_get_status,
            sync::commands::sync_get_peers,
            sync::commands::sync_forget_peer,
            sync::commands::sync_now,
            sync::commands::sync_pair_and_sync,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}