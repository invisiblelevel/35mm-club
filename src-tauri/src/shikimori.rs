use serde::{Deserialize, Serialize};

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