-- 027: Тестовая аниме-ачивка «Пати из дурки» (Коносуба, 3 сезона)

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('anime_konosuba_party', 'anime', 'game-icons:beer-stein', 80, 'global', 'epic', 'anime_franchise_completed', 30831, 1, 0, 'achievements.ach.anime_konosuba_party.title', 'achievements.ach.anime_konosuba_party.desc')
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