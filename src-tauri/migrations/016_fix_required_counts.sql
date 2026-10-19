-- 016: принудительно выставляем правильные пороги required_count.
-- Применяется один раз через lib.rs (user_version).

UPDATE achievements SET required_count = 10 WHERE code = 'fast_family';
UPDATE achievements SET required_count = 6  WHERE code = 'sharknado_marathon';
UPDATE achievements SET required_count = 30 WHERE code = 'marvel_fatigue';
UPDATE achievements SET required_count = 10 WHERE code = 'nolan_time';
UPDATE achievements SET required_count = 9  WHERE code = 'tarantino_feet';
UPDATE achievements SET required_count = 10 WHERE code = 'cage_fever';
UPDATE achievements SET required_count = 3  WHERE code = 'uwe_boll_survivor';
UPDATE achievements SET required_count = 5  WHERE code = 'snob_critic';
UPDATE achievements SET required_count = 5  WHERE code = 'speedrunner';
UPDATE achievements SET required_count = 1  WHERE code = 'night_owl';
UPDATE achievements SET required_count = 1  WHERE code = 'masochist_trash';