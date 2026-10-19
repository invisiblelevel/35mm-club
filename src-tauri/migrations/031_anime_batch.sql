-- 031: Пак аниме-ачивок

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('anime_yani_neko', 'anime', 'game-icons:cat', 40, 'global', 'rare', 'anime_episodes_completed', 59639, 1, 0, 'achievements.ach.anime_yani_neko.title', 'achievements.ach.anime_yani_neko.desc'),
    ('anime_gurren_lagann', 'anime', 'game-icons:drill', 70, 'global', 'epic', 'anime_episodes_completed', 2001, 1, 0, 'achievements.ach.anime_gurren_lagann.title', 'achievements.ach.anime_gurren_lagann.desc'),
    ('anime_toradora', 'anime', 'game-icons:tiger', 60, 'global', 'epic', 'anime_episodes_completed', 4224, 1, 0, 'achievements.ach.anime_toradora.title', 'achievements.ach.anime_toradora.desc'),
    ('anime_chi_sweet_home', 'anime', 'game-icons:cat', 50, 'global', 'rare', 'anime_episodes_completed', 3841, 1, 0, 'achievements.ach.anime_chi_sweet_home.title', 'achievements.ach.anime_chi_sweet_home.desc'),
    ('anime_made_in_abyss', 'anime', 'game-icons:abyss', 80, 'global', 'epic', 'anime_franchise_completed', 97986, 2, 0, 'achievements.ach.anime_made_in_abyss.title', 'achievements.ach.anime_made_in_abyss.desc')
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