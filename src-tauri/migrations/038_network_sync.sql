-- 038: Сетевая синхронизация (базовая схема)

-- 1. Сессия авторизации (Supabase Auth)
CREATE TABLE IF NOT EXISTS auth_session (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    remote_id TEXT,
    email TEXT,
    access_token TEXT,
    refresh_token TEXT,
    expires_at INTEGER,
    last_login_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 2. Поля для отслеживания локальных изменений
-- (SQLite не поддерживает ADD COLUMN IF NOT EXISTS, поэтому делаем через проверку в Rust)
-- Эти ALTER сделаны в lib.rs через column_exists, см. ниже.

-- 3. Журнал операций для синхронизации
CREATE TABLE IF NOT EXISTS sync_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_type TEXT NOT NULL CHECK(entity_type IN ('profile', 'title', 'achievement', 'friend')),
    entity_id INTEGER,
    action TEXT NOT NULL CHECK(action IN ('insert', 'update', 'delete')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    synced_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_sync_log_pending ON sync_log(synced_at) WHERE synced_at IS NULL;

-- 4. Заявки в друзья (отдельно от списка друзей)
CREATE TABLE IF NOT EXISTS friend_requests (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    from_remote_id TEXT NOT NULL,
    to_remote_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'accepted', 'declined', 'cancelled')),
    message TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    responded_at TEXT,
    UNIQUE(from_remote_id, to_remote_id)
);

CREATE INDEX IF NOT EXISTS idx_friend_requests_to ON friend_requests(to_remote_id, status);
CREATE INDEX IF NOT EXISTS idx_friend_requests_from ON friend_requests(from_remote_id, status);

-- 5. Уведомления
CREATE TABLE IF NOT EXISTS notifications (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    remote_id TEXT NOT NULL,
    type TEXT NOT NULL CHECK(type IN ('friend_request', 'friend_accepted', 'like', 'comment')),
    payload TEXT,
    is_read INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    synced_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_notifications_remote ON notifications(remote_id, is_read);

-- 6. Таблица друзей (если её ещё нет — из твоего списка)
CREATE TABLE IF NOT EXISTS friends (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_remote_id TEXT NOT NULL,
    friend_remote_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'pending' CHECK(status IN ('pending', 'accepted', 'blocked', 'declined')),
    created_at TEXT NOT NULL DEFAULT (datetime('now')),
    updated_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(user_remote_id, friend_remote_id)
);

CREATE INDEX IF NOT EXISTS idx_friends_user ON friends(user_remote_id, status);
CREATE INDEX IF NOT EXISTS idx_friends_friend ON friends(friend_remote_id, status);

-- 7. Кэш публичных профилей
CREATE TABLE IF NOT EXISTS remote_profiles (
    remote_id TEXT PRIMARY KEY,
    username TEXT NOT NULL,
    avatar_url TEXT,
    level INTEGER DEFAULT 1,
    xp INTEGER DEFAULT 0,
    total_watch_minutes INTEGER DEFAULT 0,
    rank_name TEXT,
    last_seen_at TEXT,
    last_synced_at TEXT NOT NULL DEFAULT (datetime('now'))
);

-- 8. Кэш фильмов друзей
CREATE TABLE IF NOT EXISTS remote_user_titles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    remote_user_id TEXT NOT NULL,
    tmdb_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    year INTEGER,
    poster_path TEXT,
    status TEXT NOT NULL,
    user_rating REAL,
    is_dirty INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(remote_user_id, tmdb_id)
);

CREATE INDEX IF NOT EXISTS idx_remote_titles_user ON remote_user_titles(remote_user_id);

-- 9. Кэш ачивок друзей
CREATE TABLE IF NOT EXISTS remote_user_achievements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    remote_user_id TEXT NOT NULL,
    achievement_code TEXT NOT NULL,
    unlocked_at TEXT NOT NULL,
    is_dirty INTEGER NOT NULL DEFAULT 0,
    last_synced_at TEXT NOT NULL DEFAULT (datetime('now')),
    UNIQUE(remote_user_id, achievement_code)
);

CREATE INDEX IF NOT EXISTS idx_remote_ach_user ON remote_user_achievements(remote_user_id);

-- 10. Состояние синхронизации
CREATE TABLE IF NOT EXISTS sync_state (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    last_profiles_sync TEXT,
    last_friends_sync TEXT,
    last_titles_sync TEXT,
    last_achievements_sync TEXT
);

INSERT OR IGNORE INTO sync_state (id) VALUES (1);