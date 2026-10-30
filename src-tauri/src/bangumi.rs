use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tokio::time::timeout;

const BANGUMI_API: &str = "https://api.bgm.tv";
// Браузерный User-Agent — без него Cloudflare блокирует /v0/subjects
const BROWSER_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

#[derive(Debug, Serialize, Clone)]
pub struct BangumiSearchItem {
    pub bangumi_id: i64,
    pub title: String,
    pub title_original: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub episodes: Option<i64>,
    pub format: Option<String>,
    pub status: Option<String>,
    pub average_score: Option<f64>,
    pub year: Option<i64>,
    pub mal_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct BangumiSearchResponse {
    results: Option<i64>,
    list: Option<Vec<BangumiSubject>>,
}

#[derive(Debug, Deserialize)]
struct BangumiSubject {
    id: i64,
    name: Option<String>,
    name_cn: Option<String>,
    summary: Option<String>,
    images: Option<BangumiImages>,
    eps: Option<i64>,
    eps_count: Option<i64>,
    air_date: Option<String>,
    rating: Option<BangumiRating>,
}

#[derive(Debug, Deserialize)]
struct BangumiImages {
    large: Option<String>,
    common: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BangumiRating {
    score: Option<f64>,
}

// ==========================================
// V0 API (для рулетки) — С ПРАВИЛЬНОЙ ОБЁРТКОЙ
// ==========================================

#[derive(Debug, Deserialize)]
struct BangumiV0Response {
    data: Vec<BangumiV0Subject>,
}

#[derive(Debug, Deserialize)]
struct BangumiV0Subject {
    id: i64,
    name: Option<String>,
    name_cn: Option<String>,
    summary: Option<String>,
    images: Option<BangumiV0Images>,
    eps: Option<i64>,
    total_episodes: Option<i64>,
    date: Option<String>,
    rating: Option<BangumiV0Rating>,
}

#[derive(Debug, Deserialize)]
struct BangumiV0Images {
    large: Option<String>,
    common: Option<String>,
    medium: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BangumiV0Rating {
    score: Option<f64>,
}

// Кэш: bangumi_id → mal_id
static MAL_CACHE: Mutex<Option<HashMap<i64, Option<i64>>>> = Mutex::new(None);

fn cache_get(bangumi_id: i64) -> Option<Option<i64>> {
    let guard = MAL_CACHE.lock().unwrap();
    guard.as_ref().and_then(|m| m.get(&bangumi_id).copied())
}

fn cache_set(bangumi_id: i64, mal_id: Option<i64>) {
    let mut guard = MAL_CACHE.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    if let Some(ref mut map) = *guard {
        map.insert(bangumi_id, mal_id);
    }
}

// ==========================================
// IN-MEMORY ПУЛ ДЛЯ РУЛЕТКИ
// ==========================================

struct BangumiPool {
    items: Vec<BangumiSearchItem>,
    fetched_at: Option<Instant>,
}

static POOL: Mutex<Option<BangumiPool>> = Mutex::new(None);

const POOL_TTL_SECS: u64 = 600;
const POOL_SIZE: i64 = 50;

fn pool_fresh(pool: &BangumiPool) -> bool {
    match pool.fetched_at {
        Some(t) => t.elapsed() < Duration::from_secs(POOL_TTL_SECS),
        None => false,
    }
}

fn pool_take_random() -> Option<BangumiSearchItem> {
    let guard = POOL.lock().ok()?;
    let pool = guard.as_ref()?;
    if pool.items.is_empty() { return None; }
    let idx = (rand::random::<u32>() as usize) % pool.items.len();
    Some(pool.items[idx].clone())
}

fn pool_replace(items: Vec<BangumiSearchItem>) {
    if let Ok(mut guard) = POOL.lock() {
        *guard = Some(BangumiPool { items, fetched_at: Some(Instant::now()) });
    }
}

fn pool_len() -> usize {
    POOL.lock().ok().and_then(|g| g.as_ref().map(|p| p.items.len())).unwrap_or(0)
}

fn pool_is_fresh() -> bool {
    POOL.lock().ok().and_then(|g| g.as_ref().map(pool_fresh)).unwrap_or(false)
}

// ==========================================
// ПОИСК (старый API, не трогаем)
// ==========================================

pub async fn search_anime(query: &str, limit: i64) -> Result<Vec<BangumiSearchItem>, String> {
    let encoded = urlencoding::encode(query);
    let url = format!("{}/search/subject/{}?type=2&responseGroup=large&max_results={}", BANGUMI_API, encoded, limit);

    let client = reqwest::Client::new();
    let resp = client.get(&url).header("User-Agent", USER_AGENT).header("Accept", "application/json")
        .send().await.map_err(|e| format!("Bangumi request error: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        let preview: String = body.chars().take(200).collect();
        return Err(format!("Bangumi HTTP {}: {}", status, preview));
    }

    let parsed: BangumiSearchResponse = resp.json().await.map_err(|e| format!("Bangumi parse error: {}", e))?;
    let subjects = parsed.list.unwrap_or_default();

    let items: Vec<BangumiSearchItem> = subjects.into_iter().take(limit as usize).map(|s| {
        let cover = s.images.as_ref().and_then(|i| i.large.clone().or(i.common.clone()));
        let year = s.air_date.as_ref().and_then(|d| d.split('-').next()).and_then(|y| y.parse::<i64>().ok());
        let score = s.rating.as_ref().and_then(|r| r.score);
        let title = s.name_cn.clone().or(s.name.clone()).unwrap_or_else(|| format!("Subject {}", s.id));
        let title_original = s.name.clone();

        BangumiSearchItem {
            bangumi_id: s.id, title, title_original, description: s.summary,
            cover_url: cover, episodes: s.eps_count.or(s.eps), format: None, status: None,
            average_score: score, year, mal_id: None,
        }
    }).collect();

    Ok(items)
}

// Восстанавливаем константу для поиска, т.к. удалили её
const USER_AGENT: &str = "35mmClub/1.2.0 (https://github.com/invisiblelevel/35mm-club)";

async fn mal_from_bangumi_infobox(bangumi_id: i64) -> Option<i64> {
    let url = format!("{}/subject/{}", BANGUMI_API, bangumi_id);
    let client = reqwest::Client::new();

    // Тут тоже лучше браузерный, но для совместимости оставим старый. Если не работает — заменим.
    let resp = timeout(Duration::from_secs(5), client.get(&url).header("User-Agent", BROWSER_UA).header("Accept", "application/json").send()).await.ok()?.ok()?;
    if !resp.status().is_success() { return None; }

    let json: serde_json::Value = resp.json().await.ok()?;
    let infobox = json.get("infobox")?.as_array()?;

    for item in infobox {
        let key = item.get("key").and_then(|v| v.as_str())?.to_lowercase();
        if key.contains("mal") || key.contains("myanimelist") {
            let val = item.get("value")?;
            if let Some(s) = val.as_str() {
                if let Ok(id) = s.trim().parse::<i64>() { return Some(id); }
            }
            if let Some(arr) = val.as_array() {
                for v in arr {
                    if let Some(s) = v.as_str() {
                        if let Ok(id) = s.trim().parse::<i64>() { return Some(id); }
                    }
                }
            }
        }
    }
    None
}

pub async fn find_mal_id(bangumi_id: i64, title: &str) -> Option<i64> {
    if let Some(cached) = cache_get(bangumi_id) { return cached; }

    if let Some(mal_id) = mal_from_bangumi_infobox(bangumi_id).await {
        cache_set(bangumi_id, Some(mal_id));
        return Some(mal_id);
    }

    let anilist_result = timeout(Duration::from_secs(5), crate::anilist::search_anime(title, 1, 5)).await.ok()
        .and_then(|r| r.ok())
        .and_then(|r| r.first().and_then(|a| a.id_mal));

    if let Some(mal_id) = anilist_result {
        cache_set(bangumi_id, Some(mal_id));
        return Some(mal_id);
    }

    cache_set(bangumi_id, None);
    None
}

// ==========================================
// DISCOVER (для рулетки) — БРАУЗЕРНЫЙ USER-AGENT
// ==========================================

async fn fetch_pool(offset: i64) -> Result<Vec<BangumiSearchItem>, String> {
    let url = format!("{}/v0/subjects?type=2&sort=rank&limit={}&offset={}", BANGUMI_API, POOL_SIZE, offset);

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(&url)
        .header("User-Agent", BROWSER_UA) // <--- ВОТ ГЛАВНЫЙ ФИКС
        .header("Accept", "application/json")
        .header("Referer", "https://bgm.tv/")
        .send().await
        .map_err(|e| format!("Bangumi request error: {}", e))?;

    if !resp.status().is_success() { return Err(format!("Bangumi HTTP {}", resp.status())); }

    let response: BangumiV0Response = resp.json().await.map_err(|e| format!("Bangumi parse error: {}", e))?;
    let subjects = response.data;

    let items: Vec<BangumiSearchItem> = subjects.into_iter().filter_map(|s| {
        let title = s.name_cn.clone().or(s.name.clone())?;
        let title_original = s.name.clone();
        let cover = s.images.as_ref().and_then(|i| i.large.clone().or(i.common.clone()).or(i.medium.clone()));
        let year = s.date.as_ref().and_then(|d| d.split('-').next()).and_then(|y| y.parse::<i64>().ok());
        let score = s.rating.as_ref().and_then(|r| r.score);
        let episodes = s.total_episodes.or(s.eps);

        Some(BangumiSearchItem {
            bangumi_id: s.id, title, title_original, description: s.summary,
            cover_url: cover, episodes, format: None, status: None,
            average_score: score, year, mal_id: None,
        })
    }).collect();

    Ok(items)
}

pub async fn discover_random() -> Result<BangumiSearchItem, String> {
    if pool_is_fresh() && pool_len() > 0 {
        if let Some(item) = pool_take_random() {
            if let Some(mal_id) = find_mal_id(item.bangumi_id, &item.title).await {
                let mut item = item;
                item.mal_id = Some(mal_id);
                return Ok(item);
            }
            return Ok(item);
        }
    }

    let offset = (rand::random::<u32>() % 200) as i64;
    let items = match tokio::time::timeout(Duration::from_secs(5), fetch_pool(offset)).await {
        Ok(Ok(items)) => items,
        Ok(Err(e)) => return Err(format!("Bangumi API error: {}", e)),
        Err(_) => return Err("Bangumi не отвечает (таймаут 5с).".to_string()),
    };

    if items.is_empty() {
        let items2 = match tokio::time::timeout(Duration::from_secs(5), fetch_pool(0)).await {
            Ok(Ok(items)) => items,
            Ok(Err(e)) => return Err(format!("Bangumi API error (fallback): {}", e)),
            Err(_) => return Err("Bangumi не отвечает (таймаут 5с).".to_string()),
        };
        if items2.is_empty() { return Err("Bangumi вернул пустой список".to_string()); }
        pool_replace(items2.clone());
        let idx = (rand::random::<u32>() as usize) % items2.len();
        let mut item = items2[idx].clone();
        if let Some(mal_id) = find_mal_id(item.bangumi_id, &item.title).await {
            item.mal_id = Some(mal_id);
        }
        return Ok(item);
    }

    pool_replace(items.clone());
    let idx = (rand::random::<u32>() as usize) % items.len();
    let mut item = items[idx].clone();
    if let Some(mal_id) = find_mal_id(item.bangumi_id, &item.title).await {
        item.mal_id = Some(mal_id);
    }

    Ok(item)
}