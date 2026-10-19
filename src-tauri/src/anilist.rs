use serde::{Deserialize, Serialize};

const ANILIST_URL: &str = "https://graphql.anilist.co";

// ---------- Модели ответов AniList ----------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AniListTitle {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AniListCover {
    pub large: Option<String>,
    pub medium: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AniListStudio {
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AniListMedia {
    pub id: i64,
    #[serde(rename = "idMal")]
    pub id_mal: Option<i64>,
    pub title: AniListTitle,
    pub description: Option<String>,
    #[serde(rename = "coverImage")]
    pub cover_image: Option<AniListCover>,
    #[serde(rename = "bannerImage")]
    pub banner_image: Option<String>,
    pub episodes: Option<i64>,
    pub duration: Option<i64>,
    pub status: Option<String>,
    pub format: Option<String>,
    pub genres: Option<Vec<String>>,
    pub studios: Option<AniListStudioConnection>,
    #[serde(rename = "averageScore")]
    pub average_score: Option<i64>,
    #[serde(rename = "startDate")]
    pub start_date: Option<AniListFuzzyDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AniListStudioConnection {
    pub nodes: Option<Vec<AniListStudio>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AniListFuzzyDate {
    pub year: Option<i64>,
    pub month: Option<i64>,
    pub day: Option<i64>,
}

// ---------- Модели запросов ----------

#[derive(Debug, Serialize)]
struct GraphQLRequest {
    query: String,
    variables: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct GraphQLResponse<T> {
    data: Option<T>,
    errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
struct GraphQLError {
    message: String,
}

#[derive(Debug, Deserialize)]
struct SearchData {
    #[serde(rename = "Page")]
    page: PageData,
}

#[derive(Debug, Deserialize)]
struct PageData {
    media: Vec<AniListMedia>,
}

#[derive(Debug, Deserialize)]
struct MediaData {
    #[serde(rename = "Media")]
    media: AniListMedia,
}

// ---------- Публичные структуры для фронта ----------

#[derive(Debug, Serialize, Clone)]
pub struct AnimeSearchItem {
    pub anilist_id: i64,
    pub id_mal: Option<i64>,
    pub title: String,
    pub title_romaji: Option<String>,
    pub title_native: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub episodes: Option<i64>,
    pub format: Option<String>,
    pub status: Option<String>,
    pub average_score: Option<i64>,
    pub year: Option<i64>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AnimeDetails {
    pub anilist_id: i64,
    pub id_mal: Option<i64>,
    pub title: String,
    pub title_romaji: Option<String>,
    pub title_native: Option<String>,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub banner_url: Option<String>,
    pub episodes: Option<i64>,
    pub duration: Option<i64>,
    pub format: Option<String>,
    pub status: Option<String>,
    pub genres: Vec<String>,
    pub studio: Option<String>,
    pub average_score: Option<i64>,
    pub year: Option<i64>,
}

// ---------- Вспомогательные ----------

fn pick_title(t: &AniListTitle) -> String {
    t.english.clone()
        .or_else(|| t.romaji.clone())
        .or_else(|| t.native.clone())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn year_from_date(d: &Option<AniListFuzzyDate>) -> Option<i64> {
    d.as_ref().and_then(|x| x.year)
}

// ---------- Запросы ----------

pub async fn search_anime(query: &str, page: i64, per_page: i64) -> Result<Vec<AnimeSearchItem>, String> {
    let gql = r#"
        query ($search: String, $page: Int, $perPage: Int) {
            Page(page: $page, perPage: $perPage) {
                media(search: $search, type: ANIME, sort: SEARCH_MATCH) {
                    id
                    idMal
                    title { romaji english native }
                    description
                    coverImage { large medium }
                    episodes
                    format
                    status
                    averageScore
                    startDate { year }
                }
            }
        }
    "#;

    let body = GraphQLRequest {
        query: gql.to_string(),
        variables: serde_json::json!({
            "search": query,
            "page": page,
            "perPage": per_page
        }),
    };

    let client = reqwest::Client::new();
    let resp = client.post(ANILIST_URL)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("AniList error: {}", resp.status()));
    }

    let raw_text = resp.text().await.map_err(|e| e.to_string())?;
    let parsed: GraphQLResponse<SearchData> = serde_json::from_str(&raw_text).map_err(|e| e.to_string())?;

    if let Some(errors) = parsed.errors {
        if !errors.is_empty() {
            return Err(errors[0].message.clone());
        }
    }

    let page_data = parsed.data.ok_or("No data")?.page;

    let items: Vec<AnimeSearchItem> = page_data.media.into_iter().map(|m| {
        let cover = m.cover_image.as_ref().and_then(|c| c.large.clone().or(c.medium.clone()));
        AnimeSearchItem {
            anilist_id: m.id,
            id_mal: m.id_mal,
            title: pick_title(&m.title),
            title_romaji: m.title.romaji,
            title_native: m.title.native,
            description: m.description,
            cover_url: cover,
            episodes: m.episodes,
            format: m.format,
            status: m.status,
            average_score: m.average_score,
            year: year_from_date(&m.start_date),
        }
    }).collect();

    Ok(items)
}

pub async fn get_anime_details(anilist_id: i64) -> Result<AnimeDetails, String> {
    let gql = r#"
        query ($id: Int) {
            Media(id: $id, type: ANIME) {
                id
                idMal
                title { romaji english native }
                description
                coverImage { large medium }
                bannerImage
                episodes
                duration
                format
                status
                genres
                studios { nodes { name } }
                averageScore
                startDate { year }
            }
        }
    "#;

    let body = GraphQLRequest {
        query: gql.to_string(),
        variables: serde_json::json!({ "id": anilist_id }),
    };

    let client = reqwest::Client::new();
    let resp = client.post(ANILIST_URL)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Err(format!("AniList error: {}", resp.status()));
    }

    let parsed: GraphQLResponse<MediaData> = resp.json().await.map_err(|e| e.to_string())?;

    if let Some(errors) = parsed.errors {
        if !errors.is_empty() {
            return Err(errors[0].message.clone());
        }
    }

    let m = parsed.data.ok_or("No data")?.media;

    let genres = m.genres.unwrap_or_default();
    let studio = m.studios
        .and_then(|s| s.nodes)
        .and_then(|n| n.first().and_then(|s| s.name.clone()));

    Ok(AnimeDetails {
        anilist_id: m.id,
        id_mal: m.id_mal,
        title: pick_title(&m.title),
        title_romaji: m.title.romaji,
        title_native: m.title.native,
        description: m.description,
        cover_url: m.cover_image.and_then(|c| c.large.or(c.medium)),
        banner_url: m.banner_image,
        episodes: m.episodes,
        duration: m.duration,
        format: m.format,
        status: m.status,
        genres,
        studio,
        average_score: m.average_score,
        year: year_from_date(&m.start_date),
    })
}

/// Получает описание аниме из AniList по его MAL ID.
pub async fn get_description_by_mal_id(mal_id: i64) -> Option<String> {
    let gql = r#"
        query ($idMal: Int) {
            Media(idMal: $idMal, type: ANIME) {
                description
            }
        }
    "#;

    let body = GraphQLRequest {
        query: gql.to_string(),
        variables: serde_json::json!({ "idMal": mal_id }),
    };

    let client = reqwest::Client::new();
    let resp = client.post(ANILIST_URL)
        .json(&body)
        .send()
        .await
        .ok()?;

    if !resp.status().is_success() { return None; }

    #[derive(Debug, Deserialize)]
    struct DescData { #[serde(rename = "Media")] media: Option<DescMedia> }
    #[derive(Debug, Deserialize)]
    struct DescMedia { description: Option<String> }

    let parsed: GraphQLResponse<DescData> = resp.json().await.ok()?;
    parsed.data?.media?.description
}