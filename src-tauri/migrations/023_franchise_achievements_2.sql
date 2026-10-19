-- 023: Вторая пачка франшизных ачивок

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('godfather_offer', 'franchise', 'game-icons:spiked-collar', 60, 'global', 'epic', 'collection_id', 230, 3, 0, 'achievements.ach.godfather_offer.title', 'achievements.ach.godfather_offer.desc'),
    ('mission_impossible_stunt', 'franchise', 'game-icons:spy', 60, 'global', 'epic', 'collection_id', 87359, 8, 0, 'achievements.ach.mission_impossible_stunt.title', 'achievements.ach.mission_impossible_stunt.desc'),
    ('hunger_games_mockingjay', 'franchise', 'game-icons:bow-arrow', 50, 'global', 'epic', 'collection_id', 131635, 4, 0, 'achievements.ach.hunger_games_mockingjay.title', 'achievements.ach.hunger_games_mockingjay.desc'),
    ('halloween_myers', 'franchise', 'game-icons:thrown-knife', 60, 'global', 'epic', 'collection_id', 91361, 11, 0, 'achievements.ach.halloween_myers.title', 'achievements.ach.halloween_myers.desc'),
    ('ocean_eleven_heist', 'franchise', 'game-icons:diamonds', 50, 'global', 'epic', 'collection_id', 304, 3, 0, 'achievements.ach.ocean_eleven_heist.title', 'achievements.ach.ocean_eleven_heist.desc'),
    ('madagascar_smile_and_wave', 'franchise', 'game-icons:lion', 40, 'global', 'rare', 'collection_id', 14740, 3, 0, 'achievements.ach.madagascar_smile_and_wave.title', 'achievements.ach.madagascar_smile_and_wave.desc'),
    ('scream_ghostface', 'franchise', 'game-icons:knife-thrust', 60, 'global', 'epic', 'collection_id', 2602, 8, 0, 'achievements.ach.scream_ghostface.title', 'achievements.ach.scream_ghostface.desc')
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