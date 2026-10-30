use serde::{Deserialize, Serialize};

const TMDB_BASE: &str = "https://api.themoviedb.org/3";
const TMDB_IMAGE_BASE: &str = "https://image.tmdb.org/t/p/w500";

/// Встроенный ключ приложения.
///
/// Читается на этапе компиляции из переменной окружения `TMDB_API_KEY`,
/// которую `build.rs` подхватывает из `.env` в корне проекта.
/// `.env` в `.gitignore` — в репозиторий ключ не попадает.
///
/// Fallback на `std::env::var` — для случаев, когда ключ передан
/// в окружение явно (CI, `set TMDB_API_KEY=... && cargo build`).
fn default_tmdb_key() -> String {
    if let Some(k) = option_env!("TMDB_API_KEY") {
        if !k.trim().is_empty() {
            return k.trim().to_string();
        }
    }
    if let Ok(k) = std::env::var("TMDB_API_KEY") {
        if !k.trim().is_empty() {
            return k.trim().to_string();
        }
    }
    String::new()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TmdbSearchResult {
    pub id: i64,
    pub title: Option<String>,
    pub name: Option<String>,
    pub original_title: Option<String>,
    pub original_name: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
    pub vote_average: Option<f64>,
    pub media_type: Option<String>,
    pub original_language: Option<String>,
    pub genre_ids: Option<Vec<i64>>,
}

#[derive(Debug, Deserialize)]
struct TmdbSearchResponse {
    results: Vec<TmdbSearchResult>,
}

#[derive(Debug, Serialize)]
pub struct MovieSearchItem {
    pub tmdb_id: i64,
    pub title: String,
    pub overview: Option<String>,
    pub poster_url: Option<String>,
    pub year: Option<i64>,
    pub rating: Option<f64>,
    pub media_type: String,
    pub original_language: Option<String>,
    pub genre_ids: Vec<i64>,
}

#[derive(Debug, Serialize)]
pub struct TitleDetails {
    pub tmdb_id: i64,
    pub title: String,
    pub overview: Option<String>,
    pub poster_url: Option<String>,
    pub backdrop_url: Option<String>,
    pub year: Option<i64>,
    pub rating: Option<f64>,
    pub runtime: Option<i64>,
    pub genres: Vec<String>,
    pub director: Option<String>,
    pub director_ids: Vec<i64>,
    pub cast: Vec<CastMember>,
    pub cast_ids: Vec<i64>,
    pub producer_ids: Vec<i64>,
    pub production_company_ids: Vec<i64>,
    pub keyword_ids: Vec<i64>,
    pub trailer_key: Option<String>,
    pub media_type: String,
    pub collection_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct CastMember {
    pub id: i64,
    pub name: String,
    pub character: Option<String>,
    pub profile_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TvSeason {
    pub season_number: i64,
    pub name: Option<String>,
    pub episode_count: i64,
    pub poster_url: Option<String>,
    pub watched_count: i64,
}

#[derive(Debug, Serialize)]
pub struct TvEpisode {
    pub episode_number: i64,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub runtime: Option<i64>,
    pub air_date: Option<String>,
    pub still_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TmdbDetailsRaw {
    id: i64,
    title: Option<String>,
    name: Option<String>,
    overview: Option<String>,
    poster_path: Option<String>,
    backdrop_path: Option<String>,
    release_date: Option<String>,
    first_air_date: Option<String>,
    vote_average: Option<f64>,
    runtime: Option<i64>,
    episode_run_time: Option<Vec<i64>>,
    genres: Option<Vec<TmdbGenre>>,
    credits: Option<TmdbCredits>,
    videos: Option<TmdbVideos>,
    belongs_to_collection: Option<TmdbCollection>,
    production_companies: Option<Vec<TmdbCompany>>,
    keywords: Option<TmdbKeywords>,
    seasons: Option<Vec<TmdbSeasonRaw>>,
}

#[derive(Debug, Deserialize)]
struct TmdbSeasonRaw {
    season_number: i64,
    name: Option<String>,
    episode_count: Option<i64>,
    poster_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TmdbSeasonDetails {
    episodes: Option<Vec<TmdbEpisodeRaw>>,
}

#[derive(Debug, Deserialize)]
struct TmdbEpisodeRaw {
    episode_number: i64,
    name: Option<String>,
    overview: Option<String>,
    runtime: Option<i64>,
    air_date: Option<String>,
    still_path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TmdbCollection {
    id: i64,
}

#[derive(Debug, Deserialize)]
struct TmdbGenre { name: String }

#[derive(Debug, Deserialize)]
struct TmdbCompany { id: i64 }

#[derive(Debug, Deserialize)]
struct TmdbKeywords {
    keywords: Option<Vec<TmdbKeyword>>,
    results: Option<Vec<TmdbKeyword>>,
}

#[derive(Debug, Deserialize)]
struct TmdbKeyword { id: i64 }

#[derive(Debug, Deserialize)]
struct TmdbCredits {
    cast: Option<Vec<TmdbCast>>,
    crew: Option<Vec<TmdbCrew>>,
}

#[derive(Debug, Deserialize)]
struct TmdbCast { id: i64, name: String, character: Option<String>, profile_path: Option<String> }

#[derive(Debug, Deserialize)]
struct TmdbCrew { id: i64, name: String, job: Option<String> }

#[derive(Debug, Deserialize)]
struct TmdbVideos { results: Option<Vec<TmdbVideo>> }

#[derive(Debug, Deserialize)]
struct TmdbVideo { key: String, site: String, #[serde(rename = "type")] video_type: String }

// ==========================================
// Discover (для рулетки)
// ==========================================

#[derive(Debug, Deserialize)]
struct TmdbDiscoverResponse {
    page: i64,
    total_pages: i64,
    results: Vec<TmdbSearchResult>,
}

/// Результат «рулетки» — универсальная структура для фильмов и сериалов.
/// Для аниме используется своя (в roulette.rs), но поля совпадают.
#[derive(Debug, Serialize, Clone)]
pub struct DiscoverItem {
    pub tmdb_id: i64,
    pub title: String,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub poster_url: Option<String>,
    pub year: Option<i64>,
    pub rating: Option<f64>,
    pub media_type: String,
    pub runtime: Option<i64>,
    pub episodes: Option<i64>,
    pub genres: Vec<String>,
}

fn resolve_api_key(user_key: Option<String>) -> String {
    match user_key {
        Some(k) if !k.trim().is_empty() => k.trim().to_string(),
        _ => default_tmdb_key(),
    }
}

fn resolve_tmdb_language(locale: &str) -> &'static str {
    match locale {
        "ru" => "ru-RU",
        "zh-CN" => "zh-CN",
        _ => "en-US",
    }
}

pub async fn search_movies(query: &str, media_type: &str, user_api_key: Option<String>, locale: &str) -> Result<Vec<MovieSearchItem>, String> {
    let api_key = resolve_api_key(user_api_key);
    if api_key.is_empty() { return Err("TMDB key not set".to_string()); }

    let endpoint = match media_type { "movie" => "search/movie", "tv" => "search/tv", _ => "search/multi" };
    let url = format!("{}/{}", TMDB_BASE, endpoint);
    let lang = resolve_tmdb_language(locale);

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .query(&[("api_key", api_key.as_str()), ("query", query), ("language", lang), ("include_adult", "false")])
        .send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() { return Err(format!("TMDB error: {}", response.status())); }

    let data: TmdbSearchResponse = response.json().await.map_err(|e| e.to_string())?;

    let items = data.results.into_iter().map(|r| {
        let title = r.title.or(r.name).or(r.original_title).or(r.original_name).unwrap_or_else(|| "Unknown".to_string());
        let date = r.release_date.or(r.first_air_date);
        let year = date.as_ref().and_then(|d| d.split('-').next()).and_then(|y| y.parse::<i64>().ok());
        let poster_url = r.poster_path.map(|p| format!("{}{}", TMDB_IMAGE_BASE, p));

        MovieSearchItem {
            tmdb_id: r.id, title, overview: r.overview, poster_url, year,
            rating: r.vote_average, media_type: r.media_type.unwrap_or_else(|| media_type.to_string()),
            original_language: r.original_language,
            genre_ids: r.genre_ids.unwrap_or_default(),
        }
    }).collect();

    Ok(items)
}

pub async fn get_title_details(tmdb_id: i64, media_type: &str, user_api_key: Option<String>, locale: &str) -> Result<TitleDetails, String> {
    let api_key = resolve_api_key(user_api_key);
    if api_key.is_empty() { return Err("TMDB key not set".to_string()); }

    let endpoint = match media_type { "tv" => format!("tv/{}", tmdb_id), _ => format!("movie/{}", tmdb_id) };
    let url = format!("{}/{}", TMDB_BASE, endpoint);
    let lang = resolve_tmdb_language(locale);

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .query(&[("api_key", api_key.as_str()), ("language", lang), ("append_to_response", "credits,videos,keywords")])
        .send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() { return Err(format!("TMDB error: {}", response.status())); }

    let raw: TmdbDetailsRaw = response.json().await.map_err(|e| e.to_string())?;

    let title = raw.title.or(raw.name).unwrap_or_else(|| "Unknown".to_string());
    let date = raw.release_date.or(raw.first_air_date);
    let year = date.as_ref().and_then(|d| d.split('-').next()).and_then(|y| y.parse::<i64>().ok());
    let poster_url = raw.poster_path.map(|p| format!("{}{}", TMDB_IMAGE_BASE, p));
    let backdrop_url = raw.backdrop_path.map(|p| format!("https://image.tmdb.org/t/p/w1280{}", p));
    let runtime = raw.runtime.or_else(|| raw.episode_run_time.as_ref().and_then(|v| v.first().copied()));
    let genres = raw.genres.unwrap_or_default().into_iter().map(|g| g.name).collect();

    let mut director = None;
    let mut director_ids = Vec::new();
    let mut cast = Vec::new();
    let mut cast_ids = Vec::new();
    let mut producer_ids = Vec::new();

    if let Some(credits) = raw.credits {
        if let Some(crew) = credits.crew {
            for member in crew {
                match member.job.as_deref() {
                    Some("Director") => {
                        director = Some(member.name.clone());
                        director_ids.push(member.id);
                    }
                    Some("Producer") => {
                        producer_ids.push(member.id);
                    }
                    _ => {}
                }
            }
        }
        if let Some(cast_raw) = credits.cast {
            for c in cast_raw.into_iter().take(15) {
                cast_ids.push(c.id);
                cast.push(CastMember {
                    id: c.id, name: c.name, character: c.character,
                    profile_url: c.profile_path.map(|p| format!("https://image.tmdb.org/t/p/w185{}", p)),
                });
            }
        }
    }

    let production_company_ids: Vec<i64> = raw
        .production_companies
        .unwrap_or_default()
        .into_iter()
        .map(|c| c.id)
        .collect();

    let keyword_ids: Vec<i64> = raw
        .keywords
        .and_then(|k| k.keywords.or(k.results))
        .unwrap_or_default()
        .into_iter()
        .map(|k| k.id)
        .collect();

    let trailer_key = raw.videos.and_then(|v| v.results.and_then(|results| {
        results.into_iter().find(|r| r.site == "YouTube" && r.video_type == "Trailer").map(|r| r.key)
    }));

    let mut collection_id = raw.belongs_to_collection.map(|c| c.id);

    if collection_id.is_none() {
        let collection_url = format!("{}/movie/{}/collection", TMDB_BASE, tmdb_id);
        if let Ok(coll_resp) = client.get(&collection_url)
            .query(&[("api_key", api_key.as_str()), ("language", lang)])
            .send().await
        {
            if coll_resp.status().is_success() {
                if let Ok(coll_json) = coll_resp.json::<serde_json::Value>().await {
                    if let Some(id) = coll_json.get("id").and_then(|v| v.as_i64()) {
                        collection_id = Some(id);
                    }
                }
            }
        }
    }

    Ok(TitleDetails {
        tmdb_id: raw.id, title, overview: raw.overview, poster_url, backdrop_url,
        year, rating: raw.vote_average, runtime, genres, director, director_ids,
        cast, cast_ids, producer_ids, production_company_ids, keyword_ids,
        trailer_key, media_type: media_type.to_string(),
        collection_id,
    })
}

pub async fn get_tv_seasons(tmdb_id: i64, user_api_key: Option<String>, locale: &str) -> Result<Vec<TvSeason>, String> {
    let api_key = resolve_api_key(user_api_key);
    if api_key.is_empty() { return Err("TMDB key not set".to_string()); }

    let url = format!("{}/tv/{}", TMDB_BASE, tmdb_id);
    let lang = resolve_tmdb_language(locale);

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .query(&[("api_key", api_key.as_str()), ("language", lang)])
        .send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() { return Err(format!("TMDB error: {}", response.status())); }

    let raw: TmdbDetailsRaw = response.json().await.map_err(|e| e.to_string())?;

    let seasons = raw.seasons.unwrap_or_default().into_iter().map(|s| {
        TvSeason {
            season_number: s.season_number,
            name: s.name,
            episode_count: s.episode_count.unwrap_or(0),
            poster_url: s.poster_path.map(|p| format!("{}{}", TMDB_IMAGE_BASE, p)),
            watched_count: 0,
        }
    }).collect();

    Ok(seasons)
}

pub async fn get_season_details(tmdb_id: i64, season_number: i64, user_api_key: Option<String>, locale: &str) -> Result<Vec<TvEpisode>, String> {
    let api_key = resolve_api_key(user_api_key);
    if api_key.is_empty() { return Err("TMDB key not set".to_string()); }

    let url = format!("{}/tv/{}/season/{}", TMDB_BASE, tmdb_id, season_number);
    let lang = resolve_tmdb_language(locale);

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .query(&[("api_key", api_key.as_str()), ("language", lang)])
        .send().await.map_err(|e| e.to_string())?;

    if !response.status().is_success() { return Err(format!("TMDB error: {}", response.status())); }

    let raw: TmdbSeasonDetails = response.json().await.map_err(|e| e.to_string())?;

    let episodes = raw.episodes.unwrap_or_default().into_iter().map(|e| {
        TvEpisode {
            episode_number: e.episode_number,
            name: e.name,
            overview: e.overview,
            runtime: e.runtime,
            air_date: e.air_date,
            still_url: e.still_path.map(|p| format!("https://image.tmdb.org/t/p/w300{}", p)),
        }
    }).collect();

    Ok(episodes)
}

// ==========================================
// DISCOVER RANDOM (для рулетки)
// ==========================================

/// Берёт случайный фильм или сериал с TMDB.
///
/// Логика:
/// 1. Случайная страница в диапазоне 1..=max_page (по popularity.desc).
/// 2. Из результатов берём случайный.
/// 3. Дозапрашиваем детали (runtime, episodes) для карточки.
///
/// Это даёт «честный рандом» из популярного пула — не топ-100,
/// но и не мусор с 1 голосом.
pub async fn discover_random(
    media_type: &str,
    user_api_key: Option<String>,
    locale: &str,
) -> Result<DiscoverItem, String> {
    let api_key = resolve_api_key(user_api_key);
    if api_key.is_empty() { return Err("TMDB key not set".to_string()); }

    let endpoint = match media_type {
        "tv" => "discover/tv",
        _ => "discover/movie",
    };
    let url = format!("{}/{}", TMDB_BASE, endpoint);
    let lang = resolve_tmdb_language(locale);

    let client = reqwest::Client::new();

    // 1. Первый запрос — узнаём total_pages и берём случайную страницу
    //    (сразу страницу 1, чтобы не делать 2 запроса)
    let first_resp = client.get(&url)
        .query(&[
            ("api_key", api_key.as_str()),
            ("language", lang),
            ("sort_by", "popularity.desc"),
            ("include_adult", "false"),
            ("page", "1"),
        ])
        .send().await.map_err(|e| e.to_string())?;

    if !first_resp.status().is_success() {
        return Err(format!("TMDB error: {}", first_resp.status()));
    }

    let first: TmdbDiscoverResponse = first_resp.json().await.map_err(|e| e.to_string())?;

    // TMDB реально отдаёт максимум 500 страниц, даже если total_pages больше
    let total_pages = first.total_pages.min(500).max(1);
    let random_page = (rand::random::<u32>() % total_pages as u32) + 1;

    // 2. Если random_page == 1 — используем уже полученный ответ,
    //    иначе делаем второй запрос
    let discover: TmdbDiscoverResponse = if random_page == 1 {
        first
    } else {
        let resp = client.get(&url)
            .query(&[
                ("api_key", api_key.as_str()),
                ("language", lang),
                ("sort_by", "popularity.desc"),
                ("include_adult", "false"),
                ("page", &random_page.to_string()),
            ])
            .send().await.map_err(|e| e.to_string())?;

        if !resp.status().is_success() {
            return Err(format!("TMDB error: {}", resp.status()));
        }
        resp.json().await.map_err(|e| e.to_string())?
    };

    if discover.results.is_empty() {
        return Err("TMDB вернул пустой список".to_string());
    }

    // 3. Случайный из результатов
    let idx = (rand::random::<u32>() as usize) % discover.results.len();
    let r = &discover.results[idx];

    let title = r.title.clone()
        .or(r.name.clone())
        .or(r.original_title.clone())
        .or(r.original_name.clone())
        .unwrap_or_else(|| "Unknown".to_string());
    let original_title = r.original_title.clone().or(r.original_name.clone());
    let date = r.release_date.clone().or(r.first_air_date.clone());
    let year = date.as_ref().and_then(|d| d.split('-').next()).and_then(|y| y.parse::<i64>().ok());
    let poster_url = r.poster_path.clone().map(|p| format!("{}{}", TMDB_IMAGE_BASE, p));

    // 4. Дозапрашиваем детали (runtime для фильма, episodes для сериала)
    let mut runtime: Option<i64> = None;
    let mut episodes: Option<i64> = None;
    let mut genres: Vec<String> = Vec::new();

    let details_endpoint = match media_type {
        "tv" => format!("tv/{}", r.id),
        _ => format!("movie/{}", r.id),
    };
    let details_url = format!("{}/{}", TMDB_BASE, details_endpoint);

    if let Ok(det_resp) = client.get(&details_url)
        .query(&[("api_key", api_key.as_str()), ("language", lang)])
        .send().await
    {
        if det_resp.status().is_success() {
            if let Ok(det) = det_resp.json::<serde_json::Value>().await {
                if media_type == "tv" {
                    episodes = det.get("number_of_episodes").and_then(|v| v.as_i64());
                } else {
                    runtime = det.get("runtime").and_then(|v| v.as_i64());
                }
                if let Some(gs) = det.get("genres").and_then(|v| v.as_array()) {
                    genres = gs.iter()
                        .filter_map(|g| g.get("name").and_then(|v| v.as_str()).map(|s| s.to_string()))
                        .collect();
                }
            }
        }
    }

    Ok(DiscoverItem {
        tmdb_id: r.id,
        title,
        original_title,
        overview: r.overview.clone(),
        poster_url,
        year,
        rating: r.vote_average,
        media_type: media_type.to_string(),
        runtime,
        episodes,
        genres,
    })
}