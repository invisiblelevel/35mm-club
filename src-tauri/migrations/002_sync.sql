-- ============================================================
-- 35mm Club — Миграция 002: Wi-Fi синхронизация
-- Переделывает sync_log, добавляет device_identity, sync_peers,
-- UUID для коллекций.
-- Применяется один раз (user_version 1 → 2).
-- ============================================================

-- ------------------------------------------------------------
-- 1. СТАРЫЙ sync_log НЕИСПОЛЬЗУЕМ — ДРОПАЕМ
-- ------------------------------------------------------------
DROP TABLE IF EXISTS sync_log;

-- ------------------------------------------------------------
-- 2. ИДЕНТИЧНОСТЬ УСТРОЙСТВА
-- Одна строка. Приватный ключ Ed25519 лежит в файле device.key
-- рядом с БД, тут только pubkey и метаданные.
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS device_identity (
    id           INTEGER PRIMARY KEY CHECK (id = 1),
    device_id    TEXT NOT NULL UNIQUE,
    device_name  TEXT NOT NULL,
    public_key   TEXT NOT NULL,
    created_at   TEXT NOT NULL DEFAULT (datetime('now'))
);

-- ------------------------------------------------------------
-- 3. СПАРЕННЫЕ УСТРОЙСТВА
-- ------------------------------------------------------------
CREATE TABLE IF NOT EXISTS sync_peers (
    device_id        TEXT PRIMARY KEY,
    device_name      TEXT NOT NULL,
    public_key       TEXT NOT NULL,
    last_pushed_seq  INTEGER NOT NULL DEFAULT 0,
    last_pulled_seq  INTEGER NOT NULL DEFAULT 0,
    paired_at        TEXT NOT NULL DEFAULT (datetime('now')),
    last_seen_at     TEXT
);

-- ------------------------------------------------------------
-- 4. НОВЫЙ ЛОГ ИЗМЕНЕНИЙ
-- Натуральные ключи (entity_key), LWW по updated_at.
-- device_id — автор изменения, чтобы не отправлять обратно.
-- ------------------------------------------------------------
CREATE TABLE sync_log (
    seq          INTEGER PRIMARY KEY AUTOINCREMENT,
    device_id    TEXT NOT NULL,
    entity_type  TEXT NOT NULL,
    entity_key   TEXT NOT NULL,
    action       TEXT NOT NULL CHECK(action IN ('upsert', 'delete')),
    payload      TEXT,
    updated_at   TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sync_log_seq ON sync_log(seq);
CREATE INDEX IF NOT EXISTS idx_sync_log_entity ON sync_log(entity_type, entity_key);
CREATE INDEX IF NOT EXISTS idx_sync_log_device ON sync_log(device_id);

-- ------------------------------------------------------------
-- 5. UUID ДЛЯ КОЛЛЕКЦИЙ (натуральный ключ синка)
-- ------------------------------------------------------------
ALTER TABLE collections ADD COLUMN uuid TEXT;

UPDATE collections
   SET uuid = lower(hex(randomblob(16)))
 WHERE uuid IS NULL;

CREATE UNIQUE INDEX IF NOT EXISTS idx_collections_uuid
    ON collections(uuid) WHERE uuid IS NOT NULL;

-- ------------------------------------------------------------
-- 6. user_profile.updated_at — уже есть, но подстрахуемся
-- ------------------------------------------------------------
-- (поле уже есть в 001_initial.sql, ничего не делаем)

-- ------------------------------------------------------------
-- 7. МАРКЕР ВЕРСИИ
-- ------------------------------------------------------------
PRAGMA user_version = 2;