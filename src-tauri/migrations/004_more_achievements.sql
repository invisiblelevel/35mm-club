INSERT OR IGNORE INTO achievements (
  code, category, icon, xp_reward, scope, tier, condition_type, condition_value, title_key, description_key
) VALUES
  ('not_the_bees', 'humor', null, 50, 'global', 'rare', 'movie_id', 9306, 'achievements.ach.not_the_bees.title', 'achievements.ach.not_the_bees.desc'),
  ('nevsky_absolute', 'humor', null, 30, 'global', 'rare', 'cast_contains', null, 'achievements.ach.nevsky_absolute.title', 'achievements.ach.nevsky_absolute.desc'),
  ('room_wiseau', 'humor', null, 40, 'global', 'rare', 'movie_id', 12444, 'achievements.ach.room_wiseau.title', 'achievements.ach.room_wiseau.desc'),
  ('green_elephant', 'humor', null, 40, 'global', 'rare', 'movie_id', 145952, 'achievements.ach.green_elephant.title', 'achievements.ach.green_elephant.desc'),
  ('morbius_time', 'humor', null, 20, 'global', 'common', 'movie_id', 526896, 'achievements.ach.morbius_time.title', 'achievements.ach.morbius_time.desc'),
  ('cats_horrifying', 'humor', null, 20, 'global', 'common', 'movie_id', 538243, 'achievements.ach.cats_horrifying.title', 'achievements.ach.cats_horrifying.desc'),
  ('fast_family', 'franchise', null, 60, 'global', 'epic', 'collection_id', 9485, 'achievements.ach.fast_family.title', 'achievements.ach.fast_family.desc'),
  ('nolan_time', 'director', null, 80, 'global', 'epic', 'director_id', 525, 'achievements.ach.nolan_time.title', 'achievements.ach.nolan_time.desc'),
  ('sharknado_marathon', 'franchise', null, 60, 'global', 'epic', 'collection_id', null, 'achievements.ach.sharknado_marathon.title', 'achievements.ach.sharknado_marathon.desc'),
  ('marvel_fatigue', 'franchise', null, 70, 'global', 'epic', 'collection_id', 529892, 'achievements.ach.marvel_fatigue.title', 'achievements.ach.marvel_fatigue.desc'),
  ('tarantino_feet', 'director', null, 80, 'global', 'epic', 'director_id', 138, 'achievements.ach.tarantino_feet.title', 'achievements.ach.tarantino_feet.desc'),
  ('cage_fever', 'actor', null, 50, 'global', 'rare', 'actor_id', 2963, 'achievements.ach.cage_fever.title', 'achievements.ach.cage_fever.desc'),
  ('masochist_trash', 'humor', null, 40, 'global', 'rare', 'user_rating', null, 'achievements.ach.masochist_trash.title', 'achievements.ach.masochist_trash.desc'),
  ('speedrunner', 'time', null, 40, 'global', 'rare', 'daily_completed', 5, 'achievements.ach.speedrunner.title', 'achievements.ach.speedrunner.desc'),
  ('night_owl', 'time', null, 30, 'global', 'rare', 'night_watch', null, 'achievements.ach.night_owl.title', 'achievements.ach.night_owl.desc'),
  ('marathon_endurance', 'time', null, 30, 'global', 'rare', 'runtime_min', 200, 'achievements.ach.marathon_endurance.title', 'achievements.ach.marathon_endurance.desc'),
  ('ancient_fossil', 'time', null, 30, 'global', 'rare', 'release_before', 1950, 'achievements.ach.ancient_fossil.title', 'achievements.ach.ancient_fossil.desc'),
  ('short_and_sweet', 'time', null, 20, 'global', 'common', 'runtime_max', 70, 'achievements.ach.short_and_sweet.title', 'achievements.ach.short_and_sweet.desc'),
  ('uwe_boll_survivor', 'director', null, 50, 'global', 'rare', 'director_id_count', 10950, 'achievements.ach.uwe_boll_survivor.title', 'achievements.ach.uwe_boll_survivor.desc'),
  ('snob_critic', 'humor', null, 30, 'global', 'rare', 'ratings_1', 5, 'achievements.ach.snob_critic.title', 'achievements.ach.snob_critic.desc');