CREATE TABLE IF NOT EXISTS title_cast (
    title_id INTEGER NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    person_id INTEGER NOT NULL,
    PRIMARY KEY (title_id, person_id)
);

CREATE TABLE IF NOT EXISTS title_directors (
    title_id INTEGER NOT NULL REFERENCES titles(id) ON DELETE CASCADE,
    person_id INTEGER NOT NULL,
    PRIMARY KEY (title_id, person_id)
);

-- Пчёлы (правильный ID)
UPDATE achievements SET condition_value = 9708 WHERE code = 'not_the_bees';

-- Absolute Cinema (Александр Невский, правильный ID)
UPDATE achievements SET condition_value = 146402, condition_type = 'cast_contains' WHERE code = 'nevsky_absolute';

-- Nolan (director_id 525)
UPDATE achievements SET condition_value = 525, condition_type = 'director_contains' WHERE code = 'nolan_time';

-- Tarantino (director_id 138)
UPDATE achievements SET condition_value = 138, condition_type = 'director_contains' WHERE code = 'tarantino_feet';

-- Cage (actor_id 2963)
UPDATE achievements SET condition_value = 2963, condition_type = 'cast_contains' WHERE code = 'cage_fever';

-- Runtime > 200
UPDATE achievements SET condition_value = 200, condition_type = 'runtime_gt' WHERE code = 'marathon_endurance';

-- Runtime < 70
UPDATE achievements SET condition_value = 70, condition_type = 'runtime_lt' WHERE code = 'short_and_sweet';

-- Year < 1950
UPDATE achievements SET condition_value = 1950, condition_type = 'year_lt' WHERE code = 'ancient_fossil';

-- Morbius (ID 526896)
UPDATE achievements SET condition_value = 526896, condition_type = 'movie_id' WHERE code = 'morbius_time';

-- Room (ID 12444)
UPDATE achievements SET condition_value = 12444, condition_type = 'movie_id' WHERE code = 'room_wiseau';

-- Green Elephant (ID 145952)
UPDATE achievements SET condition_value = 145952, condition_type = 'movie_id' WHERE code = 'green_elephant';

-- Cats (ID 538243)
UPDATE achievements SET condition_value = 538243, condition_type = 'movie_id' WHERE code = 'cats_horrifying';