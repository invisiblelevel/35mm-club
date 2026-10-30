use serde::Serialize;

use crate::bangumi;
use crate::shikimori;
use crate::tmdb;

#[derive(Debug, Serialize, Clone)]
pub struct RollResult {
    pub source: String,
    pub media_type: String,
    pub tmdb_id: Option<i64>,
    pub mal_id: Option<i64>,
    pub title: String,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub poster_url: Option<String>,
    pub year: Option<i64>,
    pub rating: Option<f64>,
    pub runtime: Option<i64>,
    pub episodes: Option<i64>,
    pub format: Option<String>,
    pub genres: Vec<String>,
}

/// Вспомогательная функция для получения одного результата
async fn fetch_one(
    media_type: &str,
    locale: &str,
    user_api_key: Option<String>,
) -> Result<RollResult, String> {
    match media_type {
        "movie" | "tv" => {
            let item = tmdb::discover_random(media_type, user_api_key, locale).await?;
            Ok(RollResult {
                source: "tmdb".to_string(),
                media_type: media_type.to_string(),
                tmdb_id: Some(item.tmdb_id),
                mal_id: None,
                title: item.title,
                original_title: item.original_title,
                overview: item.overview,
                poster_url: item.poster_url,
                year: item.year,
                rating: item.rating,
                runtime: item.runtime,
                episodes: item.episodes,
                format: None,
                genres: item.genres,
            })
        }
        "anime" => {
            if locale == "zh-CN" {
                let item = bangumi::discover_random().await?;
                Ok(RollResult {
                    source: "bangumi".to_string(),
                    media_type: media_type.to_string(),
                    tmdb_id: None,
                    mal_id: item.mal_id,
                    title: item.title,
                    original_title: item.title_original,
                    overview: item.description,
                    poster_url: item.cover_url,
                    year: item.year,
                    rating: item.average_score,
                    runtime: None,
                    episodes: item.episodes,
                    format: item.format,
                    genres: Vec::new(),
                })
            } else {
                let item = shikimori::discover_random(locale).await?;
                Ok(RollResult {
                    source: "shikimori".to_string(),
                    media_type: media_type.to_string(),
                    tmdb_id: None,
                    mal_id: Some(item.mal_id),
                    title: item.title,
                    original_title: item.title_original,
                    overview: item.description,
                    poster_url: item.cover_url,
                    year: item.year,
                    rating: item.average_score,
                    runtime: None,
                    episodes: item.episodes,
                    format: item.format,
                    genres: Vec::new(),
                })
            }
        }
        _ => Err(format!("Unknown media type: {}", media_type)),
    }
}

/// Команда 1 (одиночная, старая) — оставлена для совместимости
#[tauri::command]
pub async fn roll_random(
    media_type: String,
    locale: String,
    user_api_key: Option<String>,
) -> Result<RollResult, String> {
    fetch_one(&media_type, &locale, user_api_key).await
}

/// Команда 2 (батч из 8 штук) — для нового колеса
#[tauri::command]
pub async fn roll_random_batch(
    media_type: String,
    locale: String,
    user_api_key: Option<String>,
) -> Result<Vec<RollResult>, String> {
    let mut items = Vec::with_capacity(8);

    if media_type == "movie" || media_type == "tv" {
        // Для TMDB делаем 8 параллельных запросов (быстро)
        let mut handles = Vec::new();
        for _ in 0..8 {
            let mt = media_type.clone();
            let loc = locale.clone();
            let key = user_api_key.clone();
            handles.push(tokio::spawn(async move {
                fetch_one(&mt, &loc, key).await
            }));
        }
        for h in handles {
            if let Ok(res) = h.await {
                if let Ok(item) = res {
                    items.push(item);
                }
            }
        }
    } else {
        // Для аниме просто берём 8 раз из пула
        for _ in 0..8 {
            if let Ok(item) = fetch_one(&media_type, &locale, user_api_key.clone()).await {
                items.push(item);
            }
        }
    }

    if items.is_empty() {
        return Err("Не удалось получить результаты".to_string());
    }

    Ok(items)
}