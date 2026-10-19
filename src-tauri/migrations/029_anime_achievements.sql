-- 029: Пак аниме-ачивок

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    ('anime_prison_school', 'anime', 'game-icons:prison', 60, 'global', 'epic', 'anime_franchise_completed', 30240, 2, 0, 'achievements.ach.anime_prison_school.title', 'achievements.ach.anime_prison_school.desc'),
    ('anime_redo_healer', 'anime', 'game-icons:healing', 60, 'global', 'epic', 'anime_episodes_completed', 37992, 1, 0, 'achievements.ach.anime_redo_healer.title', 'achievements.ach.anime_redo_healer.desc'),
    ('anime_death_note', 'anime', 'game-icons:notebook', 70, 'global', 'epic', 'anime_episodes_completed', 1535, 1, 0, 'achievements.ach.anime_death_note.title', 'achievements.ach.anime_death_note.desc'),
    ('anime_nagatoro', 'anime', 'game-icons:heart-plus', 50, 'global', 'rare', 'anime_episodes_completed', 42361, 1, 0, 'achievements.ach.anime_nagatoro.title', 'achievements.ach.anime_nagatoro.desc'),
    ('anime_elfen_lied', 'anime', 'game-icons:blood', 60, 'global', 'epic', 'anime_episodes_completed', 1836, 1, 0, 'achievements.ach.anime_elfen_lied.title', 'achievements.ach.anime_elfen_lied.desc'),
    ('anime_higurashi', 'anime', 'game-icons:cricket', 60, 'global', 'epic', 'anime_episodes_completed', 934, 1, 0, 'achievements.ach.anime_higurashi.title', 'achievements.ach.anime_higurashi.desc'),
    ('anime_chainsaw_man', 'anime', 'game-icons:chainsaw', 60, 'global', 'epic', 'anime_episodes_completed', 44511, 1, 0, 'achievements.ach.anime_chainsaw_man.title', 'achievements.ach.anime_chainsaw_man.desc'),
    ('anime_black_lagoon', 'anime', 'game-icons:revolver', 70, 'global', 'epic', 'anime_franchise_completed', 889, 2, 0, 'achievements.ach.anime_black_lagoon.title', 'achievements.ach.anime_black_lagoon.desc')
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