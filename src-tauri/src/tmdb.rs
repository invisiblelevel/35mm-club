use serde::{Deserialize, Serialize};

const DEFAULT_TMDB_KEY: &str = "ВСТАВЬ_СВОЙ_КЛЮЧ_СЮДА";
const TMDB_BASE: &str = "https://api.themoviedb.org/3";
const TMDB_IMAGE_BASE: &str = "https://image.tmdb.org/t/p/w500";

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

fn resolve_api_key(user_key: Option<String>) -> String {
    match user_key {
        Some(k) if !k.trim().is_empty() => k.trim().to_string(),
        _ => DEFAULT_TMDB_KEY.to_string(),
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
    if api_key == "ВСТАВЬ_СВОЙ_КЛЮЧ_СЮДА" { return Err("TMDB key not set".to_string()); }

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
    if api_key == "ВСТАВЬ_СВОЙ_КЛЮЧ_СЮДА" { return Err("TMDB key not set".to_string()); }

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
    if api_key == "ВСТАВЬ_СВОЙ_КЛЮЧ_СЮДА" { return Err("TMDB key not set".to_string()); }

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
    if api_key == "ВСТАВЬ_СВОЙ_КЛЮЧ_СЮДА" { return Err("TMDB key not set".to_string()); }

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