-- 024: Третья пачка франшизных ачивок

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('predator_hunter', 'franchise', 'game-icons:hunter-eyes', 60, 'global', 'epic', 'collection_id', 399, 6, 0, 'achievements.ach.predator_hunter.title', 'achievements.ach.predator_hunter.desc'),
    ('evil_dead_groovy', 'franchise', 'game-icons:chainsaw', 50, 'global', 'epic', 'collection_id', 1960, 3, 0, 'achievements.ach.evil_dead_groovy.title', 'achievements.ach.evil_dead_groovy.desc'),
    ('ghostbusters_proton', 'franchise', 'game-icons:ghost', 50, 'global', 'epic', 'collection_id', 2980, 4, 0, 'achievements.ach.ghostbusters_proton.title', 'achievements.ach.ghostbusters_proton.desc'),
    ('rocky_balboa_eye', 'franchise', 'game-icons:boxing-glove', 60, 'global', 'epic', 'collection_id', 1575, 6, 0, 'achievements.ach.rocky_balboa_eye.title', 'achievements.ach.rocky_balboa_eye.desc'),
    ('creed_legacy', 'franchise', 'game-icons:fist', 50, 'global', 'epic', 'collection_id', 553717, 4, 0, 'achievements.ach.creed_legacy.title', 'achievements.ach.creed_legacy.desc'),
    ('conjuring_universe', 'franchise', 'game-icons:gothic-cross', 60, 'global', 'epic', 'collection_id', 313086, 4, 0, 'achievements.ach.conjuring_universe.title', 'achievements.ach.conjuring_universe.desc'),
    ('planet_of_apes', 'franchise', 'game-icons:monkey', 50, 'global', 'epic', 'collection_id', 173710, 4, 0, 'achievements.ach.planet_of_apes.title', 'achievements.ach.planet_of_apes.desc'),
    ('kingsman_manners', 'franchise', 'game-icons:umbrella', 50, 'global', 'epic', 'collection_id', 391860, 3, 0, 'achievements.ach.kingsman_manners.title', 'achievements.ach.kingsman_manners.desc')
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