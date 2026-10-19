-- Исправляем ID Плетёного человека (2006) на правильный (9708)
UPDATE achievements SET condition_value = 9708 WHERE code = 'not_the_bees';

-- Убираем ачивки, у которых нет числового условия (они пока не поддерживаются)
-- или ставим им 0, чтобы код их пропускал
UPDATE achievements SET condition_value = 0 
WHERE condition_value IS NULL AND condition_type NOT IN ('total_count', 'movies_count', 'tv_count', 'anime_count');