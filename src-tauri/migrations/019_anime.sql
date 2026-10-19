-- 019: AniList для аниме
-- Добавляем поля в titles для данных из AniList.

ALTER TABLE titles ADD COLUMN anilist_id INTEGER;
ALTER TABLE titles ADD COLUMN episodes_total INTEGER;
ALTER TABLE titles ADD COLUMN anime_format TEXT;
ALTER TABLE titles ADD COLUMN anime_status TEXT;
ALTER TABLE titles ADD COLUMN studio TEXT;