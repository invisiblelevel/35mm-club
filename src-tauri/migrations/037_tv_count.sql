-- 037: Количественные ачивки для сериалов. Удаляем старую tv_10.

-- Чистим связи у пользователей
DELETE FROM user_achievements WHERE achievement_id IN (SELECT id FROM achievements WHERE code = 'tv_10');
-- Удаляем саму ачивку
DELETE FROM achievements WHERE code = 'tv_10';

-- Вставляем новые количественные ачивки для сериалов
INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('tv_count_10', 'tv', 'game-icons:tv', 40, 'global', 'common', 'tv_count', 10, 1, 0, 'achievements.ach.tv_count_10.title', 'achievements.ach.tv_count_10.desc'),
    ('tv_count_50', 'tv', 'game-icons:popcorn', 80, 'global', 'rare', 'tv_count', 50, 1, 0, 'achievements.ach.tv_count_50.title', 'achievements.ach.tv_count_50.desc'),
    ('tv_count_150', 'tv', 'game-icons:knife-thrust', 150, 'global', 'epic', 'tv_count', 150, 1, 0, 'achievements.ach.tv_count_150.title', 'achievements.ach.tv_count_150.desc'),
    ('tv_count_300', 'tv', 'game-icons:sofa', 250, 'global', 'epic', 'tv_count', 300, 1, 0, 'achievements.ach.tv_count_300.title', 'achievements.ach.tv_count_300.desc'),
    ('tv_count_500', 'tv', 'game-icons:spy', 400, 'global', 'legendary', 'tv_count', 500, 1, 0, 'achievements.ach.tv_count_500.title', 'achievements.ach.tv_count_500.desc'),
    ('tv_count_750', 'tv', 'game-icons:beer-stein', 600, 'global', 'legendary', 'tv_count', 750, 1, 0, 'achievements.ach.tv_count_750.title', 'achievements.ach.tv_count_750.desc'),
    ('tv_count_1000', 'tv', 'game-icons:fast-forward-button', 1000, 'global', 'legendary', 'tv_count', 1000, 1, 0, 'achievements.ach.tv_count_1000.title', 'achievements.ach.tv_count_1000.desc'),
    ('tv_count_1500', 'tv', 'game-icons:netflix', 2000, 'global', 'legendary', 'tv_count', 1500, 1, 0, 'achievements.ach.tv_count_1500.title', 'achievements.ach.tv_count_1500.desc')
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