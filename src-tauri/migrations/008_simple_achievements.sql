CREATE TABLE IF NOT EXISTS title_metadata (
    title_id INTEGER PRIMARY KEY REFERENCES titles(id) ON DELETE CASCADE,
    cast_ids TEXT,          -- JSON-массив ID актёров
    director_ids TEXT,      -- JSON-массив ID режиссёров
    runtime INTEGER,
    release_year INTEGER
);