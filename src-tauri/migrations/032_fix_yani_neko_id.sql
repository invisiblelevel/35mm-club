-- 032: Фикс MAL ID для ачивки anime_yani_neko (Табакошка)
-- Старый ID был 59639 (неверный), правильный 63403.

UPDATE achievements 
SET condition_value = 63403 
WHERE code = 'anime_yani_neko' AND condition_value = 59639;