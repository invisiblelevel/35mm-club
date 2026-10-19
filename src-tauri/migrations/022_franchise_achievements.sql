-- 022: Франшизные ачивки

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('potterhead', 'franchise', 'game-icons:magic-swirl', 60, 'global', 'epic', 'collection_id', 1241, 8, 0, 'achievements.ach.potterhead.title', 'achievements.ach.potterhead.desc'),
    ('lotr_ringbearer', 'franchise', 'game-icons:ring', 50, 'global', 'epic', 'collection_id', 119, 3, 0, 'achievements.ach.lotr_ringbearer.title', 'achievements.ach.lotr_ringbearer.desc'),
    ('matrix_redpill', 'franchise', 'game-icons:pill', 50, 'global', 'epic', 'collection_id', 2344, 4, 0, 'achievements.ach.matrix_redpill.title', 'achievements.ach.matrix_redpill.desc'),
    ('saw_game', 'franchise', 'game-icons:puzzle', 60, 'global', 'epic', 'collection_id', 656, 10, 0, 'achievements.ach.saw_game.title', 'achievements.ach.saw_game.desc'),
    ('starwars_boomer', 'franchise', 'game-icons:light-sabers', 60, 'global', 'epic', 'collection_id', 10, 9, 0, 'achievements.ach.starwars_boomer.title', 'achievements.ach.starwars_boomer.desc'),
    ('shrek_swamp', 'franchise', 'game-icons:swamp', 40, 'global', 'rare', 'collection_id', 2150, 5, 0, 'achievements.ach.shrek_swamp.title', 'achievements.ach.shrek_swamp.desc'),
    ('alien_chestburster', 'franchise', 'game-icons:alien-bug', 50, 'global', 'epic', 'collection_id', 8091, 4, 0, 'achievements.ach.alien_chestburster.title', 'achievements.ach.alien_chestburster.desc'),
    ('terminator_fates', 'franchise', 'game-icons:robot-helmet', 50, 'global', 'epic', 'collection_id', 528, 6, 0, 'achievements.ach.terminator_fates.title', 'achievements.ach.terminator_fates.desc'),
    ('pirates_rum', 'franchise', 'game-icons:pirate-hat', 50, 'global', 'epic', 'collection_id', 295, 5, 0, 'achievements.ach.pirates_rum.title', 'achievements.ach.pirates_rum.desc'),
    ('transformers_bayhem', 'franchise', 'game-icons:robot-golem', 50, 'global', 'epic', 'collection_id', 8650, 5, 0, 'achievements.ach.transformers_bayhem.title', 'achievements.ach.transformers_bayhem.desc'),
    ('john_wick_pencil', 'franchise', 'game-icons:pencil', 50, 'global', 'epic', 'collection_id', 404609, 4, 0, 'achievements.ach.john_wick_pencil.title', 'achievements.ach.john_wick_pencil.desc'),
    ('resident_evil_milla', 'franchise', 'game-icons:biohazard', 50, 'global', 'epic', 'collection_id', 17255, 6, 0, 'achievements.ach.resident_evil_milla.title', 'achievements.ach.resident_evil_milla.desc'),
    ('twilight_vampire', 'franchise', 'game-icons:vampire-dracula', 40, 'global', 'rare', 'collection_id', 33514, 5, 0, 'achievements.ach.twilight_vampire.title', 'achievements.ach.twilight_vampire.desc'),
    ('final_destination', 'franchise', 'game-icons:truck', 50, 'global', 'epic', 'collection_id', 8864, 7, 0, 'achievements.ach.final_destination.title', 'achievements.ach.final_destination.desc')
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