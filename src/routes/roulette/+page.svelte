<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { goto } from '$app/navigation';
  import confetti from 'canvas-confetti';
  import {
    Film, Tv, Sparkles, Dices, RotateCw, Bookmark, ExternalLink,
    Star, Clock, ListVideo, AlertCircle
  } from 'lucide-svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import RouletteWheel, { type WheelItem } from '$lib/components/RouletteWheel.svelte';

  type MediaType = 'movie' | 'tv' | 'anime';
  type RollResult = {
    source: string; media_type: string; tmdb_id: number | null; mal_id: number | null;
    title: string; original_title: string | null; overview: string | null;
    poster_url: string | null; year: number | null; rating: number | null;
    runtime: number | null; episodes: number | null; format: string | null; genres: string[];
  };
  type HistoryItem = RollResult & { ts: number };

  const SECTORS = 8;
  const TABS: { type: MediaType; key: string; icon: typeof Film; accent: string }[] = [
    { type: 'movie', key: 'roulette.tab_movies', icon: Film, accent: '#ff9000' },
    { type: 'tv', key: 'roulette.tab_tv', icon: Tv, accent: '#3b82f6' },
    { type: 'anime', key: 'roulette.tab_anime', icon: Sparkles, accent: '#ec4899' },
  ];

  let activeTab = $state<MediaType>('movie');
  let spinning = $state(false);
  let loading = $state(false);
  let error = $state<string | null>(null);
  let result = $state<RollResult | null>(null);
  let winnerIndex = $state(0);
  let wheelItems = $state<WheelItem[]>([]);
  let history = $state<HistoryItem[]>([]);
  let wheelRef: RouletteWheel;

  const accent = $derived(TABS.find(t => t.type === activeTab)?.accent ?? '#ff9000');

  function makePlaceholderItem(i: number): WheelItem {
    return { id: `placeholder-${i}`, poster_url: null, title: '' };
  }

  // Кэшируем постеры (TMDB → data: URL), чтобы WebView их отрисовал
  async function cachePoster(url: string | null): Promise<string | null> {
    if (!url) return null;
    if (url.startsWith('data:')) return url;
    try {
      return await invoke<string>('cache_cover', { url });
    } catch (e) {
      // если не удалось — возвращаем оригинал (может сработать, а может нет)
      return url;
    }
  }

  async function preloadPosters(items: RollResult[]): Promise<string[]> {
    const promises = items.map(async (item) => {
      const cached = await cachePoster(item.poster_url);
      return cached ?? '';
    });
    return Promise.all(promises);
  }

  function triggerConfetti() {
    const colors = [accent, '#ff9000', '#ffffff', '#ffcc00'];
    confetti({ particleCount: 120, spread: 100, origin: { y: 0.5 }, colors, zIndex: 9999, disableForReducedMotion: true, ticks: 200, gravity: 1.2, scalar: 1.2 });
    setTimeout(() => {
      confetti({ particleCount: 60, angle: 60, spread: 55, origin: { x: 0, y: 0.6 }, colors, zIndex: 9999, disableForReducedMotion: true });
      confetti({ particleCount: 60, angle: 120, spread: 55, origin: { x: 1, y: 0.6 }, colors, zIndex: 9999, disableForReducedMotion: true });
    }, 150);
  }

  let pendingResult: RollResult | null = null;
  let safetyTimer: ReturnType<typeof setTimeout> | null = null;

  async function doSpin() {
    if (spinning || loading) return;
    if (safetyTimer) clearTimeout(safetyTimer);
    error = null; loading = true; spinning = true;

    try {
      const res = await invoke<RollResult[]>('roll_random_batch', { mediaType: activeTab, locale: $locale, userApiKey: null });

      if (!res || res.length === 0) {
        throw new Error("Не удалось загрузить тайтлы. Попробуйте ещё раз.");
      }

      // КЭШИРУЕМ ВСЕ 8 ПОСТЕРОВ ПЕРЕД ЗАПУСКОМ КОЛЕСА
      const cachedUrls = await preloadPosters(res);

      wheelItems = Array.from({ length: SECTORS }, (_, i) => {
        const item = res[i];
        if (!item) return makePlaceholderItem(i);
        return {
          id: `w-${item.tmdb_id ?? item.mal_id}`,
          poster_url: cachedUrls[i] || null,
          title: item.title
        };
      });

      const winIdx = Math.floor(Math.random() * res.length);
      winnerIndex = winIdx;
      pendingResult = res[winIdx];

      await new Promise(r => setTimeout(r, 100));
      loading = false;
      
      wheelRef?.triggerSpin();

      safetyTimer = setTimeout(() => {
        if (spinning) {
          spinning = false;
          if (!result && pendingResult) { result = pendingResult; history = [{ ...pendingResult, ts: Date.now() }, ...history].slice(0, 5); }
        }
      }, 5000);

    } catch (e) {
      error = String(e); loading = false; spinning = false;
    }
  }

  function onSpinEnd() {
    if (safetyTimer) clearTimeout(safetyTimer);
    spinning = false;
    if (!pendingResult) return;
    result = pendingResult;
    history = [{ ...pendingResult, ts: Date.now() }, ...history].slice(0, 5);
    
    triggerConfetti();
    try { new Audio('/achivka.wav').play().catch(() => {}); } catch {}
    pendingResult = null;
  }

  function switchTab(type: MediaType) {
    if (spinning || loading) return;
    activeTab = type; result = null; error = null;
    wheelItems = Array.from({ length: SECTORS }, (_, i) => makePlaceholderItem(i));
    winnerIndex = 0; wheelRef?.resetRotation();
  }

  function openCard(item: RollResult) {
    if (item.media_type === 'anime' && item.mal_id) goto(`/anime/${item.mal_id}`);
    else if (item.tmdb_id) goto(`/title/${item.tmdb_id}?type=${item.media_type}`);
  }

  async function addToLibrary(item: RollResult, status: 'planned' | 'watching') {
    try {
      if (item.media_type === 'anime' && item.mal_id) {
        await invoke('add_anime_to_library', { payload: { mal_id: item.mal_id, title: item.title, title_original: item.original_title, description: item.overview, cover_url: item.poster_url, episodes: item.episodes, format: item.format, status, studio: null, year: item.year } });
      } else if (item.tmdb_id) {
        await invoke('add_to_library', { payload: { tmdb_id: item.tmdb_id, type_field: item.media_type, title: item.title, original_title: item.original_title, year: item.year, poster_path: item.poster_url, overview: item.overview, imdb_rating: item.rating, runtime: item.runtime, status, cast_ids: null, director_ids: null, producer_ids: null, production_company_ids: null, keyword_ids: null, collection_id: null, locale: $locale } });
      }
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
    } catch (e) { console.error('add_to_library failed:', e); }
  }

  function formatRating(r: number | null): string { return r == null ? '—' : r.toFixed(1); }
  function formatRuntime(min: number | null): string | null {
    if (min == null || min <= 0) return null;
    const h = Math.floor(min / 60), m = min % 60;
    return h === 0 ? `${m} ${$t('roulette.minutes_short')}` : `${h}ч ${m}м`;
  }
  function formatFormat(fmt: string | null): string | null {
    if (!fmt) return null;
    const map: Record<string, string> = { tv: 'format_tv', movie: 'format_movie', ova: 'format_ova', ona: 'format_ona', special: 'format_special', music: 'format_music' };
    return map[fmt.toLowerCase()] ? $t(`roulette.${map[fmt.toLowerCase()]}`) : fmt.toUpperCase();
  }

  onMount(() => { wheelItems = Array.from({ length: SECTORS }, (_, i) => makePlaceholderItem(i)); });
</script>

<svelte:head><title>{$t('roulette.title')} — 35mm Club</title></svelte:head>

<div class="relative max-w-4xl mx-auto">
  <div class="pointer-events-none fixed inset-0 -z-10 transition-all duration-700" style="background: radial-gradient(circle at 50% 30%, {accent}18 0%, transparent 55%);"></div>

  <PageHeader title={$t('roulette.title')} subtitle={$t('roulette.subtitle')} icon={Dices} />

  <div class="flex gap-2 mb-6 p-1 rounded-xl bg-surface border border-border">
    {#each TABS as tab}
      <button onclick={() => switchTab(tab.type)} disabled={spinning || loading} class="flex-1 flex items-center justify-center gap-2 px-3 py-2.5 rounded-lg text-sm font-medium transition-all duration-300 disabled:opacity-60" style={activeTab === tab.type ? `background: linear-gradient(180deg, ${tab.accent}, color-mix(in srgb, ${tab.accent} 70%, #000)); color: #0a0a0a; box-shadow: 0 0 16px ${tab.accent}55;` : `color: #a1a1aa;`}>
        <tab.icon size={16} /><span class="hidden sm:inline">{$t(tab.key)}</span>
      </button>
    {/each}
  </div>

  <div class="flex flex-col items-center relative">
    <RouletteWheel bind:this={wheelRef} items={wheelItems} {winnerIndex} {accent} showButton={false} onSpinEnd={onSpinEnd} />

    <button onclick={doSpin} disabled={spinning || loading} class="mt-8 px-10 py-3.5 rounded-xl font-display font-bold text-lg tracking-wide text-ink transition-all duration-200 hover:brightness-110 active:scale-[0.97] disabled:opacity-60 disabled:cursor-not-allowed flex items-center gap-2" style="background: linear-gradient(180deg, {accent}, color-mix(in srgb, {accent} 70%, #000)); box-shadow: 0 0 28px {accent}66, 0 8px 24px #00000080;">
      {#if loading}
        <span class="inline-block w-5 h-5 border-2 border-ink/30 border-t-ink rounded-full animate-spin"></span><span>{$t('roulette.loading')}</span>
      {:else if spinning}
        <RotateCw size={20} class="animate-spin" /><span>{$t('roulette.spinning')}</span>
      {:else if result}
        <RotateCw size={20} /><span>{$t('roulette.spin_again')}</span>
      {:else}
        <Dices size={20} /><span>{$t('roulette.spin')}</span>
      {/if}
    </button>

    {#if error}
      <div class="mt-4 max-w-md w-full flex items-start gap-2 p-3 rounded-lg bg-red-500/10 border border-red-500/30 text-sm text-red-300">
        <AlertCircle size={18} class="shrink-0 mt-0.5" />
        <div class="flex-1 min-w-0"><div class="font-medium">{$t('roulette.error')}</div><div class="text-xs text-red-400/80 mt-0.5 break-words">{error}</div></div>
      </div>
    {/if}
  </div>

  {#if result}
    <div class="mt-8 rounded-2xl bg-surface border overflow-hidden transition-all duration-500" style="border-color: {accent}55; box-shadow: 0 0 40px {accent}22; animation: card-bounce 0.5s cubic-bezier(0.34, 1.56, 0.64, 1);">
      <div class="flex flex-col sm:flex-row gap-4 p-4 sm:p-5">
        <div class="w-full sm:w-32 shrink-0"><div class="aspect-[2/3] rounded-xl overflow-hidden bg-ink-light relative">{#if result.poster_url}<img src={result.poster_url} alt={result.title} class="absolute inset-0 w-full h-full object-cover" referrerpolicy="no-referrer" />{:else}<div class="absolute inset-0 flex items-center justify-center text-zinc-600"><Dices size={40} /></div>{/if}</div></div>
        <div class="flex-1 min-w-0 flex flex-col">
          <div class="flex items-start justify-between gap-2">
            <div class="min-w-0 flex-1"><h2 class="font-display text-xl sm:text-2xl font-bold leading-tight truncate" style="color: {accent};">{result.title}</h2>{#if result.original_title && result.original_title !== result.title}<p class="text-sm text-zinc-500 truncate">{result.original_title}</p>{/if}</div>
            <span class="shrink-0 text-[10px] uppercase tracking-wider px-2 py-0.5 rounded bg-ink-light text-zinc-500 border border-border">{$t(`roulette.source_${result.source}`)}</span>
          </div>
          <div class="flex flex-wrap items-center gap-3 mt-2 text-sm text-zinc-400">
            {#if result.year}<span>{result.year}</span>{/if}
            {#if result.rating != null}<span class="flex items-center gap-1 text-gold"><Star size={14} fill="currentColor" />{formatRating(result.rating)}</span>{/if}
            {#if result.runtime}<span class="flex items-center gap-1"><Clock size={14} />{formatRuntime(result.runtime)}</span>{/if}
            {#if result.episodes}<span class="flex items-center gap-1"><ListVideo size={14} />{result.episodes} {$t('roulette.episodes_short')}</span>{/if}
            {#if formatFormat(result.format)}<span class="px-1.5 py-0.5 rounded bg-ink-light text-xs">{formatFormat(result.format)}</span>{/if}
          </div>
          {#if result.genres.length > 0}<div class="flex flex-wrap gap-1.5 mt-2">{#each result.genres.slice(0, 5) as g}<span class="text-[11px] px-2 py-0.5 rounded-full bg-ink-light text-zinc-400 border border-border">{g}</span>{/each}</div>{/if}
          {#if result.overview}<p class="text-sm text-zinc-400 mt-3 line-clamp-3">{result.overview}</p>{/if}
          <div class="flex flex-wrap gap-2 mt-auto pt-3">
            <button onclick={() => result && addToLibrary(result, 'planned')} class="flex items-center gap-1.5 px-3 py-2 rounded-lg bg-ink-light border border-border text-sm text-zinc-300 hover:border-gold/50 hover:text-gold transition-colors"><Bookmark size={16} />{$t('roulette.add_planned')}</button>
            <button onclick={() => result && addToLibrary(result, 'watching')} class="flex items-center gap-1.5 px-3 py-2 rounded-lg bg-ink-light border border-border text-sm text-zinc-300 hover:border-gold/50 hover:text-gold transition-colors"><Tv size={16} />{$t('roulette.add_watching')}</button>
            {#if result.media_type !== 'anime' || result.mal_id}
              <button onclick={() => result && openCard(result)} class="flex items-center gap-1.5 px-3 py-2 rounded-lg text-sm font-medium transition-all hover:brightness-110 text-ink" style="background: linear-gradient(180deg, {accent}, color-mix(in srgb, {accent} 70%, #000));"><ExternalLink size={16} />{$t('roulette.open_card')}</button>
            {/if}
          </div>
        </div>
      </div>
    </div>
  {/if}

  {#if history.length > 0}
    <div class="mt-8">
      <h3 class="text-sm font-medium text-zinc-500 mb-2">{$t('roulette.history_title')}</h3>
      <div class="flex gap-2 overflow-x-auto pb-2">
        {#each history as h (h.ts)}
          <button onclick={() => { result = h; triggerConfetti(); }} class="shrink-0 w-20 group" title={h.title}>
            <div class="aspect-[2/3] rounded-lg overflow-hidden bg-ink-light border border-border group-hover:border-gold/50 transition-colors relative">
              {#if h.poster_url}<img src={h.poster_url} alt={h.title} class="absolute inset-0 w-full h-full object-cover" referrerpolicy="no-referrer" />{:else}<div class="absolute inset-0 flex items-center justify-center text-zinc-600"><Dices size={24} /></div>{/if}
            </div>
            <p class="text-[10px] text-zinc-500 truncate mt-1">{h.title}</p>
          </button>
        {/each}
      </div>
    </div>
  {/if}
</div>

<style>
  @keyframes card-bounce { 0% { transform: scale(0.92); opacity: 0; } 60% { transform: scale(1.02); opacity: 1; } 100% { transform: scale(1); opacity: 1; } }
</style>