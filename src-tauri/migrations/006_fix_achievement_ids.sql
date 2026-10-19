UPDATE achievements SET condition_value = 9708 WHERE code = 'not_the_bees';
UPDATE achievements SET condition_value = 146402 WHERE code = 'nevsky_absolute';
UPDATE achievements SET condition_value = 526896 WHERE code = 'morbius_time';
UPDATE achievements SET condition_value = 12444 WHERE code = 'room_wiseau';
UPDATE achievements SET condition_value = 145952 WHERE code = 'green_elephant';
UPDATE achievements SET condition_value = 538243 WHERE code = 'cats_horrifying';

-- Для ачивок, зависящих от актёров/режиссёров, тип условия меняем на cast_contains / director_contains
UPDATE achievements SET condition_type = 'cast_contains', condition_value = 146402 WHERE code = 'nevsky_absolute';
UPDATE achievements SET condition_type = 'director_contains', condition_value = 525 WHERE code = 'nolan_time';
UPDATE achievements SET condition_type = 'director_contains', condition_value = 138 WHERE code = 'tarantino_feet';
UPDATE achievements SET condition_type = 'cast_contains', condition_value = 2963 WHERE code = 'cage_fever';

-- Обновляем тип для runtime и year
UPDATE achievements SET condition_type = 'runtime_gt', condition_value = 200 WHERE code = 'marathon_endurance';
UPDATE achievements SET condition_type = 'runtime_lt', condition_value = 70 WHERE code = 'short_and_sweet';
UPDATE achievements SET condition_type = 'year_lt', condition_value = 1950 WHERE code = 'ancient_fossil';