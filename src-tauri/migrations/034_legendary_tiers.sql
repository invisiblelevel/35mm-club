-- 034: Повышаем самые эпичные ачивки до Legendary

UPDATE achievements SET tier = 'legendary' WHERE code IN (
    -- Фильмы
    'fast_family',
    'marvel_fatigue',
    'nolan_time',
    'tarantino_feet',
    'uwe_boll_survivor',
    'sharknado_marathon',

    -- Сериалы
    'tv_walking_dead_coral',
    'tv_supernatural_pie',
    'tv_breaking_bad_cook',
    'tv_got_coffee_cup',
    'tv_friends_pivot',
    'tv_lost_numbers',
    'tv_sopranos_duck',
    'tv_twin_peaks_pie'
);