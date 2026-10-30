use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::time::{Duration, Instant};

const SHIKIMORI_URL: &str = "https://shikimori.io/api/graphql";

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    variables: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
}

#[derive(Debug, Deserialize)]
struct SearchData {
    animes: Vec<ShikiAnime>,
}

#[derive(Debug, Deserialize)]
struct DetailsData {
    animes: Vec<ShikiAnimeFull>,
}

#[derive(Debug, Deserialize)]
struct ShikiAnime {
    id: String,
    russian: Option<String>,
    name: Option<String>,
    description: Option<String>,
    episodes: Option<i64>,
    score: Option<f64>,
    kind: Option<String>,
    #[serde(rename = "airedOn")]
    aired_on: Option<ShikiDate>,
    poster: Option<ShikiPoster>,
}

#[derive(Debug, Deserialize)]
struct ShikiAnimeFull {
    id: String,
    russian: Option<String>,
    name: Option<String>,
    description: Option<String>,
    episodes: Option<i64>,
    duration: Option<i64>,
    score: Option<f64>,
    kind: Option<String>,
    status: Option<String>,
    genres: Option<Vec<ShikiGenre>>,
    studios: Option<Vec<ShikiStudio>>,
    #[serde(rename = "airedOn")]
    aired_on: Option<ShikiDate>,
    poster: Option<ShikiPoster>,
}

#[derive(Debug, Deserialize)]
struct ShikiGenre {
    russian: Option<String>,
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ShikiStudio {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ShikiDate {
    year: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct ShikiPoster {
    #[serde(rename = "originalUrl")]
    original_url: Option<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ShikiSearchItem {
    pub mal_id: i64,
    pub title: String,
    pub title_original: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub episodes: Option<i64>,
    pub average_score: Option<f64>,
    pub format: Option<String>,
    pub year: Option<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ShikiAnimeDetails {
    pub mal_id: i64,
    pub title: String,
    pub title_original: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub episodes: Option<i64>,
    pub duration: Option<i64>,
    pub average_score: Option<f64>,
    pub format: Option<String>,
    pub status: Option<String>,
    pub genres: Vec<String>,
    pub studio: Option<String>,
    pub year: Option<i64>,
}

// ==========================================
// IN-MEMORY ПУЛ ДЛЯ РУЛЕТКИ
// ==========================================
//
// Shikimori имеет лимит 5 запросов/сек и 90/мин.
// Рулетка может крутиться каждые 2 секунды — так мы быстро
// упрёмся в лимит.
//
// Решение: один запрос на 50 аниме, складываем в пул,
// крутим локально. Обновляем пул, когда он пуст или старше 10 минут.
//
struct AnimePool {
    items: Vec<ShikiSearchItem>,
    fetched_at: Option<Instant>,
}

static POOL: Mutex<Option<AnimePool>> = Mutex::new(None);

const POOL_TTL_SECS: u64 = 600; // 10 минут
const POOL_SIZE: i64 = 50;

fn pool_fresh(pool: &AnimePool) -> bool {
    match pool.fetched_at {
        Some(t) => t.elapsed() < Duration::from_secs(POOL_TTL_SECS),
        None => false,
    }
}

/// Забирает весь пул (для отладки/рулетки).
fn pool_take_random() -> Option<ShikiSearchItem> {
    let guard = POOL.lock().ok()?;
    let pool = guard.as_ref()?;
    if pool.items.is_empty() {
        return None;
    }
    let idx = (rand::random::<u32>() as usize) % pool.items.len();
    Some(pool.items[idx].clone())
}

fn pool_replace(items: Vec<ShikiSearchItem>) {
    if let Ok(mut guard) = POOL.lock() {
        *guard = Some(AnimePool {
            items,
            fetched_at: Some(Instant::now()),
        });
    }
}

fn pool_len() -> usize {
    POOL.lock()
        .ok()
        .and_then(|g| g.as_ref().map(|p| p.items.len()))
        .unwrap_or(0)
}

fn pool_is_fresh() -> bool {
    POOL.lock()
        .ok()
        .and_then(|g| g.as_ref().map(pool_fresh))
        .unwrap_or(false)
}

/// Выбирает название в зависимости от локали.
/// - ru: русское приоритетнее
/// - en / zh-CN: английское / оригинальное приоритетнее
fn pick_title(russian: &Option<String>, name: &Option<String>, locale: &str) -> String {
    if locale == "ru" {
        russian.clone()
            .or_else(|| name.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    } else {
        name.clone()
            .or_else(|| russian.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    }
}

pub async fn search_anime(query_str: &str, limit: i64, locale: &str) -> Result<Vec<ShikiSearchItem>, String> {
    let query = r#"
        query ($search: String, $limit: Int) {
            animes(search: $search, limit: $limit) {
                id
                russian
                name
                description
                episodes
                score
                kind
                airedOn { year }
                poster { originalUrl }
            }
        }
    "#;

    let body = GraphQLRequest {
        query: query.to_string(),
        variables: serde_json::json!({ "search": query_str, "limit": limit }),
    };

    let client = reqwest::Client::new();
    let resp = client.post(SHIKIMORI_URL)
        .header("User-Agent", "35mmClub")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Shikimori error: {}", resp.status()));
    }

    let parsed: GraphQLResponse<SearchData> = resp.json().await.map_err(|e| e.to_string())?;

    let items = parsed.data.map(|d| d.animes).unwrap_or_default().into_iter().filter_map(|a| {
        let id = a.id.parse::<i64>().ok()?;
        Some(ShikiSearchItem {
            mal_id: id,
            title: pick_title(&a.russian, &a.name, locale),
            title_original: a.name,
            description: a.description,
            cover_url: a.poster.and_then(|p| p.original_url),
            episodes: a.episodes,
            average_score: a.score,
            format: a.kind,
            year: a.aired_on.and_then(|d| d.year),
        })
    }).collect();

    Ok(items)
}

pub async fn get_anime_details(mal_id: i64, locale: &str) -> Result<ShikiAnimeDetails, String> {
    let query = r#"
        query ($ids: String) {
            animes(ids: $ids) {
                id
                russian
                name
                description
                episodes
                duration
                score
                kind
                status
                genres { russian name }
                studios { name }
                airedOn { year }
                poster { originalUrl }
            }
        }
    "#;

    let body = GraphQLRequest {
        query: query.to_string(),
        variables: serde_json::json!({ "ids": mal_id.to_string() }),
    };

    let client = reqwest::Client::new();
    let resp = client.post(SHIKIMORI_URL)
        .header("User-Agent", "35mmClub")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Shikimori error: {}", resp.status()));
    }

    let parsed: GraphQLResponse<DetailsData> = resp.json().await.map_err(|e| e.to_string())?;

    let a = parsed.data.and_then(|d| d.animes.into_iter().next()).ok_or("Not found")?;

    let genres = a.genres.unwrap_or_default().into_iter()
        .map(|g| if locale == "ru" { g.russian.or(g.name).unwrap_or_default() } else { g.name.or(g.russian).unwrap_or_default() })
        .filter(|s| !s.is_empty())
        .collect();

    let studio = a.studios.and_then(|s| s.into_iter().next()).and_then(|s| s.name);

    Ok(ShikiAnimeDetails {
        mal_id: a.id.parse().unwrap_or(0),
        title: pick_title(&a.russian, &a.name, locale),
        title_original: a.name,
        description: a.description,
        cover_url: a.poster.and_then(|p| p.original_url),
        episodes: a.episodes,
        duration: a.duration,
        average_score: a.score,
        format: a.kind,
        status: a.status,
        genres,
        studio,
        year: a.aired_on.and_then(|d| d.year),
    })
}

// ==========================================
// DISCOVER (для рулетки)
// ==========================================

/// Запрашивает пул из 50 аниме с Shikimori.
/// Сортировка — по популярности (`order: popularity`), без поискового запроса.
///
/// Shikimori GraphQL: `animes(limit: 50, order: popularity, kind: tv)`.
/// `kind: tv` отсекает OVA/фильмы — но рулетка хочет «что посмотреть»,
/// включая и фильмы. Поэтому запрашиваем без kind, а фильтруем сами
/// (или не фильтруем — пусть выпадает что угодно).
async fn fetch_pool(locale: &str) -> Result<Vec<ShikiSearchItem>, String> {
    let query = r#"
        query ($limit: Int) {
            animes(limit: $limit, order: popularity) {
                id
                russian
                name
                description
                episodes
                score
                kind
                airedOn { year }
                poster { originalUrl }
            }
        }
    "#;

    let body = GraphQLRequest {
        query: query.to_string(),
        variables: serde_json::json!({ "limit": POOL_SIZE }),
    };

    let client = reqwest::Client::new();
    let resp = client.post(SHIKIMORI_URL)
        .header("User-Agent", "35mmClub")
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("Shikimori error: {}", resp.status()));
    }

    let parsed: GraphQLResponse<SearchData> = resp.json().await.map_err(|e| e.to_string())?;

    let items: Vec<ShikiSearchItem> = parsed
        .data
        .map(|d| d.animes)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|a| {
            let id = a.id.parse::<i64>().ok()?;
            Some(ShikiSearchItem {
                mal_id: id,
                title: pick_title(&a.russian, &a.name, locale),
                title_original: a.name,
                description: a.description,
                cover_url: a.poster.and_then(|p| p.original_url),
                episodes: a.episodes,
                average_score: a.score,
                format: a.kind,
                year: a.aired_on.and_then(|d| d.year),
            })
        })
        .collect();

    Ok(items)
}

/// Возвращает один случайный аниме-тайтл.
///
/// Логика:
/// 1. Если пул свежий (моложе 10 минут) и не пуст — берём локально.
/// 2. Иначе запрашиваем Shikimori (1 запрос на 50 аниме), складываем в пул.
/// 3. Если пул пуст после запроса — ошибка.
///
/// `locale` влияет на выбор названия (ru: русское, иначе — оригинальное).
pub async fn discover_random(locale: &str) -> Result<ShikiSearchItem, String> {
    // 1. Если пул свежий — локальный рандом
    if pool_is_fresh() && pool_len() > 0 {
        if let Some(item) = pool_take_random() {
            return Ok(item);
        }
    }

    // 2. Иначе — обновляем пул
    let items = fetch_pool(locale).await?;

    if items.is_empty() {
        return Err("Shikimori вернул пустой список".to_string());
    }

    // 3. Заменяем пул
    pool_replace(items.clone());

    // 4. Берём случайный из свежего пула
    let idx = (rand::random::<u32>() as usize) % items.len();
    Ok(items[idx].clone())
}