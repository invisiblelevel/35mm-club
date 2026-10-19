-- 036: Количественные ачивки для фильмов. Удаляем старые movies_10 и movies_100.

-- Чистим связи у пользователей
DELETE FROM user_achievements WHERE achievement_id IN (SELECT id FROM achievements WHERE code IN ('movies_10', 'movies_100'));
-- Удаляем сами ачивки
DELETE FROM achievements WHERE code IN ('movies_10', 'movies_100');

-- Вставляем новые количественные ачивки для фильмов
INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('movie_count_10', 'movie', 'game-icons:popcorn', 40, 'global', 'common', 'movies_count', 10, 1, 0, 'achievements.ach.movie_count_10.title', 'achievements.ach.movie_count_10.desc'),
    ('movie_count_50', 'movie', 'game-icons:film-strip', 80, 'global', 'rare', 'movies_count', 50, 1, 0, 'achievements.ach.movie_count_50.title', 'achievements.ach.movie_count_50.desc'),
    ('movie_count_150', 'movie', 'game-icons:sofa', 150, 'global', 'epic', 'movies_count', 150, 1, 0, 'achievements.ach.movie_count_150.title', 'achievements.ach.movie_count_150.desc'),
    ('movie_count_300', 'movie', 'game-icons:crown', 250, 'global', 'epic', 'movies_count', 300, 1, 0, 'achievements.ach.movie_count_300.title', 'achievements.ach.movie_count_300.desc'),
    ('movie_count_500', 'movie', 'game-icons:film-projector', 400, 'global', 'legendary', 'movies_count', 500, 1, 0, 'achievements.ach.movie_count_500.title', 'achievements.ach.movie_count_500.desc'),
    ('movie_count_750', 'movie', 'game-icons:skull-crossed-bones', 600, 'global', 'legendary', 'movies_count', 750, 1, 0, 'achievements.ach.movie_count_750.title', 'achievements.ach.movie_count_750.desc'),
    ('movie_count_1000', 'movie', 'game-icons:matrix', 1000, 'global', 'legendary', 'movies_count', 1000, 1, 0, 'achievements.ach.movie_count_1000.title', 'achievements.ach.movie_count_1000.desc'),
    ('movie_count_1500', 'movie', 'game-icons:film', 2000, 'global', 'legendary', 'movies_count', 1500, 1, 0, 'achievements.ach.movie_count_1500.title', 'achievements.ach.movie_count_1500.desc')
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