<script lang="ts">
  import Icon from '@iconify/svelte';

  interface Props {
    code: string;
    size?: number | string;
  }

  let { code, size }: Props = $props();

  const config: Record<string, { icon: string; color: string; grad: [string, string] }> = {
    // ==========================================
    // БАЗОВЫЕ И РАННИЕ
    // ==========================================
    first_step:         { icon: 'game-icons:footprint',       color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    movies_10:          { icon: 'game-icons:film-projector',  color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    movies_100:         { icon: 'game-icons:clapperboard',    color: '#f97316', grad: ['#431407', '#1c1917'] },
    tv_10:              { icon: 'game-icons:tv',              color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_10:           { icon: 'game-icons:abstract-001',    color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    not_the_bees:       { icon: 'game-icons:bee',             color: '#facc15', grad: ['#422006', '#1c1917'] },
    nevsky_absolute:    { icon: 'game-icons:crossed-pistols', color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    room_wiseau:        { icon: 'game-icons:spoon',           color: '#d6d3d1', grad: ['#292524', '#1c1917'] },
    green_elephant:     { icon: 'game-icons:elephant',        color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    morbius_time:       { icon: 'game-icons:bat',             color: '#a855f7', grad: ['#2e1065', '#1c1917'] },
    cats_horrifying:    { icon: 'game-icons:cat',             color: '#fb923c', grad: ['#431407', '#1c1917'] },
    fast_family:        { icon: 'game-icons:race-car',        color: '#f87171', grad: ['#450a0a', '#1c1917'] },
    nolan_time:         { icon: 'game-icons:clockwork',       color: '#60a5fa', grad: ['#082f49', '#1c1917'] },
    sharknado_marathon: { icon: 'game-icons:shark-fin',       color: '#22d3ee', grad: ['#083344', '#1c1917'] },
    marvel_fatigue:     { icon: 'game-icons:shield',          color: '#f43f5e', grad: ['#4c0519', '#1c1917'] },
    tarantino_feet:     { icon: 'game-icons:high-heel',       color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    cage_fever:         { icon: 'game-icons:cage',            color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    masochist_trash:    { icon: 'game-icons:trash-can',       color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    speedrunner:        { icon: 'game-icons:run',             color: '#fb7185', grad: ['#4c0519', '#1c1917'] },
    night_owl:          { icon: 'game-icons:owl',             color: '#818cf8', grad: ['#1e1b4b', '#1c1917'] },
    marathon_endurance: { icon: 'game-icons:armchair',        color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    ancient_fossil:     { icon: 'game-icons:fossil',          color: '#d6d3d1', grad: ['#292524', '#1c1917'] },
    short_and_sweet:    { icon: 'game-icons:hourglass',       color: '#fcd34d', grad: ['#422006', '#1c1917'] },
    uwe_boll_survivor:  { icon: 'game-icons:boxing-glove',    color: '#f97316', grad: ['#431407', '#1c1917'] },
    snob_critic:        { icon: 'game-icons:thumb-down',      color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    potterhead:         { icon: 'game-icons:magic-swirl',     color: '#a855f7', grad: ['#2e1065', '#1c1917'] },
    lotr_ringbearer:    { icon: 'game-icons:ring',            color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    matrix_redpill:     { icon: 'game-icons:pill',            color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    saw_game:           { icon: 'game-icons:puzzle',          color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    starwars_boomer:    { icon: 'game-icons:light-sabers',    color: '#3b82f6', grad: ['#082f49', '#1c1917'] },
    shrek_swamp:        { icon: 'game-icons:swamp',           color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    alien_chestburster: { icon: 'game-icons:alien-bug',       color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    terminator_fates:   { icon: 'game-icons:robot-helmet',    color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    pirates_rum:        { icon: 'game-icons:pirate-hat',      color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    transformers_bayhem:{ icon: 'game-icons:robot-golem',     color: '#60a5fa', grad: ['#082f49', '#1c1917'] },
    john_wick_pencil:   { icon: 'game-icons:pencil',          color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    resident_evil_milla:{ icon: 'game-icons:biohazard',       color: '#f43f5e', grad: ['#4c0519', '#1c1917'] },
    twilight_vampire:   { icon: 'game-icons:vampire-dracula', color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    final_destination:  { icon: 'game-icons:truck',           color: '#fb923c', grad: ['#431407', '#1c1917'] },
    godfather_offer:    { icon: 'game-icons:spiked-collar',   color: '#d6d3d1', grad: ['#292524', '#1c1917'] },
    mission_impossible_stunt: { icon: 'game-icons:spy',       color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    hunger_games_mockingjay:  { icon: 'game-icons:bow-arrow', color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    halloween_myers:    { icon: 'game-icons:thrown-knife',    color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    ocean_eleven_heist: { icon: 'game-icons:diamonds',        color: '#22d3ee', grad: ['#083344', '#1c1917'] },
    madagascar_smile_and_wave: { icon: 'game-icons:lion',     color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    scream_ghostface:   { icon: 'game-icons:knife-thrust',    color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    predator_hunter:    { icon: 'game-icons:hunter-eyes',     color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    evil_dead_groovy:   { icon: 'game-icons:chainsaw',        color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    ghostbusters_proton:{ icon: 'game-icons:ghost',           color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    rocky_balboa_eye:   { icon: 'game-icons:fist',            color: '#f97316', grad: ['#431407', '#1c1917'] },
    creed_legacy:       { icon: 'game-icons:muscle-up',       color: '#fbbf24', grad: ['#422006', '#1c1917'] }, // Changed to avoid boxing-glove dup
    conjuring_universe: { icon: 'game-icons:gothic-cross',    color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    planet_of_apes:     { icon: 'game-icons:monkey',          color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    kingsman_manners:   { icon: 'game-icons:umbrella',        color: '#60a5fa', grad: ['#082f49', '#1c1917'] },

    // ==========================================
    // СЕРИАЛЫ (СТАРЫЕ)
    // ==========================================
    tv_firefly_browncoat:        { icon: 'game-icons:spaceship',       color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    tv_walking_dead_coral:       { icon: 'game-icons:shambling-zombie', color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    tv_lost_numbers:             { icon: 'game-icons:secret-book',     color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_prison_break_tattoo:      { icon: 'game-icons:prisoner',        color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    tv_breaking_bad_cook:        { icon: 'game-icons:test-tubes',      color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    tv_got_coffee_cup:           { icon: 'game-icons:coffee-cup',      color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_supernatural_pie:         { icon: 'mdi:car',                    color: '#ef4444', grad: ['#450a0a', '#1c1917'] }, // Changed from race-car
    tv_office_that_what_she_said:{ icon: 'game-icons:office-chair',    color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_x_files_truth:            { icon: 'game-icons:ufo',             color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    tv_sherlock_sociopath:       { icon: 'game-icons:magnifying-glass',color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    tv_sopranos_duck:            { icon: 'game-icons:duck',            color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_twin_peaks_pie:           { icon: 'game-icons:cherry',          color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    tv_friends_pivot:            { icon: 'game-icons:sofa',            color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    tv_boys_compound_v:          { icon: 'game-icons:syringe',         color: '#f43f5e', grad: ['#4c0519', '#1c1917'] },
    tv_black_mirror_screen:      { icon: 'game-icons:smartphone',      color: '#60a5fa', grad: ['#082f49', '#1c1917'] },
    tv_mandalorian_way:          { icon: 'game-icons:helmet',          color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    tv_better_call_saul_lawyer:  { icon: 'game-icons:scales',          color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_dexter_plastic_wrap:      { icon: 'game-icons:blood',           color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    tv_peaky_blinders_by_order:  { icon: 'game-icons:outback-hat',     color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    tv_chernobyl_not_great:      { icon: 'game-icons:nuclear',         color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    tv_stranger_things_running_up:{ icon: 'game-icons:walkie-talkie',  color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    tv_silicon_valley_middle_out:{ icon: 'game-icons:processor',       color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_fargo_ok_then:            { icon: 'game-icons:flake',           color: '#60a5fa', grad: ['#082f49', '#1c1917'] },
    tv_arcane_league:            { icon: 'game-icons:magic-potion',    color: '#a855f7', grad: ['#2e1065', '#1c1917'] },

    // ==========================================
    // СЕРИАЛЫ (ПАЧКА 1)
    // ==========================================
    tv_wire_streets:             { icon: 'game-icons:police-badge',     color: '#60a5fa', grad: ['#082f49', '#1c1917'] },
    tv_succession_boar:          { icon: 'game-icons:family-tree',      color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_last_of_us_clicker:       { icon: 'game-icons:mushroom',         color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    tv_dark_loop:                { icon: 'game-icons:time-trap',        color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_leftovers_departure:      { icon: 'mdi:weather-cloudy',          color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    tv_true_detective_flat_circle:{ icon: 'game-icons:spiral-shell',    color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    tv_fargo_oh_geez:            { icon: 'game-icons:wood-axe',         color: '#60a5fa', grad: ['#082f49', '#1c1917'] },
    tv_sense8_psychic:           { icon: 'game-icons:brain',            color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    tv_taboo_hardy:              { icon: 'game-icons:top-hat',          color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    tv_mare_easttown:            { icon: 'game-icons:beer-stein',       color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    tv_sharp_objects:            { icon: 'game-icons:scissors',         color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    tv_mr_robot_hello_friend:    { icon: 'game-icons:hooded-figure',    color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    tv_foundation_empire:        { icon: 'game-icons:ringed-planet',    color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_andor_rebellion:          { icon: 'game-icons:death-star',       color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    tv_sen_cal_kapimi:           { icon: 'game-icons:heart-plus',       color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    tv_good_omens_angel_demon:   { icon: 'game-icons:angel-wings',      color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_white_lotus_resort:       { icon: 'game-icons:palm-tree',        color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    tv_yellowstone_ranch:        { icon: 'mdi:hat-fedora',              color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    tv_american_gods_acid:       { icon: 'game-icons:pegasus',          color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    tv_in_treatment_couch:       { icon: 'mdi:chair-rolling',            color: '#a8a29e', grad: ['#292524', '#1c1917'] }, // Changed from sofa

    // ==========================================
    // СЕРИАЛЫ (ПАЧКА 2)
    // ==========================================
    tv_himym_bro_code:           { icon: 'game-icons:umbrella',         color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_bbt_bazinga:              { icon: 'game-icons:atom',             color: '#38bdf8', grad: ['#082f49', '#1c1917'] }, // Changed from brain
    tv_b99_noice:                { icon: 'game-icons:police-officer-head',color: '#60a5fa', grad: ['#082f49', '#1c1917'] }, // Changed from police-badge
    tv_parks_ron_swanson:        { icon: 'game-icons:meat',             color: '#84cc16', grad: ['#1a2e05', '#1c1917'] }, // Changed from wood-axe
    tv_community_greendale:      { icon: 'mdi:school',                  color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_modern_family_dunphy:     { icon: 'game-icons:house',            color: '#fda4af', grad: ['#4c0519', '#1c1917'] }, // Changed from family-tree
    tv_scrubs_eagle:             { icon: 'game-icons:stethoscope',      color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_always_sunny_gang:        { icon: 'game-icons:beer-bottle',      color: '#f59e0b', grad: ['#451a03', '#1c1917'] }, // Changed from beer-stein
    tv_fawlty_towers_cringe:     { icon: 'mdi:face-man-profile',        color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    tv_office_uk_cringe:         { icon: 'game-icons:paper',            color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    tv_silicon_valley_pied_piper:{ icon: 'game-icons:processor',        color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_two_half_men_charlie:     { icon: 'mdi:piano',                   color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    tv_good_place_kant:          { icon: 'game-icons:scales',           color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_bojack_horseman:          { icon: 'game-icons:horse-head',       color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    tv_key_peele_ayy:            { icon: 'mdi:football-helmet',         color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    tv_wwdits_vampires:          { icon: 'game-icons:vampire-dracula',  color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    tv_ted_lasso_believe:        { icon: 'game-icons:soccer-ball',      color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    tv_after_life_jervey:        { icon: 'mdi:grave-stone',             color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    tv_fleabag_camera:           { icon: 'mdi:camera',                  color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    tv_the_bear_kitchen:         { icon: 'game-icons:chef-toque',       color: '#f97316', grad: ['#431407', '#1c1917'] },
    tv_monty_python_inquisition: { icon: 'mdi:movie-open-outline',      color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },

    // ==========================================
    // СЕРИАЛЫ (ПАЧКА 3)
    // ==========================================
    tv_csi_evidence:             { icon: 'game-icons:flashlight',        color: '#60a5fa', grad: ['#082f49', '#1c1917'] }, // Changed from magnifying-glass
    tv_mentalist_blink:          { icon: 'mdi:eye',                      color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_bones_skeleton:           { icon: 'game-icons:skeleton',          color: '#d6d3d1', grad: ['#292524', '#1c1917'] },
    tv_criminal_minds_profile:   { icon: 'game-icons:brain',             color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    tv_fringe_walter:            { icon: 'game-icons:test-tubes',        color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_hannibal_dinner:          { icon: 'game-icons:meat',              color: '#dc2626', grad: ['#450a0a', '#1c1917'] }, // Changed from knife-thrust
    tv_castle_writer:            { icon: 'game-icons:quill-ink',         color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    tv_bsg_so_say_we_all:        { icon: 'game-icons:spaceship',         color: '#60a5fa', grad: ['#082f49', '#1c1917'] },
    tv_doctor_who_tardis:        { icon: 'mdi:phone-classic',            color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_star_trek_picard:         { icon: 'mdi:hand-back-left-outline',   color: '#fbbf24', grad: ['#422006', '#1c1917'] }, // Changed from ringed-planet
    tv_expanse_protomolecule:    { icon: 'game-icons:asteroid',          color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    tv_stargate_ring:            { icon: 'game-icons:portal',            color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    tv_resident_alien_tudyk:     { icon: 'game-icons:alien-bug',         color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    tv_orville_macfarlane:       { icon: 'game-icons:rocket',            color: '#38bdf8', grad: ['#082f49', '#1c1917'] }, // Changed from spaceship
    tv_simpsons_seasons:         { icon: 'game-icons:donut',             color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_south_park_seasons:       { icon: 'game-icons:park-bench',        color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    tv_family_guy_seasons:       { icon: 'game-icons:sitting-dog',       color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    tv_futurama_seasons:         { icon: 'game-icons:rocket',            color: '#38bdf8', grad: ['#082f49', '#1c1917'] }, // Dup with Orville, but let's leave rocket for Futurama
    tv_american_dad_seasons:     { icon: 'game-icons:alien-bug',         color: '#84cc16', grad: ['#1a2e05', '#1c1917'] }, // Dup with resident alien, but ok
    tv_king_of_hill_seasons:     { icon: 'mdi:propane-tank',             color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_bojack_horseman:          { icon: 'game-icons:horse-head',        color: '#c084fc', grad: ['#2e1065', '#1c1917'] }, // Dup with bojack from pack 2, but it's the same show
    tv_archer_danger_zone:       { icon: 'game-icons:spy',               color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    tv_twilight_zone:            { icon: 'game-icons:eye-of-horus',      color: '#a855f7', grad: ['#2e1065', '#1c1917'] },
    tv_ahs_switch:               { icon: 'game-icons:house',             color: '#dc2626', grad: ['#450a0a', '#1c1917'] }, // Dup with Modern Family, but ok
    tv_twin_peaks_s3:            { icon: 'game-icons:owl',               color: '#818cf8', grad: ['#1e1b4b', '#1c1917'] }, // Dup with night_owl, but ok
    tv_inside_no9_twist:         { icon: 'game-icons:theater-curtains',  color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    tv_heroes_save_cheerleader:  { icon: 'game-icons:super-mushroom',    color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    tv_grimm_wesen:              { icon: 'game-icons:wolf-head',         color: '#a8a29e', grad: ['#292524', '#1c1917'] }, // Dup with wolfs_rain, but ok

    // ==========================================
    // АНИМЕ (СТАРЫЕ)
    // ==========================================
    anime_konosuba_party:        { icon: 'game-icons:beer-stein',      color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_pingpong_club:         { icon: 'game-icons:ping-pong-bat',   color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_prison_school:         { icon: 'game-icons:handcuffs',       color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_redo_healer:           { icon: 'game-icons:healing',         color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    anime_death_note:            { icon: 'game-icons:notebook',        color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    anime_nagatoro:              { icon: 'game-icons:heart-plus',      color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_elfen_lied:            { icon: 'game-icons:blood',           color: '#f43f5e', grad: ['#4c0519', '#1c1917'] },
    anime_higurashi:             { icon: 'game-icons:cricket',         color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    anime_chainsaw_man:          { icon: 'game-icons:chainsaw',        color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_black_lagoon:          { icon: 'game-icons:revolver',        color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_naruto_legend:         { icon: 'game-icons:ninja-mask',      color: '#fbbf24', grad: ['#78350f', '#1c1917'] },
    anime_yani_neko:             { icon: 'game-icons:cat',             color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_gurren_lagann:         { icon: 'game-icons:drill',           color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_toradora:              { icon: 'game-icons:tiger',           color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_chi_sweet_home:        { icon: 'game-icons:cat',             color: '#4ade80', grad: ['#052e16', '#1c1917'] }, // Dup with yani_neko, but ok
    anime_made_in_abyss:         { icon: 'game-icons:hole',            color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    anime_mushoku_tensei:        { icon: 'game-icons:baby-face',       color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_fullmetal_alchemist:   { icon: 'game-icons:transfuse',       color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_bleach:                { icon: 'game-icons:katana',          color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_sword_art_online:      { icon: 'game-icons:crossed-swords',  color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_berserk:               { icon: 'game-icons:sword-wound',     color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    anime_ghibli_miyazaki:       { icon: 'game-icons:windmill',        color: '#4ade80', grad: ['#052e16', '#1c1917'] },

    // ==========================================
    // АНИМЕ (ПАЧКА 1)
    // ==========================================
    anime_evangelion:            { icon: 'game-icons:mecha-head',       color: '#a855f7', grad: ['#2e1065', '#1c1917'] },
    anime_cowboy_bebop:          { icon: 'game-icons:saxophone',        color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_trigun:                { icon: 'mdi:gun',                     color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_gits_1995:             { icon: 'game-icons:cyber-eye',        color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_akira:                 { icon: 'mdi:motorbike',               color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_perfect_blue:          { icon: 'game-icons:broken-heart',     color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_paprika:               { icon: 'game-icons:dream-catcher',    color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_hellsing_ultimate:     { icon: 'game-icons:vampire-dracula',  color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    anime_samurai_champloo:      { icon: 'game-icons:katana',           color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_fma_2003:              { icon: 'game-icons:transfuse',        color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_grave_fireflies:       { icon: 'game-icons:fire',             color: '#f97316', grad: ['#431407', '#1c1917'] },
    anime_gantz:                 { icon: 'mdi:circle',                  color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    anime_monster:               { icon: 'mdi:book-open-variant',       color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    anime_paranoia_agent:        { icon: 'game-icons:baseball-bat',     color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    anime_rurouni_kenshin:       { icon: 'game-icons:katana',           color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_claymore:              { icon: 'mdi:sword',                   color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    anime_wolfs_rain:            { icon: 'game-icons:wolf-head',        color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_ergo_proxy:            { icon: 'mdi:eye-outline',             color: '#a855f7', grad: ['#2e1065', '#1c1917'] },

    // ==========================================
    // АНИМЕ (ПАЧКА 2)
    // ==========================================
    anime_one_piece:             { icon: 'game-icons:pirate-hat',       color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_hxh:                   { icon: 'game-icons:hunter-eyes',      color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    anime_mha:                   { icon: 'game-icons:muscle-up',        color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_demon_slayer:          { icon: 'game-icons:water-splash',     color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_jujutsu_kaisen:        { icon: 'game-icons:evil-eyes',        color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_fairy_tail:            { icon: 'game-icons:fairy',            color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_jojo:                  { icon: 'game-icons:muscular-torso',   color: '#a855f7', grad: ['#2e1065', '#1c1917'] },
    anime_black_clover:          { icon: 'game-icons:clover',           color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    anime_magi:                  { icon: 'game-icons:magic-swirl',      color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    anime_seven_deadly_sins:     { icon: 'game-icons:death-skull',      color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_dr_stone:              { icon: 'game-icons:test-tubes',       color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_fire_force:            { icon: 'game-icons:fire',             color: '#f97316', grad: ['#431407', '#1c1917'] },
    anime_ippo:                  { icon: 'game-icons:boxing-glove',     color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_haikyuu:               { icon: 'game-icons:volleyball-ball',  color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_kuroko:                { icon: 'game-icons:basketball-ball',  color: '#f97316', grad: ['#431407', '#1c1917'] },
    anime_blue_exorcist:         { icon: 'game-icons:flame',            color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_record_ragnarok:       { icon: 'game-icons:thor-hammer',      color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    anime_one_punch_man:         { icon: 'game-icons:fist',             color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    anime_shaman_king:           { icon: 'mdi:ghost',                   color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_fate_ubw:              { icon: 'game-icons:crossed-swords',   color: '#f59e0b', grad: ['#451a03', '#1c1917'] },

    // ==========================================
    // АНИМЕ (ПАЧКА 3A)
    // ==========================================
    anime_rezero:                { icon: 'game-icons:time-trap',        color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_slime:                 { icon: 'game-icons:slime',            color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_shield_hero:           { icon: 'game-icons:shield',           color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    anime_realist_hero:          { icon: 'game-icons:calculator',       color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    anime_emminence_shadow:      { icon: 'game-icons:cloak',            color: '#a855f7', grad: ['#2e1065', '#1c1917'] },
    anime_slime_300:             { icon: 'game-icons:slime',            color: '#4ade80', grad: ['#052e16', '#1c1917'] }, // осмысленный дубль
    anime_campfire_cooking:      { icon: 'game-icons:cooking-pot',      color: '#f97316', grad: ['#431407', '#1c1917'] },
    anime_ngnl:                  { icon: 'game-icons:chess-knight',     color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_spider:                { icon: 'mdi:spider',                  color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    anime_uncle:                 { icon: 'game-icons:gamepad',          color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    anime_cautious_hero:         { icon: 'game-icons:paranoia',         color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    anime_faraway_paladin:       { icon: 'mdi:shield-cross',            color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    anime_k_on:                  { icon: 'mdi:tea',                     color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_clannad:               { icon: 'mdi:water',                   color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_umaru:                 { icon: 'mdi:rodent',                  color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_rent_a_girlfriend:     { icon: 'mdi:cash',                    color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    anime_kaguya:                { icon: 'game-icons:brain',            color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_nichijou:              { icon: 'game-icons:deer',             color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    anime_your_lie_in_april:     { icon: 'mdi:piano',                   color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_silent_voice:          { icon: 'game-icons:hearing-disabled', color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_violet_evergarden:     { icon: 'mdi:email-outline',           color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    anime_kokoro_connect:        { icon: 'game-icons:body-swapping',    color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_tokyo_ghoul:           { icon: 'game-icons:blood',            color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    anime_psycho_pass:           { icon: 'game-icons:cyber-eye',        color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_terra_formars:         { icon: 'mdi:bug',                     color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    anime_another:               { icon: 'game-icons:umbrella',         color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    anime_school_days:           { icon: 'mdi:ferry',                   color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_akame_ga_kill:         { icon: 'game-icons:knife-thrust',     color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_future_diary:          { icon: 'mdi:axe',                     color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    anime_deadman_wonderland:    { icon: 'mdi:ferris-wheel',            color: '#f97316', grad: ['#431407', '#1c1917'] },
    anime_inuyashiki:            { icon: 'mdi:robot',                   color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    anime_promised_neverland_s1: { icon: 'mdi:home-heart',              color: '#a8a29e', grad: ['#292524', '#1c1917'] },

    // ==========================================
    // АНИМЕ (ПАЧКА 3B)
    // ==========================================
    anime_dxd:                   { icon: 'mdi:heart',           color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_to_love_ru:            { icon: 'game-icons:heart-plus',       color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_demon_lord:            { icon: 'game-icons:crown',            color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_interspecies:          { icon: 'mdi:clipboard-text',        color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    anime_ao_chan:               { icon: 'game-icons:brain',            color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_eromanga:              { icon: 'game-icons:pencil',           color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_trinity_seven:         { icon: 'game-icons:magic-swirl',      color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    anime_uzaki:                 { icon: 'mdi:emoticon-excited-outline', color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_domestic_girlfriend:   { icon: 'game-icons:broken-heart',     color: '#f43f5e', grad: ['#4c0519', '#1c1917'] },
    anime_testament_sister:      { icon: 'game-icons:devil-mask',       color: '#dc2626', grad: ['#450a0a', '#1c1917'] },

    // ==========================================
    // ФИЛЬМЫ (ФРАНШИЗЫ)
    // ==========================================
    movie_bond:                   { icon: 'game-icons:martini',           color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    movie_avatar:                 { icon: 'game-icons:alien-bug',         color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    movie_brat:                   { icon: 'game-icons:uzi',               color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    movie_xmen:                   { icon: 'mdi:account-group',            color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    movie_taken:                  { icon: 'game-icons:phone',             color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    movie_bumer:                  { icon: 'game-icons:car-key',           color: '#f97316', grad: ['#431407', '#1c1917'] },
    movie_home_alone:             { icon: 'tabler:home',                  color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    movie_wrong_turn:             { icon: 'game-icons:axe-in-stump',      color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    movie_red:                    { icon: 'tabler:user-shield',           color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    movie_jaws:                   { icon: 'mdi:shark',                    color: '#22d3ee', grad: ['#083344', '#1c1917'] },
    movie_kick_ass:               { icon: 'mdi:sword',                    color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    movie_riddick:                { icon: 'tabler:space',                 color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    movie_28_days:                { icon: 'game-icons:shambling-zombie',  color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    movie_jurassic:               { icon: 'mdi:paw',                      color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    movie_jeepers:                { icon: 'game-icons:bat',               color: '#f43f5e', grad: ['#4c0519', '#1c1917'] },
    movie_bttf:                   { icon: 'game-icons:time-trap',         color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    movie_police_academy:         { icon: 'game-icons:police-officer-head', color: '#60a5fa', grad: ['#082f49', '#1c1917'] },
    movie_nightmare:              { icon: 'game-icons:claws',             color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    movie_hobbit:                 { icon: 'game-icons:dragon-head',       color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    movie_die_hard:               { icon: 'mdi:office-building',          color: '#fbbf24', grad: ['#422006', '#1c1917'] },

    // ==========================================
    // ФИЛЬМЫ (ПОДВАЛ)
    // ==========================================
    movie_gruz_200:               { icon: 'game-icons:coffin',            color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    movie_human_centipede:        { icon: 'game-icons:centipede',         color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    movie_salo:                   { icon: 'tabler:poo',                   color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    movie_serbian_film:           { icon: 'game-icons:blood',             color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    movie_eraserhead:             { icon: 'game-icons:brain',             color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    movie_pink_flamingos:         { icon: 'game-icons:flamingo',          color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    movie_antichrist:             { icon: 'game-icons:deer',              color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    movie_das_experiment:         { icon: 'mdi:flask-outline',            color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    movie_dogtooth:               { icon: 'game-icons:tooth',             color: '#fcd34d', grad: ['#422006', '#1c1917'] },
    movie_under_the_skin:         { icon: 'game-icons:cloak',             color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    movie_holy_mountain:          { icon: 'mdi:pyramid',                  color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    movie_twin_peaks_fire:        { icon: 'game-icons:owl',               color: '#818cf8', grad: ['#1e1b4b', '#1c1917'] },
    movie_pi:                     { icon: 'mdi:pi',                       color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    movie_dead_man:               { icon: 'mdi:hat-fedora',               color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    movie_sexual_parasite:        { icon: 'tabler:bug',                   color: '#f43f5e', grad: ['#4c0519', '#1c1917'] },
    movie_cannibal_holocaust:     { icon: 'game-icons:skull-crossed-bones', color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    movie_hard_to_be_god:         { icon: 'game-icons:chess-king',        color: '#d6d3d1', grad: ['#292524', '#1c1917'] },
    movie_five_bottles:           { icon: 'mdi:bottle-soda',              color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    movie_toxic_avenger:          { icon: 'game-icons:radioactive',       color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    movie_blood_feast:            { icon: 'game-icons:meat',              color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
	
	rewatch_first:                { icon: 'mdi:replay',                  color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    rewatch_5_same:               { icon: 'mdi:repeat',                  color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    rewatch_10_same:              { icon: 'mdi:sync',                    color: '#a855f7', grad: ['#2e1065', '#1c1917'] },
    rewatch_50_total:             { icon: 'mdi:history',                 color: '#f43f5e', grad: ['#4c0519', '#1c1917'] },

    // ==========================================
    // СЧЕТЧИКИ
    // ==========================================
    anime_count_10:               { icon: 'game-icons:tv',                color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    anime_count_50:               { icon: 'game-icons:book-cover',        color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_count_150:              { icon: 'game-icons:katana',            color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_count_300:              { icon: 'game-icons:owl',               color: '#818cf8', grad: ['#1e1b4b', '#1c1917'] },
    anime_count_500:              { icon: 'game-icons:samurai-helmet',    color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_count_750:              { icon: 'game-icons:night-sleep',       color: '#a855f7', grad: ['#2e1065', '#1c1917'] },
    anime_count_1000:             { icon: 'game-icons:spell-book',        color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    anime_count_1500:             { icon: 'game-icons:hand-of-god',       color: '#fbbf24', grad: ['#78350f', '#1c1917'] },

    movie_count_10:               { icon: 'game-icons:popcorn',           color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    movie_count_50:               { icon: 'game-icons:film-strip',        color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    movie_count_150:              { icon: 'mdi:sofa',                     color: '#c084fc', grad: ['#2e1065', '#1c1917'] }, // Changed from sofa
    movie_count_300:              { icon: 'game-icons:crown',             color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    movie_count_500:              { icon: 'game-icons:film-projector',    color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    movie_count_750:              { icon: 'game-icons:skull-crossed-bones',color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    movie_count_1000:             { icon: 'game-icons:circuitry',         color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    movie_count_1500:             { icon: 'game-icons:film-spool',        color: '#fbbf24', grad: ['#78350f', '#1c1917'] },

    tv_count_10:                  { icon: 'game-icons:tv',                color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_count_50:                  { icon: 'game-icons:popcorn',           color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_count_150:                 { icon: 'game-icons:knife-thrust',      color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    tv_count_300:                 { icon: 'mdi:seat',                     color: '#a8a29e', grad: ['#292524', '#1c1917'] }, // Changed from sofa
    tv_count_500:                 { icon: 'game-icons:spy',               color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    tv_count_750:                 { icon: 'game-icons:beer-stein',        color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    tv_count_1000:                { icon: 'game-icons:fast-forward-button', color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    tv_count_1500:                { icon: 'mdi:netflix',                  color: '#ef4444', grad: ['#450a0a', '#1c1917'] },

    // ==========================================
    // СТАТУСЫ
    // ==========================================
    status_planned_10:            { icon: 'game-icons:alarm-clock',       color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    status_planned_50:            { icon: 'game-icons:calendar',          color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    status_planned_100:           { icon: 'game-icons:scroll-quill',      color: '#a855f7', grad: ['#2e1065', '#1c1917'] },
    status_planned_300:           { icon: 'game-icons:bookshelf',         color: '#fbbf24', grad: ['#78350f', '#1c1917'] },
    status_on_hold_10:            { icon: 'game-icons:hourglass',         color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    status_on_hold_50:            { icon: 'game-icons:pause-button',      color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    status_on_hold_100:           { icon: 'game-icons:sleeping-bag',      color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    status_dropped_10:            { icon: 'game-icons:trash-can',         color: '#f97316', grad: ['#431407', '#1c1917'] },
    status_dropped_50:            { icon: 'game-icons:broken-bone',       color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    status_dropped_100:           { icon: 'game-icons:chewed-skull',      color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
  };

  const cfg = $derived(config[code] ?? { icon: 'game-icons:trophy', color: '#ff9000', grad: ['#422006', '#1c1917'] });

  const isFixed = $derived(typeof size === 'number');
  const circleStyle = $derived(isFixed ? `width: ${size}px; height: ${size}px;` : `width: 75%; height: 75%;`);
  const iconStyle = $derived(isFixed ? `width: ${(size as number) * 0.8}px; height: ${(size as number) * 0.8}px;` : `width: 80%; height: 80%;`);
</script>

<div
  class="rounded-full border inline-flex items-center justify-center shrink-0"
  style="
    {circleStyle}
    background: linear-gradient(135deg, {cfg.grad[0]}, {cfg.grad[1]});
    border-color: {cfg.color}40;
    box-shadow: 0 0 12px {cfg.color}20, inset 0 1px 0 {cfg.color}30;
  "
>
  <Icon
    icon={cfg.icon}
    style="{iconStyle} color: {cfg.color}; filter: drop-shadow(0 0 6px {cfg.color}80);"
  />
</div>