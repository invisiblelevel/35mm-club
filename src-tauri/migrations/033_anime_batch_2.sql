-- 033: Пак аниме-ачивок (часть 2)

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('anime_mushoku_tensei', 'anime', 'game-icons:baby-face', 90, 'global', 'epic', 'anime_franchise_completed', 39535, 2, 0, 'achievements.ach.anime_mushoku_tensei.title', 'achievements.ach.anime_mushoku_tensei.desc'),
    ('anime_fullmetal_alchemist', 'anime', 'game-icons:transfuse', 150, 'global', 'legendary', 'anime_episodes_completed', 5114, 1, 0, 'achievements.ach.anime_fullmetal_alchemist.title', 'achievements.ach.anime_fullmetal_alchemist.desc'),
    ('anime_bleach', 'anime', 'game-icons:katana', 200, 'global', 'legendary', 'anime_episodes_completed', 269, 1, 0, 'achievements.ach.anime_bleach.title', 'achievements.ach.anime_bleach.desc'),
    ('anime_sword_art_online', 'anime', 'game-icons:crossed-swords', 80, 'global', 'epic', 'anime_episodes_completed', 11757, 1, 0, 'achievements.ach.anime_sword_art_online.title', 'achievements.ach.anime_sword_art_online.desc'),
    ('anime_berserk', 'anime', 'game-icons:sword-wound', 120, 'global', 'epic', 'anime_episodes_completed', 33, 1, 0, 'achievements.ach.anime_berserk.title', 'achievements.ach.anime_berserk.desc')
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