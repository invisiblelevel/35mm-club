-- ============================================================
-- 35mm Club — Финальная инициализация базы (для релиза)
-- Включает все таблицы, колонки и ачивки.
-- ============================================================

-- ------------------------------------------------------------
-- 1. БАЗОВЫЕ ТАБЛИЦЫ КОНТЕНТА
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS titles (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    tmdb_id                 INTEGER,
    anilist_id              INTEGER,
    bangumi_id              INTEGER,
    type                    TEXT NOT NULL CHECK(type IN ('movie', 'tv', 'anime')),
    title                   TEXT NOT NULL,
    original_title          TEXT,
    year                    INTEGER,
    poster_path             TEXT,
    overview                TEXT,
    imdb_rating             REAL,
    runtime                 INTEGER,
    language                TEXT,
    episodes_total          INTEGER,
    anime_format            TEXT,
    anime_status            TEXT,
    studio                  TEXT,
    created_at              TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS title_metadata (
    title_id                INTEGER PRIMARY KEY REFERENCES titles(id) ON DELETE CASCADE,
    cast_ids                TEXT,
    director_ids            TEXT,
    producer_ids            TEXT,
    production_company_ids  TEXT,
    keyword_ids             TEXT,
    runtime                 INTEGER,
    release_year            INTEGER,
    collection_id           INTEGER
);

CREATE TABLE IF NOT EXISTS title_cast (
    title_id                INTEGER NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    person_id               INTEGER NOT NULL,
    PRIMARY KEY (title_id, person_id)
);

CREATE TABLE IF NOT EXISTS title_directors (
    title_id                INTEGER NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    person_id               INTEGER NOT NULL,
    PRIMARY KEY (title_id, person_id)
);

-- ------------------------------------------------------------
-- 2. БИБЛИОТЕКА И ПРОГРЕСС
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS user_titles (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    title_id                INTEGER NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    status                  TEXT NOT NULL CHECK(status IN ('watching', 'completed', 'planned', 'dropped', 'on_hold')),
    rating                  REAL,
    notes                   TEXT,
    started_at              TEXT,
    completed_at            TEXT,
    anime_episodes_watched  INTEGER NOT NULL DEFAULT 0,
    added_at                TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at              TEXT NOT NULL DEFAULT (datetime('now')),
    remote_user_id          TEXT,
    UNIQUE(title_id)
);

CREATE TABLE IF NOT EXISTS watch_entries (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    user_title_id           INTEGER NOT NULL REFERENCES user_titles(id) ON DELETE CASCADE,
    watched_at              TEXT NOT NULL,
    rating                  REAL,
    notes                   TEXT,
    is_rewatch              INTEGER NOT NULL DEFAULT 0
);

-- ------------------------------------------------------------
-- 3. КОЛЛЕКЦИИ
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS collections (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    name                    TEXT NOT NULL,
    description             TEXT,
    created_at              TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS collection_items (
    collection_id           INTEGER NOT NULL REFERENCES collections(id) ON DELETE CASCADE,
    user_title_id           INTEGER NOT NULL REFERENCES user_titles(id) ON DELETE CASCADE,
    added_at                TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (collection_id, user_title_id)
);

-- ------------------------------------------------------------
-- 4. ПРОФИЛЬ, XP, АЧИВКИ, ПЛАНКИ
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS user_profile (
    id                      INTEGER PRIMARY KEY CHECK (id = 1),
    username                TEXT NOT NULL DEFAULT 'Киноман',
    avatar_path             TEXT,
    remote_id               TEXT,
    level                   INTEGER NOT NULL DEFAULT 1,
    xp                      INTEGER NOT NULL DEFAULT 0,
    total_watch_minutes     INTEGER NOT NULL DEFAULT 0,
    created_at              TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at              TEXT NOT NULL DEFAULT (datetime('now'))
);

INSERT OR IGNORE INTO user_profile (id) VALUES (1);

CREATE TABLE IF NOT EXISTS achievements (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    code                    TEXT NOT NULL UNIQUE,
    category                TEXT NOT NULL,
    icon                    TEXT,
    xp_reward               INTEGER NOT NULL DEFAULT 10,
    scope                   TEXT NOT NULL DEFAULT 'global',
    tier                    TEXT NOT NULL DEFAULT 'common',
    condition_type          TEXT,
    condition_value         INTEGER,
    required_count          INTEGER NOT NULL DEFAULT 1,
    is_secret               INTEGER NOT NULL DEFAULT 0,
    title_key               TEXT,
    description_key         TEXT
);

CREATE TABLE IF NOT EXISTS user_achievements (
    achievement_id          INTEGER NOT NULL REFERENCES achievements(id) ON DELETE CASCADE,
    unlocked_at             TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at              TEXT NOT NULL DEFAULT (datetime('now')),
    remote_user_id          TEXT,
    PRIMARY KEY (achievement_id)
);

CREATE TABLE IF NOT EXISTS user_showcase (
    achievement_id          INTEGER PRIMARY KEY REFERENCES achievements(id) ON DELETE CASCADE,
    position                INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS ribbons (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    tier                    INTEGER NOT NULL UNIQUE,
    name                    TEXT NOT NULL,
    description             TEXT,
    required_count          INTEGER NOT NULL,
    color1                  TEXT DEFAULT '#ff9000',
    color2                  TEXT DEFAULT '#cc7300',
    scope                   TEXT NOT NULL DEFAULT 'global',
    milestone               INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS user_ribbons (
    ribbon_id               INTEGER NOT NULL REFERENCES ribbons(id) ON DELETE CASCADE,
    earned_at               TEXT NOT NULL DEFAULT (datetime('now')),
    PRIMARY KEY (ribbon_id)
);

-- ------------------------------------------------------------
-- 5. ТРЕКЕР СЕРИАЛОВ
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS seasons (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    title_id                INTEGER NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    tmdb_season_number      INTEGER NOT NULL,
    name                    TEXT,
    episode_count           INTEGER,
    poster_path             TEXT,
    UNIQUE(title_id, tmdb_season_number)
);

CREATE TABLE IF NOT EXISTS episodes (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    season_id               INTEGER NOT NULL REFERENCES seasons(id) ON DELETE CASCADE,
    tmdb_episode_number     INTEGER NOT NULL,
    name                    TEXT,
    overview                TEXT,
    runtime                 INTEGER,
    air_date                TEXT,
    still_path              TEXT,
    UNIQUE(season_id, tmdb_episode_number)
);

CREATE TABLE IF NOT EXISTS user_episodes (
    episode_id              INTEGER PRIMARY KEY REFERENCES episodes(id) ON DELETE CASCADE,
    watched_at              TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ------------------------------------------------------------
-- 6. СЕТЕВАЯ СИНХРОНИЗАЦИЯ (БУДУЩЕЕ)
-- sync_log создаётся в 002_sync.sql (новая структура).
-- ------------------------------------------------------------

CREATE TABLE IF NOT EXISTS auth_session (
    id                      INTEGER PRIMARY KEY CHECK (id = 1),
    remote_id               TEXT,
    email                   TEXT,
    access_token            TEXT,
    refresh_token           TEXT,
    expires_at              INTEGER,
    last_login_at           TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS friends (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    user_remote_id          TEXT NOT NULL,
    friend_remote_id        TEXT NOT NULL,
    status                  TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'accepted', 'blocked', 'declined')),
    created_at              TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at              TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(user_remote_id, friend_remote_id)
);

CREATE TABLE IF NOT EXISTS friend_requests (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    from_remote_id          TEXT NOT NULL,
    to_remote_id            TEXT NOT NULL,
    status                  TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'accepted', 'declined', 'cancelled')),
    message                 TEXT,
    created_at              TEXT NOT NULL DEFAULT (datetime('now')),
    responded_at            TEXT,
    UNIQUE(from_remote_id, to_remote_id)
);

CREATE TABLE IF NOT EXISTS notifications (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    remote_id               TEXT NOT NULL,
    type                    TEXT NOT NULL CHECK(type IN ('friend_request', 'friend_accepted', 'like', 'comment')),
    payload                 TEXT,
    is_read                 INTEGER NOT NULL DEFAULT 0,
    created_at              TEXT NOT NULL DEFAULT (datetime('now')),
    synced_at               TEXT
);

CREATE TABLE IF NOT EXISTS remote_profiles (
    remote_id               TEXT PRIMARY KEY,
    username                TEXT NOT NULL,
    avatar_url              TEXT,
    level                   INTEGER DEFAULT 1,
    xp                      INTEGER DEFAULT 0,
    total_watch_minutes     INTEGER DEFAULT 0,
    rank_name               TEXT,
    last_seen_at            TEXT,
    last_synced_at          TEXT NOT NULL DEFAULT (datetime('now'))
);

CREATE TABLE IF NOT EXISTS remote_user_titles (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    remote_user_id          TEXT NOT NULL,
    tmdb_id                 INTEGER NOT NULL,
    title                   TEXT NOT NULL,
    year                    INTEGER,
    poster_path             TEXT,
    status                  TEXT NOT NULL,
    user_rating             REAL,
    is_dirty                INTEGER NOT NULL DEFAULT 0,
    last_synced_at          TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(remote_user_id, tmdb_id)
);

CREATE TABLE IF NOT EXISTS remote_user_achievements (
    id                      INTEGER PRIMARY KEY AUTOINCREMENT,
    remote_user_id          TEXT NOT NULL,
    achievement_code        TEXT NOT NULL,
    unlocked_at             TEXT NOT NULL,
    is_dirty                INTEGER NOT NULL DEFAULT 0,
    last_synced_at          TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(remote_user_id, achievement_code)
);

CREATE TABLE IF NOT EXISTS sync_state (
    id                      INTEGER PRIMARY KEY CHECK (id = 1),
    last_profiles_sync      TEXT,
    last_friends_sync       TEXT,
    last_titles_sync        TEXT,
    last_achievements_sync  TEXT
);

INSERT OR IGNORE INTO sync_state (id) VALUES (1);

-- ------------------------------------------------------------
-- 7. ИНДЕКСЫ
-- (индексы на sync_log создаются в 002_sync.sql)
-- ------------------------------------------------------------

CREATE INDEX IF NOT EXISTS idx_user_titles_status ON user_titles(status);
CREATE INDEX IF NOT EXISTS idx_titles_type ON titles(type);
CREATE INDEX IF NOT EXISTS idx_titles_title ON titles(title);
CREATE INDEX IF NOT EXISTS idx_watch_entries_user_title ON watch_entries(user_title_id);
CREATE INDEX IF NOT EXISTS idx_seasons_title ON seasons(title_id);
CREATE INDEX IF NOT EXISTS idx_episodes_season ON episodes(season_id);
CREATE INDEX IF NOT EXISTS idx_user_episodes_ep ON user_episodes(episode_id);
CREATE INDEX IF NOT EXISTS idx_friends_user ON friends(user_remote_id, status);
CREATE INDEX IF NOT EXISTS idx_friends_friend ON friends(friend_remote_id, status);
CREATE INDEX IF NOT EXISTS idx_friend_requests_to ON friend_requests(to_remote_id, status);
CREATE INDEX IF NOT EXISTS idx_friend_requests_from ON friend_requests(from_remote_id, status);
CREATE INDEX IF NOT EXISTS idx_notifications_remote ON notifications(remote_id, is_read);
CREATE INDEX IF NOT EXISTS idx_remote_titles_user ON remote_user_titles(remote_user_id);
CREATE INDEX IF NOT EXISTS idx_remote_ach_user ON remote_user_achievements(remote_user_id);

-- ------------------------------------------------------------
-- 8. SEED АЧИВОК (ФИНАЛЬНОЕ СОСТОЯНИЕ)
-- ------------------------------------------------------------

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier, condition_type, condition_value, required_count, is_secret, title_key, description_key)
VALUES
    -- PROGRESS / COUNTS
    ('first_step', 'progress', 'game-icons:footprint', 10, 'global', 'common', 'total_count', 1, 1, 0, 'achievements.ach.first_step.title', 'achievements.ach.first_step.desc'),
    ('movie_count_10', 'movie', 'game-icons:popcorn', 40, 'global', 'common', 'movies_count', 10, 1, 0, 'achievements.ach.movie_count_10.title', 'achievements.ach.movie_count_10.desc'),
    ('movie_count_50', 'movie', 'game-icons:film-strip', 80, 'global', 'rare', 'movies_count', 50, 1, 0, 'achievements.ach.movie_count_50.title', 'achievements.ach.movie_count_50.desc'),
    ('movie_count_150', 'movie', 'game-icons:sofa', 150, 'global', 'epic', 'movies_count', 150, 1, 0, 'achievements.ach.movie_count_150.title', 'achievements.ach.movie_count_150.desc'),
    ('movie_count_300', 'movie', 'game-icons:crown', 250, 'global', 'epic', 'movies_count', 300, 1, 0, 'achievements.ach.movie_count_300.title', 'achievements.ach.movie_count_300.desc'),
    ('movie_count_500', 'movie', 'game-icons:film-projector', 400, 'global', 'legendary', 'movies_count', 500, 1, 0, 'achievements.ach.movie_count_500.title', 'achievements.ach.movie_count_500.desc'),
    ('movie_count_750', 'movie', 'game-icons:skull-crossed-bones', 600, 'global', 'legendary', 'movies_count', 750, 1, 0, 'achievements.ach.movie_count_750.title', 'achievements.ach.movie_count_750.desc'),
    ('movie_count_1000', 'movie', 'game-icons:circuitry', 1000, 'global', 'legendary', 'movies_count', 1000, 1, 0, 'achievements.ach.movie_count_1000.title', 'achievements.ach.movie_count_1000.desc'),
    ('movie_count_1500', 'movie', 'game-icons:film-spool', 2000, 'global', 'legendary', 'movies_count', 1500, 1, 0, 'achievements.ach.movie_count_1500.title', 'achievements.ach.movie_count_1500.desc'),
    
    ('tv_count_10', 'tv', 'game-icons:tv', 40, 'global', 'common', 'tv_count', 10, 1, 0, 'achievements.ach.tv_count_10.title', 'achievements.ach.tv_count_10.desc'),
    ('tv_count_50', 'tv', 'game-icons:popcorn', 80, 'global', 'rare', 'tv_count', 50, 1, 0, 'achievements.ach.tv_count_50.title', 'achievements.ach.tv_count_50.desc'),
    ('tv_count_150', 'tv', 'game-icons:knife-thrust', 150, 'global', 'epic', 'tv_count', 150, 1, 0, 'achievements.ach.tv_count_150.title', 'achievements.ach.tv_count_150.desc'),
    ('tv_count_300', 'tv', 'game-icons:sofa', 250, 'global', 'epic', 'tv_count', 300, 1, 0, 'achievements.ach.tv_count_300.title', 'achievements.ach.tv_count_300.desc'),
    ('tv_count_500', 'tv', 'game-icons:spy', 400, 'global', 'legendary', 'tv_count', 500, 1, 0, 'achievements.ach.tv_count_500.title', 'achievements.ach.tv_count_500.desc'),
    ('tv_count_750', 'tv', 'game-icons:beer-stein', 600, 'global', 'legendary', 'tv_count', 750, 1, 0, 'achievements.ach.tv_count_750.title', 'achievements.ach.tv_count_750.desc'),
    ('tv_count_1000', 'tv', 'game-icons:fast-forward-button', 1000, 'global', 'legendary', 'tv_count', 1000, 1, 0, 'achievements.ach.tv_count_1000.title', 'achievements.ach.tv_count_1000.desc'),
    ('tv_count_1500', 'tv', 'game-icons:tv', 2000, 'global', 'legendary', 'tv_count', 1500, 1, 0, 'achievements.ach.tv_count_1500.title', 'achievements.ach.tv_count_1500.desc'),
    
    ('anime_count_10', 'anime', 'game-icons:tv', 40, 'global', 'common', 'anime_count', 10, 1, 0, 'achievements.ach.anime_count_10.title', 'achievements.ach.anime_count_10.desc'),
    ('anime_count_50', 'anime', 'game-icons:book-cover', 80, 'global', 'rare', 'anime_count', 50, 1, 0, 'achievements.ach.anime_count_50.title', 'achievements.ach.anime_count_50.desc'),
    ('anime_count_150', 'anime', 'game-icons:katana', 150, 'global', 'epic', 'anime_count', 150, 1, 0, 'achievements.ach.anime_count_150.title', 'achievements.ach.anime_count_150.desc'),
    ('anime_count_300', 'anime', 'game-icons:owl', 250, 'global', 'epic', 'anime_count', 300, 1, 0, 'achievements.ach.anime_count_300.title', 'achievements.ach.anime_count_300.desc'),
    ('anime_count_500', 'anime', 'game-icons:katana', 400, 'global', 'legendary', 'anime_count', 500, 1, 0, 'achievements.ach.anime_count_500.title', 'achievements.ach.anime_count_500.desc'),
    ('anime_count_750', 'anime', 'game-icons:moon', 600, 'global', 'legendary', 'anime_count', 750, 1, 0, 'achievements.ach.anime_count_750.title', 'achievements.ach.anime_count_750.desc'),
    ('anime_count_1000', 'anime', 'game-icons:spell-book', 1000, 'global', 'legendary', 'anime_count', 1000, 1, 0, 'achievements.ach.anime_count_1000.title', 'achievements.ach.anime_count_1000.desc'),
    ('anime_count_1500', 'anime', 'game-icons:hand-of-god', 2000, 'global', 'legendary', 'anime_count', 1500, 1, 0, 'achievements.ach.anime_count_1500.title', 'achievements.ach.anime_count_1500.desc'),

    -- HUMOR (ФИЛЬМЫ)
    ('not_the_bees', 'humor', 'game-icons:bee', 50, 'global', 'rare', 'movie_id', 9708, 1, 0, 'achievements.ach.not_the_bees.title', 'achievements.ach.not_the_bees.desc'),
    ('nevsky_absolute', 'humor', 'game-icons:crossed-pistols', 30, 'global', 'rare', 'cast_or_producer_contains', 146402, 1, 0, 'achievements.ach.nevsky_absolute.title', 'achievements.ach.nevsky_absolute.desc'),
    ('room_wiseau', 'humor', 'game-icons:spoon', 40, 'global', 'rare', 'movie_id', 17473, 1, 0, 'achievements.ach.room_wiseau.title', 'achievements.ach.room_wiseau.desc'),
    ('green_elephant', 'humor', 'game-icons:elephant', 40, 'global', 'rare', 'movie_id', 132030, 1, 0, 'achievements.ach.green_elephant.title', 'achievements.ach.green_elephant.desc'),
    ('morbius_time', 'humor', 'game-icons:bat', 20, 'global', 'common', 'movie_id', 526896, 1, 0, 'achievements.ach.morbius_time.title', 'achievements.ach.morbius_time.desc'),
    ('cats_horrifying', 'humor', 'game-icons:cat', 20, 'global', 'common', 'movie_id', 536869, 1, 0, 'achievements.ach.cats_horrifying.title', 'achievements.ach.cats_horrifying.desc'),
    ('masochist_trash', 'humor', 'game-icons:trash-can', 40, 'global', 'rare', 'user_rating_with_low_tmdb', NULL, 1, 0, 'achievements.ach.masochist_trash.title', 'achievements.ach.masochist_trash.desc'),
    ('snob_critic', 'humor', 'game-icons:thumb-down', 30, 'global', 'rare', 'ratings_1', NULL, 5, 0, 'achievements.ach.snob_critic.title', 'achievements.ach.snob_critic.desc'),

    -- FRANCHISE / MARATHON (ФИЛЬМЫ)
    ('fast_family', 'franchise', 'game-icons:race-car', 60, 'global', 'legendary', 'collection_id', 9485, 10, 0, 'achievements.ach.fast_family.title', 'achievements.ach.fast_family.desc'),
    ('sharknado_marathon', 'franchise', 'game-icons:shark-fin', 60, 'global', 'legendary', 'collection_id', 286023, 6, 0, 'achievements.ach.sharknado_marathon.title', 'achievements.ach.sharknado_marathon.desc'),
    ('marvel_fatigue', 'franchise', 'game-icons:shield', 70, 'global', 'legendary', 'keyword_or_company', 180547, 30, 0, 'achievements.ach.marvel_fatigue.title', 'achievements.ach.marvel_fatigue.desc'),
    ('potterhead', 'franchise', 'game-icons:magic-swirl', 60, 'global', 'epic', 'collection_id', 1241, 8, 0, 'achievements.ach.potterhead.title', 'achievements.ach.potterhead.desc'),
    ('lotr_ringbearer', 'franchise', 'game-icons:ring', 50, 'global', 'epic', 'collection_id', 119, 3, 0, 'achievements.ach.lotr_ringbearer.title', 'achievements.ach.lotr_ringbearer.desc'),
    ('matrix_redpill', 'franchise', 'game-icons:pill', 50, 'global', 'epic', 'collection_id', 2344, 4, 0, 'achievements.ach.matrix_redpill.title', 'achievements.ach.matrix_redpill.desc'),
    ('saw_game', 'franchise', 'game-icons:puzzle', 60, 'global', 'epic', 'collection_id', 656, 10, 0, 'achievements.ach.saw_game.title', 'achievements.ach.saw_game.desc'),
    ('starwars_boomer', 'franchise', 'game-icons:light-sabers', 60, 'global', 'epic', 'collection_id', 10, 9, 0, 'achievements.ach.starwars_boomer.title', 'achievements.ach.starwars_boomer.desc'),
    ('shrek_swamp', 'franchise', 'game-icons:swamp', 40, 'global', 'rare', 'collection_id', 2150, 5, 0, 'achievements.ach.shrek_swamp.title', 'achievements.ach.shrek_swamp.desc'),
    ('alien_chestburster', 'franchise', 'game-icons:alien-bug', 50, 'global', 'epic', 'collection_id', 8091, 4, 0, 'achievements.ach.alien_chestburster.title', 'achievements.ach.alien_chestburster.desc'),
    ('terminator_fates', 'franchise', 'game-icons:robot-helmet', 50, 'global', 'epic', 'collection_id', 528, 6, 0, 'achievements.ach.terminator_fates.title', 'achievements.ach.terminator_fates.desc'),
    ('pirates_rum', 'franchise', 'game-icons:pirate-hat', 50, 'global', 'epic', 'collection_id', 295, 5, 0, 'achievements.ach.pirates_rum.title', 'achievements.ach.pirates_rum.desc'),
    ('transformers_bayhem', 'franchise', 'game-icons:robot-golem', 50, 'global', 'epic', 'collection_id', 8650, 5, 0, 'achievements.ach.transformers_bayhem.title', 'achievements.ach.transformers_bayhem.desc'),
    ('john_wick_pencil', 'franchise', 'game-icons:pencil', 50, 'global', 'epic', 'collection_id', 404609, 4, 0, 'achievements.ach.john_wick_pencil.title', 'achievements.ach.john_wick_pencil.desc'),
    ('resident_evil_milla', 'franchise', 'game-icons:biohazard', 50, 'global', 'epic', 'collection_id', 17255, 6, 0, 'achievements.ach.resident_evil_milla.title', 'achievements.ach.resident_evil_milla.desc'),
    ('twilight_vampire', 'franchise', 'game-icons:vampire-dracula', 40, 'global', 'rare', 'collection_id', 33514, 5, 0, 'achievements.ach.twilight_vampire.title', 'achievements.ach.twilight_vampire.desc'),
    ('final_destination', 'franchise', 'game-icons:truck', 50, 'global', 'epic', 'collection_id', 8864, 7, 0, 'achievements.ach.final_destination.title', 'achievements.ach.final_destination.desc'),
    ('godfather_offer', 'franchise', 'game-icons:spiked-collar', 60, 'global', 'epic', 'collection_id', 230, 3, 0, 'achievements.ach.godfather_offer.title', 'achievements.ach.godfather_offer.desc'),
    ('mission_impossible_stunt', 'franchise', 'game-icons:spy', 60, 'global', 'epic', 'collection_id', 87359, 8, 0, 'achievements.ach.mission_impossible_stunt.title', 'achievements.ach.mission_impossible_stunt.desc'),
    ('hunger_games_mockingjay', 'franchise', 'game-icons:bow-arrow', 50, 'global', 'epic', 'collection_id', 131635, 4, 0, 'achievements.ach.hunger_games_mockingjay.title', 'achievements.ach.hunger_games_mockingjay.desc'),
    ('halloween_myers', 'franchise', 'game-icons:thrown-knife', 60, 'global', 'epic', 'collection_id', 91361, 11, 0, 'achievements.ach.halloween_myers.title', 'achievements.ach.halloween_myers.desc'),
    ('ocean_eleven_heist', 'franchise', 'game-icons:diamonds', 50, 'global', 'epic', 'collection_id', 304, 3, 0, 'achievements.ach.ocean_eleven_heist.title', 'achievements.ach.ocean_eleven_heist.desc'),
    ('madagascar_smile_and_wave', 'franchise', 'game-icons:lion', 40, 'global', 'rare', 'collection_id', 14740, 3, 0, 'achievements.ach.madagascar_smile_and_wave.title', 'achievements.ach.madagascar_smile_and_wave.desc'),
    ('scream_ghostface', 'franchise', 'game-icons:knife-thrust', 60, 'global', 'epic', 'collection_id', 2602, 8, 0, 'achievements.ach.scream_ghostface.title', 'achievements.ach.scream_ghostface.desc'),
    ('predator_hunter', 'franchise', 'game-icons:hunter-eyes', 60, 'global', 'epic', 'collection_id', 399, 6, 0, 'achievements.ach.predator_hunter.title', 'achievements.ach.predator_hunter.desc'),
    ('evil_dead_groovy', 'franchise', 'game-icons:chainsaw', 50, 'global', 'epic', 'collection_id', 1960, 3, 0, 'achievements.ach.evil_dead_groovy.title', 'achievements.ach.evil_dead_groovy.desc'),
    ('ghostbusters_proton', 'franchise', 'game-icons:ghost', 50, 'global', 'epic', 'collection_id', 2980, 4, 0, 'achievements.ach.ghostbusters_proton.title', 'achievements.ach.ghostbusters_proton.desc'),
    ('rocky_balboa_eye', 'franchise', 'game-icons:boxing-glove', 60, 'global', 'epic', 'collection_id', 1575, 6, 0, 'achievements.ach.rocky_balboa_eye.title', 'achievements.ach.rocky_balboa_eye.desc'),
    ('creed_legacy', 'franchise', 'game-icons:fist', 50, 'global', 'epic', 'collection_id', 553717, 4, 0, 'achievements.ach.creed_legacy.title', 'achievements.ach.creed_legacy.desc'),
    ('conjuring_universe', 'franchise', 'game-icons:gothic-cross', 60, 'global', 'epic', 'collection_id', 313086, 4, 0, 'achievements.ach.conjuring_universe.title', 'achievements.ach.conjuring_universe.desc'),
    ('planet_of_apes', 'franchise', 'game-icons:monkey', 50, 'global', 'epic', 'collection_id', 173710, 4, 0, 'achievements.ach.planet_of_apes.title', 'achievements.ach.planet_of_apes.desc'),
    ('kingsman_manners', 'franchise', 'game-icons:umbrella', 50, 'global', 'epic', 'collection_id', 391860, 3, 0, 'achievements.ach.kingsman_manners.title', 'achievements.ach.kingsman_manners.desc'),
	-- 20 фильмовых ачивок (пачка 1)
    ('movie_bond', 'franchise', 'game-icons:martini', 80, 'global', 'legendary', 'collection_id', 645, 27, 0, 'achievements.ach.movie_bond.title', 'achievements.ach.movie_bond.desc'),
    ('movie_avatar', 'franchise', 'game-icons:alien-bug', 80, 'global', 'epic', 'collection_id', 87096, 2, 0, 'achievements.ach.movie_avatar.title', 'achievements.ach.movie_avatar.desc'),
    ('movie_brat', 'franchise', 'game-icons:uzi', 60, 'global', 'epic', 'collection_id', 218293, 2, 0, 'achievements.ach.movie_brat.title', 'achievements.ach.movie_brat.desc'),
    ('movie_xmen', 'franchise', 'game-icons:crossed-swords', 80, 'global', 'epic', 'collection_id', 748, 13, 0, 'achievements.ach.movie_xmen.title', 'achievements.ach.movie_xmen.desc'),
    ('movie_taken', 'franchise', 'game-icons:phone', 60, 'global', 'epic', 'collection_id', 135483, 3, 0, 'achievements.ach.movie_taken.title', 'achievements.ach.movie_taken.desc'),
    ('movie_bumer', 'franchise', 'game-icons:car-key', 60, 'global', 'epic', 'collection_id', 107461, 2, 0, 'achievements.ach.movie_bumer.title', 'achievements.ach.movie_bumer.desc'),
    ('movie_home_alone', 'franchise', 'game-icons:christmas-tree', 50, 'global', 'rare', 'collection_id', 9888, 4, 0, 'achievements.ach.movie_home_alone.title', 'achievements.ach.movie_home_alone.desc'),
    ('movie_wrong_turn', 'franchise', 'game-icons:axe-in-stump', 60, 'global', 'epic', 'collection_id', 52985, 6, 0, 'achievements.ach.movie_wrong_turn.title', 'achievements.ach.movie_wrong_turn.desc'),
    ('movie_red', 'franchise', 'game-icons:revolver', 60, 'global', 'epic', 'collection_id', 163902, 2, 0, 'achievements.ach.movie_red.title', 'achievements.ach.movie_red.desc'),
    ('movie_jaws', 'franchise', 'game-icons:shark-fin', 70, 'global', 'epic', 'collection_id', 2366, 4, 0, 'achievements.ach.movie_jaws.title', 'achievements.ach.movie_jaws.desc'),
    ('movie_kick_ass', 'franchise', 'game-icons:knife-thrust', 60, 'global', 'epic', 'collection_id', 179892, 2, 0, 'achievements.ach.movie_kick_ass.title', 'achievements.ach.movie_kick_ass.desc'),
    ('movie_riddick', 'franchise', 'game-icons:knife-thrust', 70, 'global', 'epic', 'collection_id', 2794, 3, 0, 'achievements.ach.movie_riddick.title', 'achievements.ach.movie_riddick.desc'),
    ('movie_28_days', 'franchise', 'game-icons:shambling-zombie', 70, 'global', 'epic', 'collection_id', 1565, 3, 0, 'achievements.ach.movie_28_days.title', 'achievements.ach.movie_28_days.desc'),
    ('movie_jurassic', 'franchise', 'game-icons:dinosaur', 80, 'global', 'legendary', 'collection_id', 328, 7, 0, 'achievements.ach.movie_jurassic.title', 'achievements.ach.movie_jurassic.desc'),
    ('movie_jeepers', 'franchise', 'game-icons:bat', 50, 'global', 'rare', 'collection_id', 94899, 3, 0, 'achievements.ach.movie_jeepers.title', 'achievements.ach.movie_jeepers.desc'),
    ('movie_bttf', 'franchise', 'game-icons:time-trap', 80, 'global', 'legendary', 'collection_id', 264, 3, 0, 'achievements.ach.movie_bttf.title', 'achievements.ach.movie_bttf.desc'),
    ('movie_police_academy', 'franchise', 'game-icons:police-officer-head', 50, 'global', 'rare', 'collection_id', 9338, 7, 0, 'achievements.ach.movie_police_academy.title', 'achievements.ach.movie_police_academy.desc'),
    ('movie_nightmare', 'franchise', 'game-icons:claws', 70, 'global', 'epic', 'collection_id', 8581, 7, 0, 'achievements.ach.movie_nightmare.title', 'achievements.ach.movie_nightmare.desc'),
    ('movie_hobbit', 'franchise', 'game-icons:dragon-head', 70, 'global', 'epic', 'collection_id', 121938, 3, 0, 'achievements.ach.movie_hobbit.title', 'achievements.ach.movie_hobbit.desc'),
    ('movie_die_hard', 'franchise', 'game-icons:skyscraper', 80, 'global', 'legendary', 'collection_id', 1570, 5, 0, 'achievements.ach.movie_die_hard.title', 'achievements.ach.movie_die_hard.desc'),
	    -- 20 ачивок «Подвалы TMDb и Психиатрия»
    ('movie_gruz_200', 'humor', 'game-icons:coffin', 60, 'global', 'epic', 'movie_id', 34869, 1, 0, 'achievements.ach.movie_gruz_200.title', 'achievements.ach.movie_gruz_200.desc'),
    ('movie_human_centipede', 'humor', 'game-icons:centipede', 70, 'global', 'epic', 'movie_id', 37169, 1, 0, 'achievements.ach.movie_human_centipede.title', 'achievements.ach.movie_human_centipede.desc'),
    ('movie_salo', 'humor', 'game-icons:feces', 70, 'global', 'epic', 'movie_id', 5336, 1, 0, 'achievements.ach.movie_salo.title', 'achievements.ach.movie_salo.desc'),
    ('movie_serbian_film', 'humor', 'game-icons:blood', 80, 'global', 'legendary', 'movie_id', 73861, 1, 0, 'achievements.ach.movie_serbian_film.title', 'achievements.ach.movie_serbian_film.desc'),
    ('movie_eraserhead', 'humor', 'game-icons:brain', 60, 'global', 'epic', 'movie_id', 985, 1, 0, 'achievements.ach.movie_eraserhead.title', 'achievements.ach.movie_eraserhead.desc'),
    ('movie_pink_flamingos', 'humor', 'game-icons:flamingo', 70, 'global', 'epic', 'movie_id', 692, 1, 0, 'achievements.ach.movie_pink_flamingos.title', 'achievements.ach.movie_pink_flamingos.desc'),
    ('movie_antichrist', 'humor', 'game-icons:deer', 70, 'global', 'epic', 'movie_id', 17609, 1, 0, 'achievements.ach.movie_antichrist.title', 'achievements.ach.movie_antichrist.desc'),
    ('movie_das_experiment', 'humor', 'game-icons:prisoner', 60, 'global', 'epic', 'movie_id', 575, 1, 0, 'achievements.ach.movie_das_experiment.title', 'achievements.ach.movie_das_experiment.desc'),
    ('movie_dogtooth', 'humor', 'game-icons:tooth', 60, 'global', 'epic', 'movie_id', 38810, 1, 0, 'achievements.ach.movie_dogtooth.title', 'achievements.ach.movie_dogtooth.desc'),
    ('movie_under_the_skin', 'humor', 'game-icons:cloak', 60, 'global', 'epic', 'movie_id', 97370, 1, 0, 'achievements.ach.movie_under_the_skin.title', 'achievements.ach.movie_under_the_skin.desc'),
    ('movie_holy_mountain', 'humor', 'game-icons:pyramid', 70, 'global', 'epic', 'movie_id', 8327, 1, 0, 'achievements.ach.movie_holy_mountain.title', 'achievements.ach.movie_holy_mountain.desc'),
    ('movie_twin_peaks_fire', 'humor', 'game-icons:owl', 70, 'global', 'epic', 'movie_id', 1923, 1, 0, 'achievements.ach.movie_twin_peaks_fire.title', 'achievements.ach.movie_twin_peaks_fire.desc'),
    ('movie_pi', 'humor', 'game-icons:pi-symbol', 60, 'global', 'epic', 'movie_id', 473, 1, 0, 'achievements.ach.movie_pi.title', 'achievements.ach.movie_pi.desc'),
    ('movie_dead_man', 'humor', 'game-icons:cowboy-hat', 60, 'global', 'epic', 'movie_id', 922, 1, 0, 'achievements.ach.movie_dead_man.title', 'achievements.ach.movie_dead_man.desc'),
    ('movie_sexual_parasite', 'humor', 'game-icons:parasite', 80, 'global', 'legendary', 'movie_id', 45676, 1, 0, 'achievements.ach.movie_sexual_parasite.title', 'achievements.ach.movie_sexual_parasite.desc'),
    ('movie_cannibal_holocaust', 'humor', 'game-icons:skull-crossed-bones', 80, 'global', 'legendary', 'movie_id', 8689, 1, 0, 'achievements.ach.movie_cannibal_holocaust.title', 'achievements.ach.movie_cannibal_holocaust.desc'),
    ('movie_hard_to_be_god', 'humor', 'game-icons:chess-king', 70, 'global', 'epic', 'movie_id', 64508, 1, 0, 'achievements.ach.movie_hard_to_be_god.title', 'achievements.ach.movie_hard_to_be_god.desc'),
    ('movie_five_bottles', 'humor', 'game-icons:vodka', 60, 'global', 'epic', 'movie_id', 211807, 1, 0, 'achievements.ach.movie_five_bottles.title', 'achievements.ach.movie_five_bottles.desc'),
    ('movie_toxic_avenger', 'humor', 'game-icons:radioactive', 70, 'global', 'epic', 'movie_id', 15239, 1, 0, 'achievements.ach.movie_toxic_avenger.title', 'achievements.ach.movie_toxic_avenger.desc'),
    ('movie_blood_feast', 'humor', 'game-icons:meat', 70, 'global', 'epic', 'movie_id', 28172, 1, 0, 'achievements.ach.movie_blood_feast.title', 'achievements.ach.movie_blood_feast.desc'),

    -- DIRECTOR / ACTOR / TIME
    ('nolan_time', 'director', 'game-icons:clockwork', 80, 'global', 'legendary', 'director_contains', 525, 10, 0, 'achievements.ach.nolan_time.title', 'achievements.ach.nolan_time.desc'),
    ('tarantino_feet', 'director', 'game-icons:high-heel', 80, 'global', 'legendary', 'director_contains', 138, 9, 0, 'achievements.ach.tarantino_feet.title', 'achievements.ach.tarantino_feet.desc'),
    ('cage_fever', 'actor', 'game-icons:cage', 50, 'global', 'rare', 'cast_contains', 2963, 10, 0, 'achievements.ach.cage_fever.title', 'achievements.ach.cage_fever.desc'),
    ('uwe_boll_survivor', 'director', 'game-icons:boxing-glove', 50, 'global', 'legendary', 'director_contains', 23799, 3, 0, 'achievements.ach.uwe_boll_survivor.title', 'achievements.ach.uwe_boll_survivor.desc'),
    ('speedrunner', 'time', 'game-icons:run', 40, 'global', 'rare', 'daily_completed', 5, 1, 0, 'achievements.ach.speedrunner.title', 'achievements.ach.speedrunner.desc'),
    ('night_owl', 'time', 'game-icons:owl', 30, 'global', 'rare', 'night_watch', NULL, 1, 0, 'achievements.ach.night_owl.title', 'achievements.ach.night_owl.desc'),
    ('marathon_endurance', 'time', 'game-icons:armchair', 30, 'global', 'rare', 'runtime_gt', 200, 1, 0, 'achievements.ach.marathon_endurance.title', 'achievements.ach.marathon_endurance.desc'),
    ('ancient_fossil', 'time', 'game-icons:fossil', 30, 'global', 'rare', 'year_lt', 1950, 1, 0, 'achievements.ach.ancient_fossil.title', 'achievements.ach.ancient_fossil.desc'),
    ('short_and_sweet', 'time', 'game-icons:hourglass', 20, 'global', 'common', 'runtime_lt', 70, 1, 0, 'achievements.ach.short_and_sweet.title', 'achievements.ach.short_and_sweet.desc'),

    -- TV (СЕРИАЛЫ)
    ('tv_firefly_browncoat', 'franchise', 'game-icons:spaceship', 60, 'global', 'epic', 'tv_episodes_completed', 1433, 1, 0, 'achievements.ach.tv_firefly_browncoat.title', 'achievements.ach.tv_firefly_browncoat.desc'),
    ('tv_walking_dead_coral', 'franchise', 'game-icons:shambling-zombie', 80, 'global', 'legendary', 'tv_episodes_completed', 1402, 1, 0, 'achievements.ach.tv_walking_dead_coral.title', 'achievements.ach.tv_walking_dead_coral.desc'),
    ('tv_lost_numbers', 'franchise', 'game-icons:secret-book', 70, 'global', 'legendary', 'tv_episodes_completed', 4607, 1, 0, 'achievements.ach.tv_lost_numbers.title', 'achievements.ach.tv_lost_numbers.desc'),
    ('tv_prison_break_tattoo', 'franchise', 'game-icons:prisoner', 60, 'global', 'epic', 'tv_episodes_completed', 2288, 1, 0, 'achievements.ach.tv_prison_break_tattoo.title', 'achievements.ach.tv_prison_break_tattoo.desc'),
    ('tv_breaking_bad_cook', 'franchise', 'game-icons:test-tubes', 80, 'global', 'legendary', 'tv_episodes_completed', 1396, 1, 0, 'achievements.ach.tv_breaking_bad_cook.title', 'achievements.ach.tv_breaking_bad_cook.desc'),
    ('tv_got_coffee_cup', 'franchise', 'game-icons:coffee-cup', 80, 'global', 'legendary', 'tv_episodes_completed', 1399, 1, 0, 'achievements.ach.tv_got_coffee_cup.title', 'achievements.ach.tv_got_coffee_cup.desc'),
    ('tv_supernatural_pie', 'franchise', 'game-icons:race-car', 80, 'global', 'legendary', 'tv_episodes_completed', 1622, 1, 0, 'achievements.ach.tv_supernatural_pie.title', 'achievements.ach.tv_supernatural_pie.desc'),
    ('tv_office_that_what_she_said', 'franchise', 'game-icons:office-chair', 60, 'global', 'epic', 'tv_episodes_completed', 2316, 1, 0, 'achievements.ach.tv_office_that_what_she_said.title', 'achievements.ach.tv_office_that_what_she_said.desc'),
    ('tv_x_files_truth', 'franchise', 'game-icons:ufo', 70, 'global', 'epic', 'tv_episodes_completed', 4087, 1, 0, 'achievements.ach.tv_x_files_truth.title', 'achievements.ach.tv_x_files_truth.desc'),
    ('tv_sherlock_sociopath', 'franchise', 'game-icons:magnifying-glass', 60, 'global', 'epic', 'tv_episodes_completed', 19885, 1, 0, 'achievements.ach.tv_sherlock_sociopath.title', 'achievements.ach.tv_sherlock_sociopath.desc'),
    ('tv_sopranos_duck', 'franchise', 'game-icons:duck', 70, 'global', 'legendary', 'tv_episodes_completed', 1398, 1, 0, 'achievements.ach.tv_sopranos_duck.title', 'achievements.ach.tv_sopranos_duck.desc'),
    ('tv_twin_peaks_pie', 'franchise', 'game-icons:cherry', 60, 'global', 'legendary', 'tv_episodes_completed', 1920, 1, 0, 'achievements.ach.tv_twin_peaks_pie.title', 'achievements.ach.tv_twin_peaks_pie.desc'),
    ('tv_friends_pivot', 'franchise', 'game-icons:sofa', 60, 'global', 'legendary', 'tv_episodes_completed', 1668, 1, 0, 'achievements.ach.tv_friends_pivot.title', 'achievements.ach.tv_friends_pivot.desc'),
    ('tv_boys_compound_v', 'franchise', 'game-icons:syringe', 60, 'global', 'epic', 'tv_episodes_completed', 76479, 1, 0, 'achievements.ach.tv_boys_compound_v.title', 'achievements.ach.tv_boys_compound_v.desc'),
    ('tv_black_mirror_screen', 'franchise', 'game-icons:smartphone', 60, 'global', 'epic', 'tv_episodes_completed', 42009, 1, 0, 'achievements.ach.tv_black_mirror_screen.title', 'achievements.ach.tv_black_mirror_screen.desc'),
    ('tv_mandalorian_way', 'franchise', 'game-icons:helmet', 60, 'global', 'epic', 'tv_episodes_completed', 82856, 1, 0, 'achievements.ach.tv_mandalorian_way.title', 'achievements.ach.tv_mandalorian_way.desc'),
    ('tv_better_call_saul_lawyer', 'franchise', 'game-icons:scales', 80, 'global', 'epic', 'tv_episodes_completed', 60059, 1, 0, 'achievements.ach.tv_better_call_saul_lawyer.title', 'achievements.ach.tv_better_call_saul_lawyer.desc'),
    ('tv_dexter_plastic_wrap', 'franchise', 'game-icons:blood', 80, 'global', 'epic', 'tv_episodes_completed', 1405, 1, 0, 'achievements.ach.tv_dexter_plastic_wrap.title', 'achievements.ach.tv_dexter_plastic_wrap.desc'),
    ('tv_peaky_blinders_by_order', 'franchise', 'game-icons:outback-hat', 70, 'global', 'epic', 'tv_episodes_completed', 60574, 1, 0, 'achievements.ach.tv_peaky_blinders_by_order.title', 'achievements.ach.tv_peaky_blinders_by_order.desc'),
    ('tv_chernobyl_not_great', 'franchise', 'game-icons:nuclear', 60, 'global', 'epic', 'tv_episodes_completed', 87108, 1, 0, 'achievements.ach.tv_chernobyl_not_great.title', 'achievements.ach.tv_chernobyl_not_great.desc'),
    ('tv_stranger_things_running_up', 'franchise', 'game-icons:walkie-talkie', 70, 'global', 'epic', 'tv_episodes_completed', 66732, 1, 0, 'achievements.ach.tv_stranger_things_running_up.title', 'achievements.ach.tv_stranger_things_running_up.desc'),
    ('tv_silicon_valley_middle_out', 'franchise', 'game-icons:processor', 60, 'global', 'epic', 'tv_episodes_completed', 60573, 1, 0, 'achievements.ach.tv_silicon_valley_middle_out.title', 'achievements.ach.tv_silicon_valley_middle_out.desc'),
    ('tv_fargo_ok_then', 'franchise', 'game-icons:flake', 70, 'global', 'epic', 'tv_episodes_completed', 60622, 1, 0, 'achievements.ach.tv_fargo_ok_then.title', 'achievements.ach.tv_fargo_ok_then.desc'),
    ('tv_arcane_league', 'franchise', 'game-icons:magic-potion', 60, 'global', 'epic', 'tv_episodes_completed', 94605, 1, 0, 'achievements.ach.tv_arcane_league.title', 'achievements.ach.tv_arcane_league.desc'),

    -- ANIME
    ('anime_konosuba_party', 'anime', 'game-icons:beer-stein', 80, 'global', 'epic', 'anime_franchise_completed', 30831, 1, 0, 'achievements.ach.anime_konosuba_party.title', 'achievements.ach.anime_konosuba_party.desc'),
    ('anime_pingpong_club', 'anime', 'game-icons:ping-pong-bat', 50, 'global', 'rare', 'anime_episodes_completed', 914, 1, 0, 'achievements.ach.anime_pingpong_club.title', 'achievements.ach.anime_pingpong_club.desc'),
    ('anime_prison_school', 'anime', 'game-icons:handcuffs', 60, 'global', 'epic', 'anime_franchise_completed', 30240, 2, 0, 'achievements.ach.anime_prison_school.title', 'achievements.ach.anime_prison_school.desc'),
    ('anime_redo_healer', 'anime', 'game-icons:healing', 60, 'global', 'epic', 'anime_episodes_completed', 40750, 1, 0, 'achievements.ach.anime_redo_healer.title', 'achievements.ach.anime_redo_healer.desc'),
    ('anime_death_note', 'anime', 'game-icons:notebook', 70, 'global', 'epic', 'anime_episodes_completed', 1535, 1, 0, 'achievements.ach.anime_death_note.title', 'achievements.ach.anime_death_note.desc'),
    ('anime_nagatoro', 'anime', 'game-icons:heart-plus', 50, 'global', 'rare', 'anime_episodes_completed', 42361, 1, 0, 'achievements.ach.anime_nagatoro.title', 'achievements.ach.anime_nagatoro.desc'),
    ('anime_elfen_lied', 'anime', 'game-icons:blood', 60, 'global', 'epic', 'anime_episodes_completed', 1836, 1, 0, 'achievements.ach.anime_elfen_lied.title', 'achievements.ach.anime_elfen_lied.desc'),
    ('anime_higurashi', 'anime', 'game-icons:cricket', 60, 'global', 'epic', 'anime_episodes_completed', 934, 1, 0, 'achievements.ach.anime_higurashi.title', 'achievements.ach.anime_higurashi.desc'),
    ('anime_chainsaw_man', 'anime', 'game-icons:chainsaw', 60, 'global', 'epic', 'anime_episodes_completed', 44511, 1, 0, 'achievements.ach.anime_chainsaw_man.title', 'achievements.ach.anime_chainsaw_man.desc'),
    ('anime_black_lagoon', 'anime', 'game-icons:revolver', 70, 'global', 'epic', 'anime_franchise_completed', 889, 2, 0, 'achievements.ach.anime_black_lagoon.title', 'achievements.ach.anime_black_lagoon.desc'),
    ('anime_naruto_legend', 'anime', 'game-icons:ninja-mask', 200, 'global', 'legendary', 'anime_franchise_completed', 20, 2, 0, 'achievements.ach.anime_naruto_legend.title', 'achievements.ach.anime_naruto_legend.desc'),
    ('anime_yani_neko', 'anime', 'game-icons:cat', 40, 'global', 'rare', 'anime_episodes_completed', 63403, 1, 0, 'achievements.ach.anime_yani_neko.title', 'achievements.ach.anime_yani_neko.desc'),
    ('anime_gurren_lagann', 'anime', 'game-icons:drill', 70, 'global', 'epic', 'anime_episodes_completed', 2001, 1, 0, 'achievements.ach.anime_gurren_lagann.title', 'achievements.ach.anime_gurren_lagann.desc'),
    ('anime_toradora', 'anime', 'game-icons:tiger', 60, 'global', 'epic', 'anime_episodes_completed', 4224, 1, 0, 'achievements.ach.anime_toradora.title', 'achievements.ach.anime_toradora.desc'),
    ('anime_chi_sweet_home', 'anime', 'game-icons:cat', 50, 'global', 'rare', 'anime_episodes_completed', 3841, 1, 0, 'achievements.ach.anime_chi_sweet_home.title', 'achievements.ach.anime_chi_sweet_home.desc'),
    ('anime_made_in_abyss', 'anime', 'game-icons:hole', 80, 'global', 'epic', 'anime_franchise_completed', 34599, 2, 0, 'achievements.ach.anime_made_in_abyss.title', 'achievements.ach.anime_made_in_abyss.desc'),
    ('anime_mushoku_tensei', 'anime', 'game-icons:baby-face', 90, 'global', 'epic', 'anime_franchise_completed', 39535, 2, 0, 'achievements.ach.anime_mushoku_tensei.title', 'achievements.ach.anime_mushoku_tensei.desc'),
    ('anime_fullmetal_alchemist', 'anime', 'game-icons:transfuse', 150, 'global', 'legendary', 'anime_episodes_completed', 5114, 1, 0, 'achievements.ach.anime_fullmetal_alchemist.title', 'achievements.ach.anime_fullmetal_alchemist.desc'),
    ('anime_bleach', 'anime', 'game-icons:katana', 200, 'global', 'legendary', 'anime_episodes_completed', 269, 1, 0, 'achievements.ach.anime_bleach.title', 'achievements.ach.anime_bleach.desc'),
    ('anime_sword_art_online', 'anime', 'game-icons:crossed-swords', 80, 'global', 'epic', 'anime_episodes_completed', 11757, 1, 0, 'achievements.ach.anime_sword_art_online.title', 'achievements.ach.anime_sword_art_online.desc'),
    ('anime_berserk', 'anime', 'game-icons:sword-wound', 120, 'global', 'epic', 'anime_episodes_completed', 33, 1, 0, 'achievements.ach.anime_berserk.title', 'achievements.ach.anime_berserk.desc'),
	
	    -- Ачивки за списки
    ('status_planned_10', 'progress', 'game-icons:clock', 30, 'global', 'common', 'status_planned', 10, 1, 0, 'achievements.ach.status_planned_10.title', 'achievements.ach.status_planned_10.desc'),
    ('status_planned_50', 'progress', 'game-icons:calendar', 80, 'global', 'rare', 'status_planned', 50, 1, 0, 'achievements.ach.status_planned_50.title', 'achievements.ach.status_planned_50.desc'),
    ('status_planned_100', 'progress', 'game-icons:scroll-quill', 150, 'global', 'epic', 'status_planned', 100, 1, 0, 'achievements.ach.status_planned_100.title', 'achievements.ach.status_planned_100.desc'),
    ('status_planned_300', 'progress', 'game-icons:bookshelf', 300, 'global', 'legendary', 'status_planned', 300, 1, 0, 'achievements.ach.status_planned_300.title', 'achievements.ach.status_planned_300.desc'),
    ('status_on_hold_10', 'progress', 'game-icons:hourglass', 30, 'global', 'common', 'status_on_hold', 10, 1, 0, 'achievements.ach.status_on_hold_10.title', 'achievements.ach.status_on_hold_10.desc'),
    ('status_on_hold_50', 'progress', 'game-icons:pause-button', 80, 'global', 'rare', 'status_on_hold', 50, 1, 0, 'achievements.ach.status_on_hold_50.title', 'achievements.ach.status_on_hold_50.desc'),
    ('status_on_hold_100', 'progress', 'game-icons:sleeping-bag', 150, 'global', 'epic', 'status_on_hold', 100, 1, 0, 'achievements.ach.status_on_hold_100.title', 'achievements.ach.status_on_hold_100.desc'),
    ('status_dropped_10', 'progress', 'game-icons:trash-can', 30, 'global', 'common', 'status_dropped', 10, 1, 0, 'achievements.ach.status_dropped_10.title', 'achievements.ach.status_dropped_10.desc'),
    ('status_dropped_50', 'progress', 'game-icons:broken-bone', 80, 'global', 'rare', 'status_dropped', 50, 1, 0, 'achievements.ach.status_dropped_50.title', 'achievements.ach.status_dropped_50.desc'),
    ('status_dropped_100', 'progress', 'game-icons:grave', 150, 'global', 'epic', 'status_dropped', 100, 1, 0, 'achievements.ach.status_dropped_100.title', 'achievements.ach.status_dropped_100.desc')
ON CONFLICT(code) DO UPDATE SET
    category = excluded.category,
    icon = excluded.icon,
    xp_reward = excluded.xp_reward,
    scope = excluded.scope,
    tier = excluded.tier,
    condition_type = excluded.condition_type,
    condition_value = excluded.condition_value,
    required_count = excluded.required_count,
    is_secret = excluded.is_secret,
    title_key = excluded.title_key,
    description_key = excluded.description_key;