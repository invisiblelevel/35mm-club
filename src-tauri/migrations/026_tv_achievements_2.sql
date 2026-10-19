-- 026: Четвёртый пак сериальных ачивок (9 подтверждённых)

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('tv_mandalorian_way', 'franchise', 'game-icons:helmet', 60, 'global', 'epic', 'tv_episodes_completed', 82856, 1, 0, 'achievements.ach.tv_mandalorian_way.title', 'achievements.ach.tv_mandalorian_way.desc'),
    ('tv_better_call_saul_lawyer', 'franchise', 'game-icons:scales', 80, 'global', 'epic', 'tv_episodes_completed', 60059, 1, 0, 'achievements.ach.tv_better_call_saul_lawyer.title', 'achievements.ach.tv_better_call_saul_lawyer.desc'),
    ('tv_dexter_plastic_wrap', 'franchise', 'game-icons:blood', 80, 'global', 'epic', 'tv_episodes_completed', 1405, 1, 0, 'achievements.ach.tv_dexter_plastic_wrap.title', 'achievements.ach.tv_dexter_plastic_wrap.desc'),
    ('tv_peaky_blinders_by_order', 'franchise', 'game-icons:razor', 70, 'global', 'epic', 'tv_episodes_completed', 60574, 1, 0, 'achievements.ach.tv_peaky_blinders_by_order.title', 'achievements.ach.tv_peaky_blinders_by_order.desc'),
    ('tv_chernobyl_not_great', 'franchise', 'game-icons:nuclear', 60, 'global', 'epic', 'tv_episodes_completed', 87108, 1, 0, 'achievements.ach.tv_chernobyl_not_great.title', 'achievements.ach.tv_chernobyl_not_great.desc'),
    ('tv_stranger_things_running_up', 'franchise', 'game-icons:walkie-talkie', 70, 'global', 'epic', 'tv_episodes_completed', 66732, 1, 0, 'achievements.ach.tv_stranger_things_running_up.title', 'achievements.ach.tv_stranger_things_running_up.desc'),
    ('tv_silicon_valley_middle_out', 'franchise', 'game-icons:processor', 60, 'global', 'epic', 'tv_episodes_completed', 60573, 1, 0, 'achievements.ach.tv_silicon_valley_middle_out.title', 'achievements.ach.tv_silicon_valley_middle_out.desc'),
    ('tv_fargo_ok_then', 'franchise', 'game-icons:flake', 70, 'global', 'epic', 'tv_episodes_completed', 60622, 1, 0, 'achievements.ach.tv_fargo_ok_then.title', 'achievements.ach.tv_fargo_ok_then.desc'),
    ('tv_arcane_league', 'franchise', 'game-icons:magic-potion', 60, 'global', 'epic', 'tv_episodes_completed', 94605, 1, 0, 'achievements.ach.tv_arcane_league.title', 'achievements.ach.tv_arcane_league.desc')
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