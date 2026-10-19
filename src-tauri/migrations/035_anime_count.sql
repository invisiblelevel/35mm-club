-- 035: Количественные ачивки для аниме. Удаляем старую anime_10.

-- Сначала чистим связи у пользователей (на случай, если старая ачивка была получена)
DELETE FROM user_achievements WHERE achievement_id IN (SELECT id FROM achievements WHERE code = 'anime_10');
-- Теперь удаляем саму ачивку
DELETE FROM achievements WHERE code = 'anime_10';

-- Вставляем новые количественные ачивки
INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('anime_count_10', 'anime', 'game-icons:tv', 40, 'global', 'common', 'anime_count', 10, 1, 0, 'achievements.ach.anime_count_10.title', 'achievements.ach.anime_count_10.desc'),
    ('anime_count_50', 'anime', 'game-icons:school', 80, 'global', 'rare', 'anime_count', 50, 1, 0, 'achievements.ach.anime_count_50.title', 'achievements.ach.anime_count_50.desc'),
    ('anime_count_150', 'anime', 'game-icons:katana', 150, 'global', 'epic', 'anime_count', 150, 1, 0, 'achievements.ach.anime_count_150.title', 'achievements.ach.anime_count_150.desc'),
    ('anime_count_300', 'anime', 'game-icons:owl', 250, 'global', 'epic', 'anime_count', 300, 1, 0, 'achievements.ach.anime_count_300.title', 'achievements.ach.anime_count_300.desc'),
    ('anime_count_500', 'anime', 'game-icons:samurai-helmet', 400, 'global', 'legendary', 'anime_count', 500, 1, 0, 'achievements.ach.anime_count_500.title', 'achievements.ach.anime_count_500.desc'),
    ('anime_count_750', 'anime', 'game-icons:night-sleep', 600, 'global', 'legendary', 'anime_count', 750, 1, 0, 'achievements.ach.anime_count_750.title', 'achievements.ach.anime_count_750.desc'),
    ('anime_count_1000', 'anime', 'game-icons:abyss', 1000, 'global', 'legendary', 'anime_count', 1000, 1, 0, 'achievements.ach.anime_count_1000.title', 'achievements.ach.anime_count_1000.desc'),
    ('anime_count_1500', 'anime', 'game-icons:god-finger', 2000, 'global', 'legendary', 'anime_count', 1500, 1, 0, 'achievements.ach.anime_count_1500.title', 'achievements.ach.anime_count_1500.desc')
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