<script lang="ts">
  import Icon from '@iconify/svelte';

  interface Props {
    code: string;
    size?: number | string;
  }

  let { code, size }: Props = $props();

  const config: Record<string, { icon: string; color: string; grad: [string, string] }> = {
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
    
    godfather_offer:         { icon: 'game-icons:spiked-collar', color: '#d6d3d1', grad: ['#292524', '#1c1917'] },
    mission_impossible_stunt:{ icon: 'game-icons:spy',           color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    hunger_games_mockingjay: { icon: 'game-icons:bow-arrow',     color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    halloween_myers:         { icon: 'game-icons:thrown-knife',  color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    ocean_eleven_heist:      { icon: 'game-icons:diamonds',      color: '#22d3ee', grad: ['#083344', '#1c1917'] },
    madagascar_smile_and_wave:{ icon: 'game-icons:lion',         color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    scream_ghostface:        { icon: 'game-icons:knife-thrust',  color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    
    predator_hunter:     { icon: 'game-icons:hunter-eyes',    color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    evil_dead_groovy:    { icon: 'game-icons:chainsaw',       color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
    ghostbusters_proton: { icon: 'game-icons:ghost',          color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    rocky_balboa_eye:    { icon: 'game-icons:boxing-glove',   color: '#f97316', grad: ['#431407', '#1c1917'] },
    creed_legacy:        { icon: 'game-icons:fist',           color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    conjuring_universe:  { icon: 'game-icons:gothic-cross',   color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    planet_of_apes:      { icon: 'game-icons:monkey',         color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    kingsman_manners:    { icon: 'game-icons:umbrella',       color: '#60a5fa', grad: ['#082f49', '#1c1917'] },

    tv_firefly_browncoat:        { icon: 'game-icons:spaceship',       color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    tv_walking_dead_coral:       { icon: 'game-icons:shambling-zombie', color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    tv_lost_numbers:             { icon: 'game-icons:secret-book',     color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_prison_break_tattoo:      { icon: 'game-icons:prisoner',        color: '#94a3b8', grad: ['#1e293b', '#1c1917'] },
    tv_breaking_bad_cook:        { icon: 'game-icons:test-tubes',      color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    tv_got_coffee_cup:           { icon: 'game-icons:coffee-cup',      color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_supernatural_pie:         { icon: 'game-icons:race-car',        color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
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
    anime_chi_sweet_home:        { icon: 'game-icons:cat',             color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    anime_made_in_abyss:         { icon: 'game-icons:hole',            color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
	
	anime_mushoku_tensei:         { icon: 'game-icons:baby-face',      color: '#fda4af', grad: ['#4c0519', '#1c1917'] },
    anime_fullmetal_alchemist:    { icon: 'game-icons:transfuse',      color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    anime_bleach:                 { icon: 'game-icons:katana',         color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    anime_sword_art_online:       { icon: 'game-icons:crossed-swords',          color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    anime_berserk:                { icon: 'game-icons:sword-wound',    color: '#dc2626', grad: ['#450a0a', '#1c1917'] },
	
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
    movie_count_150:              { icon: 'game-icons:sofa',              color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    movie_count_300:              { icon: 'game-icons:crown',             color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    movie_count_500:              { icon: 'game-icons:film-projector',    color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    movie_count_750:              { icon: 'game-icons:skull-crossed-bones',color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    movie_count_1000:             { icon: 'game-icons:circuitry',         color: '#84cc16', grad: ['#1a2e05', '#1c1917'] },
    movie_count_1500:             { icon: 'game-icons:film-spool',        color: '#fbbf24', grad: ['#78350f', '#1c1917'] },
	
	tv_count_10:                  { icon: 'game-icons:tv',                color: '#38bdf8', grad: ['#082f49', '#1c1917'] },
    tv_count_50:                  { icon: 'game-icons:popcorn',           color: '#fbbf24', grad: ['#422006', '#1c1917'] },
    tv_count_150:                 { icon: 'game-icons:knife-thrust',      color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
    tv_count_300:                 { icon: 'game-icons:sofa',              color: '#a8a29e', grad: ['#292524', '#1c1917'] },
    tv_count_500:                 { icon: 'game-icons:spy',               color: '#c084fc', grad: ['#2e1065', '#1c1917'] },
    tv_count_750:                 { icon: 'game-icons:beer-stein',        color: '#f59e0b', grad: ['#451a03', '#1c1917'] },
    tv_count_1000:                { icon: 'game-icons:fast-forward-button', color: '#4ade80', grad: ['#052e16', '#1c1917'] },
    tv_count_1500:                { icon: 'game-icons:tv',                color: '#ef4444', grad: ['#450a0a', '#1c1917'] },
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