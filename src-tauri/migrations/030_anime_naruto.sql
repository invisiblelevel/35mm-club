-- 030: Мега-эпическая ачивка за всю франшизу Наруто

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('anime_naruto_legend', 'anime', 'game-icons:ninja-mask', 200, 'global', 'legendary', 'anime_franchise_completed', 20, 2, 0, 'achievements.ach.anime_naruto_legend.title', 'achievements.ach.anime_naruto_legend.desc')
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