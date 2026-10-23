use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Title {
    pub id: i64,
    pub tmdb_id: Option<i64>,
    pub title: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub year: Option<i64>,
    pub poster_path: Option<String>,
    pub status: String,
    pub rating: Option<f64>,
    pub added_at: String,
}

#[derive(Debug, Deserialize)]
pub struct AddToLibraryPayload {
    pub tmdb_id: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub title: String,
    pub original_title: Option<String>,
    pub year: Option<i64>,
    pub poster_path: Option<String>,
    pub overview: Option<String>,
    pub imdb_rating: Option<f64>,
    pub runtime: Option<i64>,
    pub status: String,
    pub cast_ids: Option<Vec<i64>>,
    pub director_ids: Option<Vec<i64>>,
    pub collection_id: Option<i64>,
    pub producer_ids: Option<Vec<i64>>,
    pub production_company_ids: Option<Vec<i64>>,
    pub keyword_ids: Option<Vec<i64>>,
    pub locale: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddAnimePayload {
    pub mal_id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub cover_url: Option<String>,
    pub episodes: Option<i64>,
    pub format: Option<String>,
    pub status: String,
    pub studio: Option<String>,
    pub year: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub username: String,
    pub avatar_path: Option<String>,
    pub remote_id: Option<String>,
    pub level: i32,
    pub xp: i64,
    pub total_watch_minutes: i64,
    pub created_at: String,
    pub xp_to_next_level: i64,
    pub xp_in_current_level: i64,
    pub rank_name: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Achievement {
    pub id: i64,
    pub code: String,
    pub category: String,
    pub icon: Option<String>,
    pub xp_reward: i64,
    pub scope: String,
    pub tier: String,
    pub is_secret: bool,
    pub title_key: Option<String>,
    pub description_key: Option<String>,
    pub unlocked: bool,
    pub unlocked_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserTitleData {
    pub user_title_id: Option<i64>,
    pub status: Option<String>,
    pub rating: Option<f64>,
    pub notes: Option<String>,
    pub added_at: Option<String>,
    pub rewatch_count: i64,
}

#[derive(Debug, Serialize)]
pub struct EpisodeWithStatus {
    pub id: i64,
    pub episode_number: i64,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub runtime: Option<i64>,
    pub air_date: Option<String>,
    pub still_url: Option<String>,
    pub watched: bool,
}

#[derive(Debug, Serialize)]
pub struct DiaryEntry {
    pub id: i64,
    pub watched_at: String,
    pub rating: Option<f64>,
    pub notes: Option<String>,
    pub is_rewatch: bool,
    pub title: String,
    pub poster_url: Option<String>,
    pub type_field: String,
    pub tmdb_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Collection {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub item_count: i64,
    pub preview_posters: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct CollectionItem {
    pub user_title_id: i64,
    pub title: String,
    pub poster_url: Option<String>,
    pub type_field: String,
    pub tmdb_id: Option<i64>,
    pub year: Option<i64>,
    pub status: String,
}

#[derive(Debug, Serialize)]
pub struct LibraryDiaryEntry {
    pub user_title_id: i64,
    pub title: String,
    pub poster_url: Option<String>,
    pub type_field: String,
    pub tmdb_id: Option<i64>,
    pub status: String,
    pub rating: Option<f64>,
    pub notes: Option<String>,
    pub added_at: String,
}

#[derive(Debug, Serialize)]
pub struct RewatchEntry {
    pub id: i64,
    pub watched_at: String,
    pub rating: Option<f64>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct RewatchDiaryEntry {
    pub id: i64,
    pub watched_at: String,
    pub rating: Option<f64>,
    pub notes: Option<String>,
    pub title: String,
    pub poster_url: Option<String>,
    pub type_field: String,
    pub tmdb_id: Option<i64>,
}

// ==========================================
// СТАТИСТИКА
// ==========================================

#[derive(Debug, Serialize)]
pub struct StatsOverview {
    pub total_titles: i64,
    pub total_movies: i64,
    pub total_tv: i64,
    pub total_anime: i64,
    pub total_completed: i64,
    pub total_watching: i64,
    pub total_planned: i64,
    pub total_on_hold: i64,
    pub total_dropped: i64,
    pub total_minutes: i64,       // суммарное время просмотра (из user_profile)
    pub total_episodes: i64,      // отмеченные эпизоды сериалов + аниме
    pub total_rewatches: i64,     // пересмотры
    pub average_rating: Option<f64>,
    pub rated_count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsGenre {
    pub genre: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsYear {
    pub year: i64,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsRatingBucket {
    pub bucket: String,   // "0-1", "1-2", ..., "9-10"
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsTopTitle {
    pub title: String,
    pub poster_url: Option<String>,
    pub type_field: String,
    pub tmdb_id: Option<i64>,
    pub rating: Option<f64>,
    pub rewatch_count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsDecade {
    pub decade: String,   // "1950s", "1960s", ...
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsMonthly {
    pub month: String,    // "2026-01", "2026-02", ...
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct StatsFull {
    pub overview: StatsOverview,
    pub genres: Vec<StatsGenre>,
    pub years: Vec<StatsYear>,
    pub decades: Vec<StatsDecade>,
    pub ratings: Vec<StatsRatingBucket>,
    pub monthly: Vec<StatsMonthly>,
    pub weekdays: Vec<StatsWeekday>,   // ← новое
    pub top_rated: Vec<StatsTopTitle>,
    pub most_rewatched: Vec<StatsTopTitle>,
}

#[derive(Debug, Serialize)]
pub struct StatsWeekday {
    pub weekday: String,   // "0".."6" → "Пн".."Вс" на фронте
    pub count: i64,
}