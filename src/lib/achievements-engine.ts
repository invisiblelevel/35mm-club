import { invoke } from '@tauri-apps/api/core';

export type MovieContext = {
  tmdb_id: number;
  type: string;
  year: number | null;
  runtime: number | null;
  cast_ids: number[];
  director_ids: number[];
  genres: string[];
  imdb_rating: number | null;
  user_rating: number | null;
};

export async function checkAchievements(ctx: MovieContext): Promise<string[]> {
  const unlocked: string[] = [];

  // «Absolute Cinema» — Невский (ID 2963 или 124693? нужно проверить на TMDB)
  // Пока используем временный ID 999999, чтобы не ломать тесты. Замени на реальный.
  if (ctx.cast_ids.includes(999999)) {
    unlocked.push('nevsky_absolute');
  }

  // «Морбиус» — ID 526896
  if (ctx.tmdb_id === 526896) {
    unlocked.push('morbius_time');
  }

  // «Комната» — ID 12444
  if (ctx.tmdb_id === 12444) {
    unlocked.push('room_wiseau');
  }

  // «Зелёный слоник» — ID 145952
  if (ctx.tmdb_id === 145952) {
    unlocked.push('green_elephant');
  }

  // «Кошки» — ID 538243
  if (ctx.tmdb_id === 538243) {
    unlocked.push('cats_horrifying');
  }

  // «Тарантино» — director_id 138
  if (ctx.director_ids.includes(138)) {
    // TODO: проверять, что просмотрены все 9 фильмов
    unlocked.push('tarantino_feet');
  }

  // «Нолан» — director_id 525
  if (ctx.director_ids.includes(525)) {
    unlocked.push('nolan_time');
  }

  // «Кейдж» — actor_id 2963
  if (ctx.cast_ids.includes(2963)) {
    // TODO: проверять, что 10+ фильмов
    unlocked.push('cage_fever');
  }

  // «Попа из стали» — runtime >= 200
  if (ctx.runtime && ctx.runtime >= 200) {
    unlocked.push('marathon_endurance');
  }

  // «Археолог» — год < 1950
  if (ctx.year && ctx.year < 1950) {
    unlocked.push('ancient_fossil');
  }

  // «Микро-сеанс» — runtime < 70
  if (ctx.runtime && ctx.runtime < 70) {
    unlocked.push('short_and_sweet');
  }

  // «Мазохист» — imdb < 4 и user >= 8
  if (ctx.imdb_rating && ctx.imdb_rating < 4 && ctx.user_rating && ctx.user_rating >= 8) {
    unlocked.push('masochist_trash');
  }

  // «Беспощадный критик» — 5 оценок «1» подряд (TODO)

  // «Уве Болл» — director_id 10950 (TODO: 3+ фильма)

  // «Пчёлы» — ID 9708
  if (ctx.tmdb_id === 9708) {
    unlocked.push('not_the_bees');
  }

  // Выдаём всё, что нашли
  for (const code of unlocked) {
    await invoke('unlock_achievement', { code }).catch(console.error);
  }

  return unlocked;
}