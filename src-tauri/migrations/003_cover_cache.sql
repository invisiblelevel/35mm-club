-- Локальный кэш обложек
CREATE TABLE IF NOT EXISTS cover_cache (
    hash        TEXT PRIMARY KEY,
    url         TEXT NOT NULL,
    local_path  TEXT NOT NULL,
    size        INTEGER NOT NULL DEFAULT 0,
    cached_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE INDEX IF NOT EXISTS idx_cover_cache_cached_at ON cover_cache(cached_at);
CREATE INDEX IF NOT EXISTS idx_cover_cache_url ON cover_cache(url);