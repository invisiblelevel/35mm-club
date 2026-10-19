-- Удаляем дубль first_movie
DELETE FROM user_achievements WHERE achievement_id IN (SELECT id FROM achievements WHERE code = 'first_movie');
DELETE FROM achievements WHERE code = 'first_movie';

-- Исправляем ключи локализации (убираем 'ach.' в начале)
UPDATE achievements SET title_key = 'achievements.' || title_key WHERE title_key LIKE 'ach.%';
UPDATE achievements SET description_key = 'achievements.' || description_key WHERE description_key LIKE 'ach.%';