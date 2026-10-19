-- 017: Трекер серий для сериалов
-- Сезоны, эпизоды, отметки просмотра.

CREATE TABLE IF NOT EXISTS seasons (
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    title_id            INTEGER NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    tmdb_season_number  INTEGER NOT NULL,
    name                TEXT,
    episode_count       INTEGER,
    poster_path         TEXT,
    UNIQUE(title_id, tmdb_season_number)
);

CREATE TABLE IF NOT EXISTS episodes (
    id                    INTEGER PRIMARY KEY AUTOINCREMENT,
    season_id             INTEGER NOT NULL REFERENCES seasons(id) ON DELETE CASCADE,
    tmdb_episode_number   INTEGER NOT NULL,
    name                  TEXT,
    overview              TEXT,
    runtime               INTEGER,
    air_date              TEXT,
    still_path            TEXT,
    UNIQUE(season_id, tmdb_episode_number)
);

CREATE TABLE IF NOT EXISTS user_episodes (
    episode_id  INTEGER PRIMARY KEY REFERENCES episodes(id) ON DELETE CASCADE,
    watched_at  TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_seasons_title ON seasons(title_id);
CREATE INDEX IF NOT EXISTS idx_episodes_season ON episodes(season_id);
CREATE INDEX IF NOT EXISTS idx_user_episodes_ep ON user_episodes(episode_id);