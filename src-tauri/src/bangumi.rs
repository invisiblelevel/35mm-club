use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::time::{timeout, Duration};

const BANGUMI_API: &str = "https://api.bgm.tv";
const USER_AGENT: &str = "35mmClub/1.2.0 (https://github.com/invisiblelevel/35mm-club)";

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

// Кэш: bangumi_id → mal_id (или None, если не найдено)
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

pub async fn search_anime(query: &str, limit: i64) -> Result<Vec<BangumiSearchItem>, String> {
    let encoded = urlencoding::encode(query);
    let url = format!(
        "{}/search/subject/{}?type=2&responseGroup=large&max_results={}",
        BANGUMI_API, encoded, limit
    );

    let client = reqwest::Client::new();
    let resp = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| format!("Bangumi request error: {}", e))?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        let preview: String = body.chars().take(200).collect();
        return Err(format!("Bangumi HTTP {}: {}", status, preview));
    }

    let parsed: BangumiSearchResponse = resp
        .json()
        .await
        .map_err(|e| format!("Bangumi parse error: {}", e))?;

    let subjects = parsed.list.unwrap_or_default();

    let items: Vec<BangumiSearchItem> = subjects
        .into_iter()
        .take(limit as usize)
        .map(|s| {
            let cover = s.images.as_ref().and_then(|i| i.large.clone().or(i.common.clone()));
            let year = s.air_date.as_ref().and_then(|d| d.split('-').next()).and_then(|y| y.parse::<i64>().ok());
            let score = s.rating.as_ref().and_then(|r| r.score);
            let title = s.name_cn.clone()
                .or(s.name.clone())
                .unwrap_or_else(|| format!("Subject {}", s.id));
            let title_original = s.name.clone();

            BangumiSearchItem {
                bangumi_id: s.id,
                title,
                title_original,
                description: s.summary,
                cover_url: cover,
                episodes: s.eps_count.or(s.eps),
                format: None,
                status: None,
                average_score: score,
                year,
                mal_id: None,
            }
        })
        .collect();

    Ok(items)
}

/// Извлекает MAL ID из infobox тайтла на Bangumi.
async fn mal_from_bangumi_infobox(bangumi_id: i64) -> Option<i64> {
    let url = format!("{}/subject/{}", BANGUMI_API, bangumi_id);
    let client = reqwest::Client::new();

    let resp = timeout(
        Duration::from_secs(5),
        client.get(&url)
            .header("User-Agent", USER_AGENT)
            .header("Accept", "application/json")
            .send()
    ).await.ok()?.ok()?;

    if !resp.status().is_success() {
        return None;
    }

    let json: serde_json::Value = resp.json().await.ok()?;
    let infobox = json.get("infobox")?.as_array()?;

    for item in infobox {
        let key = match item.get("key").and_then(|v| v.as_str()) {
            Some(k) => k.to_lowercase(),
            None => continue,
        };
        if key.contains("mal") || key.contains("myanimelist") {
            let val = item.get("value")?;
            if let Some(s) = val.as_str() {
                if let Ok(id) = s.trim().parse::<i64>() {
                    return Some(id);
                }
            }
            if let Some(arr) = val.as_array() {
                for v in arr {
                    if let Some(s) = v.as_str() {
                        if let Ok(id) = s.trim().parse::<i64>() {
                            return Some(id);
                        }
                    }
                }
            }
        }
    }

    None
}

/// Ищет MAL ID по bangumi_id. Кэширует результат.
pub async fn find_mal_id(bangumi_id: i64, title: &str) -> Option<i64> {
    // 1. Кэш
    if let Some(cached) = cache_get(bangumi_id) {
        return cached;
    }

    // 2. Bangumi infobox (быстро, без AniList)
    if let Some(mal_id) = mal_from_bangumi_infobox(bangumi_id).await {
        cache_set(bangumi_id, Some(mal_id));
        return Some(mal_id);
    }

    // 3. Fallback: AniList поиск по названию
    let anilist_result = timeout(
        Duration::from_secs(5),
        crate::anilist::search_anime(title, 1, 5)
    ).await.ok()
        .and_then(|r| r.ok())
        .and_then(|r| r.first().and_then(|a| a.id_mal));

    if let Some(mal_id) = anilist_result {
        cache_set(bangumi_id, Some(mal_id));
        return Some(mal_id);
    }

    cache_set(bangumi_id, None);
    None
}