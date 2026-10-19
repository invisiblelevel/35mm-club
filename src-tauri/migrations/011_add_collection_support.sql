ALTER TABLE title_metadata ADD COLUMN collection_id INTEGER;

-- Форсаж (коллекция 9485)
UPDATE achievements SET condition_type = 'collection_id', condition_value = 9485 WHERE code = 'fast_family';
-- Мстители (MCU, коллекция 529892)
UPDATE achievements SET condition_type = 'collection_id', condition_value = 529892 WHERE code = 'marvel_fatigue';
-- Звёздные войны (коллекция 10 — если захочешь добавить)
-- UPDATE achievements SET condition_type = 'collection_id', condition_value = 10 WHERE code = 'star_wars';

-- Для остальных ачивок оставляем как есть