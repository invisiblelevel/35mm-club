-- ============================================================
-- 35mm Club — Система ачивок, XP, званий и планок
-- Миграция 002
-- ============================================================

-- ------------------------------------------------------------
-- 1. user_profile — профиль пользователя (одна строка)
-- remote_id — задел под P2P-синхронизацию (пока NULL)
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS user_profile_new (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    username TEXT NOT NULL DEFAULT 'Киноман',
    avatar_path TEXT,
    remote_id TEXT,
    level INTEGER NOT NULL DEFAULT 1,
    xp INTEGER NOT NULL DEFAULT 0,
    total_watch_minutes INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT OR IGNORE INTO user_profile_new (id) VALUES (1);

DROP TABLE IF EXISTS user_profile;
ALTER TABLE user_profile_new RENAME TO user_profile;

-- ------------------------------------------------------------
-- 2. user_showcase — витрина ачивок
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS user_showcase (
    achievement_id INTEGER PRIMARY KEY REFERENCES achievements(id) ON DELETE CASCADE,
    position INTEGER NOT NULL
);

-- ------------------------------------------------------------
-- 3. Пересоздаём achievements с новыми полями
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS achievements_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    code TEXT NOT NULL UNIQUE,
    category TEXT NOT NULL,
    icon TEXT,
    xp_reward INTEGER NOT NULL DEFAULT 10,
    scope TEXT NOT NULL DEFAULT 'global',
    tier TEXT NOT NULL DEFAULT 'common',
    condition_type TEXT,
    condition_value INTEGER,
    is_secret INTEGER NOT NULL DEFAULT 0,
    title_key TEXT,
    description_key TEXT
);

DROP TABLE IF EXISTS achievements;
ALTER TABLE achievements_new RENAME TO achievements;

-- ------------------------------------------------------------
-- 4. Пересоздаём ribbons с новыми полями
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS ribbons_new (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tier INTEGER NOT NULL UNIQUE,
    name TEXT NOT NULL,
    description TEXT,
    required_count INTEGER NOT NULL,
    color1 TEXT DEFAULT '#ff9000',
    color2 TEXT DEFAULT '#cc7300',
    scope TEXT NOT NULL DEFAULT 'global',
    milestone INTEGER NOT NULL DEFAULT 0
);

DROP TABLE IF EXISTS ribbons;
ALTER TABLE ribbons_new RENAME TO ribbons;

-- ------------------------------------------------------------
-- 5. Seed: 5 базовых ачивок
-- ------------------------------------------------------------
INSERT OR IGNORE INTO achievements (code, category, icon, xp_reward, scope, tier, condition_type, condition_value, title_key, description_key)
VALUES
    ('first_movie', 'progress', 'Film', 10, 'movie', 'common', 'movies_count', 1, 'ach.first_movie.title', 'ach.first_movie.desc'),
    ('movies_10', 'progress', 'Film', 15, 'movie', 'common', 'movies_count', 10, 'ach.movies_10.title', 'ach.movies_10.desc'),
    ('movies_100', 'progress', 'Film', 50, 'movie', 'rare', 'movies_count', 100, 'ach.movies_100.title', 'ach.movies_100.desc'),
    ('tv_10', 'progress', 'Tv', 15, 'tv', 'common', 'tv_count', 10, 'ach.tv_10.title', 'ach.tv_10.desc'),
    ('anime_10', 'progress', 'Clapperboard', 15, 'anime', 'common', 'anime_count', 10, 'ach.anime_10.title', 'ach.anime_10.desc');

-- ------------------------------------------------------------
-- 6. Seed: 3 базовые планки
-- ------------------------------------------------------------
INSERT OR IGNORE INTO ribbons (tier, name, description, required_count, color1, color2, scope, milestone)
VALUES
    (1, 'Первая планка', 'За первые 25 ачивок', 25, '#ff9000', '#cc7300', 'global', 1),
    (2, 'Вторая планка', 'За 50 ачивок', 50, '#ffa733', '#cc7300', 'global', 0),
    (3, 'Третья планка', 'За 75 ачивок', 75, '#ff9000', '#8a4a00', 'global', 5);