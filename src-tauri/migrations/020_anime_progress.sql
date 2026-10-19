-- 020: Прогресс эпизодов для аниме
ALTER TABLE user_titles ADD COLUMN anime_episodes_watched INTEGER NOT NULL DEFAULT 0;