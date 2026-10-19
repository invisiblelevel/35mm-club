-- 025: Сериальные ачивки (через tv_episodes_completed)

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('tv_firefly_browncoat', 'franchise', 'game-icons:spaceship', 60, 'global', 'epic', 'tv_episodes_completed', 1433, 1, 0, 'achievements.ach.tv_firefly_browncoat.title', 'achievements.ach.tv_firefly_browncoat.desc'),
    ('tv_walking_dead_coral', 'franchise', 'game-icons:shambling-zombie', 80, 'global', 'epic', 'tv_episodes_completed', 1402, 1, 0, 'achievements.ach.tv_walking_dead_coral.title', 'achievements.ach.tv_walking_dead_coral.desc'),
    ('tv_lost_numbers', 'franchise', 'game-icons:secret-book', 70, 'global', 'epic', 'tv_episodes_completed', 4607, 1, 0, 'achievements.ach.tv_lost_numbers.title', 'achievements.ach.tv_lost_numbers.desc'),
    ('tv_prison_break_tattoo', 'franchise', 'game-icons:prisoner', 60, 'global', 'epic', 'tv_episodes_completed', 2288, 1, 0, 'achievements.ach.tv_prison_break_tattoo.title', 'achievements.ach.tv_prison_break_tattoo.desc'),
    ('tv_breaking_bad_cook', 'franchise', 'game-icons:test-tubes', 80, 'global', 'epic', 'tv_episodes_completed', 1396, 1, 0, 'achievements.ach.tv_breaking_bad_cook.title', 'achievements.ach.tv_breaking_bad_cook.desc'),
    ('tv_got_coffee_cup', 'franchise', 'game-icons:coffee-cup', 80, 'global', 'epic', 'tv_episodes_completed', 1399, 1, 0, 'achievements.ach.tv_got_coffee_cup.title', 'achievements.ach.tv_got_coffee_cup.desc'),
    ('tv_supernatural_pie', 'franchise', 'game-icons:race-car', 80, 'global', 'epic', 'tv_episodes_completed', 1622, 1, 0, 'achievements.ach.tv_supernatural_pie.title', 'achievements.ach.tv_supernatural_pie.desc'),
    ('tv_office_that_what_she_said', 'franchise', 'game-icons:office-chair', 60, 'global', 'epic', 'tv_episodes_completed', 2316, 1, 0, 'achievements.ach.tv_office_that_what_she_said.title', 'achievements.ach.tv_office_that_what_she_said.desc'),
    ('tv_x_files_truth', 'franchise', 'game-icons:ufo', 70, 'global', 'epic', 'tv_episodes_completed', 4087, 1, 0, 'achievements.ach.tv_x_files_truth.title', 'achievements.ach.tv_x_files_truth.desc'),
    ('tv_sherlock_sociopath', 'franchise', 'game-icons:magnifying-glass', 60, 'global', 'epic', 'tv_episodes_completed', 19885, 1, 0, 'achievements.ach.tv_sherlock_sociopath.title', 'achievements.ach.tv_sherlock_sociopath.desc'),
    ('tv_sopranos_duck', 'franchise', 'game-icons:duck', 70, 'global', 'epic', 'tv_episodes_completed', 1398, 1, 0, 'achievements.ach.tv_sopranos_duck.title', 'achievements.ach.tv_sopranos_duck.desc'),
    ('tv_twin_peaks_pie', 'franchise', 'game-icons:cherry', 60, 'global', 'epic', 'tv_episodes_completed', 1920, 1, 0, 'achievements.ach.tv_twin_peaks_pie.title', 'achievements.ach.tv_twin_peaks_pie.desc'),
    ('tv_friends_pivot', 'franchise', 'game-icons:sofa', 60, 'global', 'epic', 'tv_episodes_completed', 1668, 1, 0, 'achievements.ach.tv_friends_pivot.title', 'achievements.ach.tv_friends_pivot.desc'),
    ('tv_boys_compound_v', 'franchise', 'game-icons:syringe', 60, 'global', 'epic', 'tv_episodes_completed', 76479, 1, 0, 'achievements.ach.tv_boys_compound_v.title', 'achievements.ach.tv_boys_compound_v.desc'),
    ('tv_black_mirror_screen', 'franchise', 'game-icons:smartphone', 60, 'global', 'epic', 'tv_episodes_completed', 42009, 1, 0, 'achievements.ach.tv_black_mirror_screen.title', 'achievements.ach.tv_black_mirror_screen.desc')
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