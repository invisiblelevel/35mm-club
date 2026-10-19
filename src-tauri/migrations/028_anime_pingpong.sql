-- 028: Ачивка за аниме «Вперёд! Школьная секция пинг-понга»

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('anime_pingpong_club', 'anime', 'game-icons:ping-pong-bat', 50, 'global', 'rare', 'anime_episodes_completed', 914, 1, 0, 'achievements.ach.anime_pingpong_club.title', 'achievements.ach.anime_pingpong_club.desc')
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