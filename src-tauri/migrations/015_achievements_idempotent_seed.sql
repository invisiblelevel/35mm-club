-- 015_achievements_idempotent_seed.sql
-- Финальное состояние всех 26 ачивок. Идемпотентно.
-- required_count — порог «сколько нужно» для cast/director/collection/company/keyword/rating.
-- Для счётчиков (movies_count и т.д.) required_count не используется (порог в condition_value).

INSERT INTO achievements
    (code, category, icon, xp_reward, scope, tier,
     condition_type, condition_value, required_count,
     is_secret, title_key, description_key)
VALUES
    -- progress
    ('first_step',         'progress',  'game-icons:footprint',      10, 'global', 'common', 'total_count',               1,      1, 0, 'achievements.ach.first_step.title',         'achievements.ach.first_step.desc'),
    ('movies_10',          'progress',  'game-icons:film-projector', 15, 'movie',  'common', 'movies_count',              10,     1, 0, 'achievements.ach.movies_10.title',          'achievements.ach.movies_10.desc'),
    ('movies_100',         'progress',  'game-icons:clapperboard',   50, 'movie',  'rare',   'movies_count',              100,    1, 0, 'achievements.ach.movies_100.title',         'achievements.ach.movies_100.desc'),
    ('tv_10',              'progress',  'game-icons:tv',             15, 'tv',     'common', 'tv_count',                  10,     1, 0, 'achievements.ach.tv_10.title',              'achievements.ach.tv_10.desc'),
    ('anime_10',           'progress',  'game-icons:abstract-001',   15, 'anime',  'common', 'anime_count',               10,     1, 0, 'achievements.ach.anime_10.title',           'achievements.ach.anime_10.desc'),

    -- humor
    ('not_the_bees',       'humor',     'game-icons:bee',            50, 'global', 'rare',   'movie_id',                  9708,   1, 0, 'achievements.ach.not_the_bees.title',       'achievements.ach.not_the_bees.desc'),
    ('nevsky_absolute',    'humor',     'game-icons:crossed-pistols',30, 'global', 'rare',   'cast_or_producer_contains', 146402, 1, 0, 'achievements.ach.nevsky_absolute.title',    'achievements.ach.nevsky_absolute.desc'),
    ('room_wiseau',        'humor',     'game-icons:spoon',          40, 'global', 'rare',   'movie_id',                  17473,  1, 0, 'achievements.ach.room_wiseau.title',        'achievements.ach.room_wiseau.desc'),
    ('green_elephant',     'humor',     'game-icons:elephant',       40, 'global', 'rare',   'movie_id',                  132030, 1, 0, 'achievements.ach.green_elephant.title',     'achievements.ach.green_elephant.desc'),
    ('morbius_time',       'humor',     'game-icons:bat',            20, 'global', 'common', 'movie_id',                  526896, 1, 0, 'achievements.ach.morbius_time.title',       'achievements.ach.morbius_time.desc'),
    ('cats_horrifying',    'humor',     'game-icons:cat',            20, 'global', 'common', 'movie_id',                  536869, 1, 0, 'achievements.ach.cats_horrifying.title',    'achievements.ach.cats_horrifying.desc'),
    ('masochist_trash',    'humor',     'game-icons:trash-can',      40, 'global', 'rare',   'user_rating_with_low_tmdb', NULL,   1, 0, 'achievements.ach.masochist_trash.title',    'achievements.ach.masochist_trash.desc'),
    ('snob_critic',        'humor',     'game-icons:thumb-down',     30, 'global', 'rare',   'ratings_1',                 NULL,   5, 0, 'achievements.ach.snob_critic.title',        'achievements.ach.snob_critic.desc'),

    -- franchise
    ('fast_family',        'franchise', 'game-icons:race-car',       60, 'global', 'epic',   'collection_id',             9485,   10,0, 'achievements.ach.fast_family.title',        'achievements.ach.fast_family.desc'),
    ('sharknado_marathon', 'franchise', 'game-icons:shark-fin',      60, 'global', 'epic',   'collection_id',             286023, 6, 0, 'achievements.ach.sharknado_marathon.title', 'achievements.ach.sharknado_marathon.desc'),
    ('marvel_fatigue',     'franchise', 'game-icons:shield',         70, 'global', 'epic',   'keyword_or_company',        180547, 30,0, 'achievements.ach.marvel_fatigue.title',     'achievements.ach.marvel_fatigue.desc'),

    -- director / actor
    ('nolan_time',         'director',  'game-icons:clockwork',      80, 'global', 'epic',   'director_contains',         525,    10,0, 'achievements.ach.nolan_time.title',         'achievements.ach.nolan_time.desc'),
    ('tarantino_feet',     'director',  'game-icons:high-heel',      80, 'global', 'epic',   'director_contains',         138,    9, 0, 'achievements.ach.tarantino_feet.title',     'achievements.ach.tarantino_feet.desc'),
    ('cage_fever',         'actor',     'game-icons:cage',           50, 'global', 'rare',   'cast_contains',             2963,   10,0, 'achievements.ach.cage_fever.title',         'achievements.ach.cage_fever.desc'),
    ('uwe_boll_survivor',  'director',  'game-icons:boxing-glove',   50, 'global', 'rare',   'director_contains',         23799,  3, 0, 'achievements.ach.uwe_boll_survivor.title',  'achievements.ach.uwe_boll_survivor.desc'),

    -- time
    ('speedrunner',        'time',      'game-icons:run',            40, 'global', 'rare',   'daily_completed',           5,      1, 0, 'achievements.ach.speedrunner.title',        'achievements.ach.speedrunner.desc'),
    ('night_owl',          'time',      'game-icons:owl',            30, 'global', 'rare',   'night_watch',               NULL,   1, 0, 'achievements.ach.night_owl.title',          'achievements.ach.night_owl.desc'),
    ('marathon_endurance', 'time',      'game-icons:armchair',       30, 'global', 'rare',   'runtime_gt',                200,    1, 0, 'achievements.ach.marathon_endurance.title', 'achievements.ach.marathon_endurance.desc'),
    ('ancient_fossil',     'time',      'game-icons:fossil',         30, 'global', 'rare',   'year_lt',                   1950,   1, 0, 'achievements.ach.ancient_fossil.title',     'achievements.ach.ancient_fossil.desc'),
    ('short_and_sweet',    'time',      'game-icons:hourglass',      20, 'global', 'common', 'runtime_lt',                70,     1, 0, 'achievements.ach.short_and_sweet.title',    'achievements.ach.short_and_sweet.desc')
ON CONFLICT(code) DO UPDATE SET
    category        = excluded.category,
    icon            = excluded.icon,
    xp_reward       = excluded.xp_reward,
    scope           = excluded.scope,
    tier            = excluded.tier,
    condition_type  = excluded.condition_type,
    condition_value = excluded.condition_value,
    required_count  = excluded.required_count,
    is_secret       = excluded.is_secret,
    title_key       = excluded.title_key,
    description_key = excluded.description_key;