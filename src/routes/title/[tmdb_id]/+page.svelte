<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { page } from '$app/state';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { ArrowLeft, Star, Clock, ExternalLink, Film, ChevronDown, ChevronRight, Check, FolderPlus } from 'lucide-svelte';
  import { settings } from '$lib/settings.svelte';
  import { goto } from '$app/navigation';

  type CastMember = { name: string; character: string | null; profile_url: string | null; };

  type TitleDetails = {
    tmdb_id: number; title: string; overview: string | null; poster_url: string | null;
    backdrop_url: string | null; year: number | null; rating: number | null; runtime: number | null;
    genres: string[]; director: string | null; cast: CastMember[]; trailer_key: string | null; media_type: string;
  };

  type UserTitleData = {
    user_title_id: number | null; status: string | null; rating: number | null; added_at: string | null;
  };

  type Achievement = { id: number; code: string; title_key: string | null; description_key: string | null; xp_reward: number; };

  type TvSeason = {
    season_number: number;
    name: string | null;
    episode_count: number;
    poster_url: string | null;
    watched_count: number;
  };

  type Episode = {
    id: number; episode_number: number; name: string | null; overview: string | null;
    runtime: number | null; air_date: string | null; still_url: string | null; watched: boolean;
  };

  type Collection = {
    id: number; name: string; description: string | null; created_at: string; item_count: number; preview_posters: string[];
  };

  let details: TitleDetails | null = $state(null);
  let userData: UserTitleData | null = $state(null);
  let loading = $state(true);
  let error = $state('');
  let hoveredStar = $state(0);
  let savingRating = $state(false);

  let seasons: TvSeason[] = $state([]);
  let openSeason: number | null = $state(null);
  let seasonEpisodes: Record<number, Episode[]> = $state({});
  let loadingEpisodes = $state<number | null>(null);

  let showCollections = $state(false);
  let allCollections: Collection[] = $state([]);
  let titleCollections: number[] = $state([]);

  const tmdbId = $derived(Number(page.params.tmdb_id));
  const mediaType = $derived((page.url.searchParams.get('type') ?? 'movie') as string);
  const isTv = $derived(mediaType === 'tv');

  const currentRating = $derived(userData?.rating ?? 0);
  const displayRating = $derived(hoveredStar > 0 ? hoveredStar : currentRating);

  async function load() {
    try {
      const [d, u] = await Promise.all([
        invoke<TitleDetails>('ensure_title_language', {
          tmdbId,
          typeField: mediaType,
          apiKey: settings.tmdbKey || null,
          locale: $locale,
        }),
        invoke<UserTitleData>('get_user_title_data', {
          tmdbId,
          typeField: mediaType,
        }).catch(() => ({ user_title_id: null, status: null, rating: null, added_at: null } as UserTitleData)),
      ]);
      details = d;
      userData = u;

      if (isTv && u.user_title_id) {
        await loadSeasons();
      }

      if (u.user_title_id) {
        await loadCollectionsForTitle(u.user_title_id);
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function loadSeasons() {
    try {
      seasons = await invoke<TvSeason[]>('get_tv_seasons', { tmdbId, apiKey: settings.tmdbKey || null, locale: $locale });
    } catch (e) { console.error('Failed to load seasons:', e); }
  }

  async function loadCollectionsForTitle(utid: number) {
    try {
      const [all, mine] = await Promise.all([
        invoke<Collection[]>('get_collections'),
        invoke<number[]>('get_user_collections_for_title', { userTitleId: utid }),
      ]);
      allCollections = all;
      titleCollections = mine;
    } catch (e) { console.error(e); }
  }

  async function toggleCollection(colId: number) {
    if (!userData?.user_title_id) return;
    try {
      if (titleCollections.includes(colId)) {
        await invoke('remove_from_collection', { collectionId: colId, userTitleId: userData.user_title_id });
        titleCollections = titleCollections.filter(id => id !== colId);
      } else {
        await invoke('add_to_collection', { collectionId: colId, userTitleId: userData.user_title_id });
        titleCollections = [...titleCollections, colId];
      }
      allCollections = await invoke<Collection[]>('get_collections');
    } catch (e) { console.error(e); }
  }

  async function toggleSeason(seasonNumber: number) {
    if (openSeason === seasonNumber) { openSeason = null; return; }
    openSeason = seasonNumber;

    if (!seasonEpisodes[seasonNumber]) {
      loadingEpisodes = seasonNumber;
      try {
        const eps = await invoke<Episode[]>('get_season_episodes', { tmdbId, seasonNumber, apiKey: settings.tmdbKey || null, locale: $locale });
        seasonEpisodes = { ...seasonEpisodes, [seasonNumber]: eps };
      } catch (e) {
        console.error('Failed to load episodes:', e);
      } finally {
        loadingEpisodes = null;
      }
    }
  }

  async function toggleEpisode(seasonNumber: number, episode: Episode) {
    const newWatched = !episode.watched;
    try {
      await invoke('mark_episode_watched', { episodeId: episode.id, watched: newWatched });
      const eps = seasonEpisodes[seasonNumber].map(e => e.id === episode.id ? { ...e, watched: newWatched } : e);
      seasonEpisodes = { ...seasonEpisodes, [seasonNumber]: eps };
      userData = await invoke<UserTitleData>('get_user_title_data', { tmdbId, typeField: mediaType });
      await loadSeasons();
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
    } catch (e) {
      console.error('Failed to mark episode:', e);
    }
  }

  function watchedCount(seasonNumber: number): number {
    return (seasonEpisodes[seasonNumber] ?? []).filter(e => e.watched).length;
  }

  onMount(load);

  let lastLocale = $state('');
  $effect(() => {
    const l = $locale;
    if (lastLocale === '') {
      lastLocale = l;
      return;
    }
    if (lastLocale !== l) {
      lastLocale = l;
      load();
    }
  });

  async function setRating(value: number) {
    if (!userData?.user_title_id || savingRating) return;
    savingRating = true;
    try {
      const unlocked = await invoke<Achievement[]>('update_rating', { userTitleId: userData.user_title_id, rating: value });
      userData = { ...userData, rating: value };
      if (unlocked && unlocked.length > 0) {
        for (const ach of unlocked) window.dispatchEvent(new CustomEvent('achievement-toast', { detail: ach }));
      }
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
    } catch (e) {
      error = String(e);
    } finally {
      savingRating = false;
    }
  }
</script>

<div class="max-w-6xl mx-auto">
  <button onclick={() => history.back()} class="flex items-center gap-2 text-sm text-zinc-400 hover:text-gold transition-colors mb-4 md:mb-6">
    <ArrowLeft size={16} /> {$t('common.back')}
  </button>

  {#if loading}
    <div class="text-center text-zinc-500 py-20">{$t('title_page.loading')}</div>
  {:else if error}
    <div class="p-4 rounded-lg bg-red-950/50 border border-red-800 text-sm text-red-200"><pre class="whitespace-pre-wrap">{error}</pre></div>
  {:else if details}
    <div class="relative rounded-2xl overflow-hidden mb-6 md:mb-8">
      {#if details.backdrop_url}
        <div class="absolute inset-0 hidden md:block">
          <img src={details.backdrop_url} alt="" class="w-full h-full object-cover" />
          <div class="absolute inset-0 bg-gradient-to-t from-ink via-ink/80 to-ink/40"></div>
        </div>
      {/if}

      <div class="relative flex flex-col md:flex-row gap-4 md:gap-8 p-4 md:p-8">
        {#if details.poster_url}
          <img src={details.poster_url} alt={details.title} class="w-full md:w-56 h-48 md:h-84 object-cover rounded-xl shadow-2xl shrink-0" />
        {:else}
          <div class="w-full md:w-56 h-48 md:h-84 bg-ink-light rounded-xl flex items-center justify-center text-zinc-600 shrink-0"><Film size={48} /></div>
        {/if}

        <div class="flex-1 min-w-0 py-0 md:py-4">
          <h1 class="font-display text-2xl md:text-4xl font-bold mb-2 md:mb-3">{details.title}</h1>

          <div class="flex items-center gap-3 md:gap-4 text-sm text-zinc-400 mb-3 md:mb-4 flex-wrap">
            {#if details.year}<span>{details.year}</span>{/if}
            {#if details.runtime}<span class="flex items-center gap-1"><Clock size={14} /> {details.runtime} {$t('title_page.minutes')}</span>{/if}
            {#if details.rating}<span class="flex items-center gap-1 text-gold"><Star size={14} fill="currentColor" /> {details.rating.toFixed(1)}</span>{/if}
          </div>

          <div class="mb-4 md:mb-6 p-3 md:p-4 rounded-xl bg-surface/80 border border-border">
            <p class="text-xs uppercase tracking-wider text-zinc-500 mb-2">{$t('title_page.your_rating')}</p>
            {#if !userData?.user_title_id}
              <p class="text-sm text-zinc-500">{$t('title_page.add_to_rate')}</p>
            {:else}
              <div class="flex items-center gap-3 flex-wrap">
                <div class="flex items-center gap-0.5">
                  {#each Array(10) as _, i}
                    {@const starFill = (() => {
                      const full = Math.floor(displayRating);
                      const frac = displayRating - full;
                      if (i < full) return 100;
                      if (i === full) return frac >= 0.5 ? 50 : 0;
                      return 0;
                    })()}
                    <div class="relative" onmouseleave={() => hoveredStar = 0}>
                      <Star size={16} class="text-zinc-600 md:hidden" />
                      <Star size={18} class="text-zinc-600 hidden md:block" />
                      {#if starFill > 0}
                        <div class="absolute inset-0 overflow-hidden pointer-events-none" style="width: {starFill}%">
                          <Star size={16} fill="currentColor" class="text-gold md:hidden" />
                          <Star size={18} fill="currentColor" class="text-gold hidden md:block" />
                        </div>
                      {/if}
                      <button type="button" disabled={savingRating} onclick={() => setRating(i + 0.5)} onmouseenter={() => hoveredStar = i + 0.5} class="absolute inset-y-0 left-0 w-1/2 disabled:opacity-50" aria-label={`Оценка ${i + 0.5}`}></button>
                      <button type="button" disabled={savingRating} onclick={() => setRating(i + 1.0)} onmouseenter={() => hoveredStar = i + 1.0} class="absolute inset-y-0 right-0 w-1/2 disabled:opacity-50" aria-label={`Оценка ${i + 1.0}`}></button>
                    </div>
                  {/each}
                </div>
                {#if userData.rating != null}
                  <span class="text-sm text-gold font-medium">{userData.rating.toFixed(1)}/10</span>
                {:else}
                  <span class="text-sm text-zinc-500">{$t('title_page.not_rated')}</span>
                {/if}
                {#if savingRating}<span class="text-xs text-zinc-500">{$t('common.loading')}</span>{/if}
              </div>
            {/if}
          </div>

          {#if userData?.user_title_id}
            <div class="mb-4 md:mb-6">
              <button
                type="button"
                onclick={() => showCollections = true}
                class="flex items-center justify-center gap-2 w-full md:w-auto px-4 py-2.5 rounded-lg bg-ink-light border border-border text-sm text-zinc-300 hover:border-gold hover:text-gold active:scale-[0.98] transition-all"
              >
                <FolderPlus size={16} />
                {$t('title_page.add_to_collection')}
              </button>
            </div>
          {/if}

          {#if details.genres.length > 0}
            <div class="flex flex-wrap gap-2 mb-3 md:mb-6">
              {#each details.genres as genre}
                <span class="px-3 py-1 rounded-full bg-gold/10 border border-gold/30 text-xs text-gold">{genre}</span>
              {/each}
            </div>
          {/if}

          {#if details.overview}
            <p class="text-zinc-300 leading-relaxed mb-4 md:mb-6 max-w-2xl text-sm md:text-base">{details.overview}</p>
          {/if}

          {#if details.trailer_key}
            <a href={`https://www.youtube.com/watch?v=${details.trailer_key}`} target="_blank" rel="noopener noreferrer" class="inline-flex items-center justify-center gap-2 w-full md:w-auto px-5 py-2.5 bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.98] text-ink font-medium rounded-lg transition-all">
              <ExternalLink size={16} /> {$t('title_page.trailer')}
            </a>
          {/if}
        </div>
      </div>
    </div>

    {#if isTv && userData?.user_title_id && seasons.length > 0}
      <div class="mb-6 md:mb-8">
        <h2 class="font-display text-lg md:text-2xl font-bold mb-3 md:mb-4">{$t('title_page.seasons_and_episodes')}</h2>
        <div class="space-y-2">
          {#each seasons as season}
            {@const isOpen = openSeason === season.season_number}
            {@const isLoaded = !!seasonEpisodes[season.season_number]}
            {@const watched = isLoaded ? watchedCount(season.season_number) : season.watched_count}

            <div class="bg-surface border border-border rounded-xl overflow-hidden">
              <button type="button" onclick={() => toggleSeason(season.season_number)} class="w-full flex items-center gap-3 md:gap-4 p-3 md:p-4 hover:bg-ink-light/50 transition-colors text-left">
                <div class="shrink-0 text-zinc-400">
                  {#if isOpen}<ChevronDown size={18} />{:else}<ChevronRight size={18} />{/if}
                </div>
                <div class="flex-1 min-w-0">
                  <p class="font-semibold text-zinc-100 text-sm md:text-base truncate">
                    {season.name ?? `${$t('title_page.season')} ${season.season_number}`}
                  </p>
                  <p class="text-xs text-zinc-500">
                    {season.episode_count} {$t('title_page.episodes')}
                    {#if watched > 0} · {$t('title_page.watched')}: {watched}/{season.episode_count}{/if}
                  </p>
                </div>
                {#if watched === season.episode_count && season.episode_count > 0}
                  <div class="shrink-0 w-6 h-6 rounded-full bg-emerald-950/50 border border-emerald-700 flex items-center justify-center"><Check size={14} class="text-emerald-400" /></div>
                {/if}
              </button>

              {#if isOpen}
                <div class="border-t border-border">
                  {#if loadingEpisodes === season.season_number}
                    <div class="p-4 text-center text-zinc-500 text-sm">{$t('common.loading')}</div>
                  {:else if seasonEpisodes[season.season_number]?.length > 0}
                    <div class="divide-y divide-border">
                      {#each seasonEpisodes[season.season_number] as ep}
                        <button type="button" onclick={() => toggleEpisode(season.season_number, ep)} class="w-full flex items-center gap-3 md:gap-4 p-3 hover:bg-ink-light/50 transition-colors text-left">
                          <div class="shrink-0 w-6 h-6 rounded-md border-2 flex items-center justify-center transition-colors {ep.watched ? 'bg-gold border-gold' : 'border-zinc-600'}">
                            {#if ep.watched}<Check size={14} class="text-ink" />{/if}
                          </div>
                          <div class="shrink-0 text-xs text-zinc-500 font-mono w-8 md:w-10">{ep.episode_number}.</div>
                          <div class="flex-1 min-w-0">
                            <p class="text-sm text-zinc-200 truncate {ep.watched ? 'opacity-60' : ''}">
                              {ep.name ?? `${$t('title_page.episode')} ${ep.episode_number}`}
                            </p>
                            {#if ep.runtime}<p class="text-xs text-zinc-500">{ep.runtime} {$t('title_page.minutes')}</p>{/if}
                          </div>
                        </button>
                      {/each}
                    </div>
                  {:else}
                    <div class="p-4 text-center text-zinc-500 text-sm">{$t('title_page.episodes_not_found')}</div>
                  {/if}
                </div>
              {/if}
            </div>
          {/each}
        </div>
      </div>
    {/if}

    {#if details.director}
      <div class="mb-4 md:mb-6">
        <span class="text-xs uppercase tracking-wider text-zinc-500">{$t('title_page.director')}</span>
        <p class="text-zinc-200 font-medium">{details.director}</p>
      </div>
    {/if}

    {#if details.cast.length > 0}
      <div>
        <h2 class="font-display text-lg md:text-2xl font-bold mb-3 md:mb-4">{$t('title_page.cast')}</h2>
        <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-4 lg:grid-cols-5 gap-2 md:gap-4">
          {#each details.cast as member}
            <div class="rounded-xl bg-surface border border-border overflow-hidden">
              {#if member.profile_url}
                <div class="aspect-[2/3] overflow-hidden bg-ink-light"><img src={member.profile_url} alt={member.name} class="w-full h-full object-cover" /></div>
              {:else}
                <div class="aspect-[2/3] bg-ink-light flex items-center justify-center text-zinc-600"><Film size={24} /></div>
              {/if}
              <div class="p-2 md:p-3">
                <p class="text-xs md:text-sm font-medium truncate">{member.name}</p>
                {#if member.character}<p class="text-[10px] md:text-xs text-zinc-500 truncate">{member.character}</p>{/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    {/if}
  {/if}
</div>

{#if showCollections}
  <div class="fixed inset-0 z-50 flex items-end md:items-center justify-center p-0 md:p-5 bg-black/70 backdrop-blur-sm" onclick={(e) => e.target === e.currentTarget && (showCollections = false)} role="presentation">
    <div class="w-full max-w-md bg-surface border border-border rounded-t-2xl md:rounded-xl p-4 md:p-6 pb-[max(1rem,env(safe-area-inset-bottom))] md:pb-6 max-h-[85vh] overflow-y-auto" onclick={(e) => e.stopPropagation()}>
      <h2 class="font-display text-lg md:text-xl font-bold mb-4">{$t('collections.title')}</h2>
      {#if allCollections.length === 0}
        <div class="text-center py-6">
          <p class="text-zinc-400 text-sm mb-3">{$t('collections.empty')}</p>
          <p class="text-zinc-500 text-xs mb-4">{$t('collections.empty_hint')}</p>
          <button
            onclick={() => { showCollections = false; goto('/collections'); }}
            class="px-4 py-2.5 bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.98] text-ink font-medium rounded-lg text-sm transition-all"
          >{$t('collections.go_to')}</button>
        </div>
      {:else}
        <div class="space-y-2 mb-4">
          {#each allCollections as c}
            <label class="flex items-center gap-3 p-3 rounded-lg bg-ink-light border border-border hover:border-gold/50 cursor-pointer active:scale-[0.99] transition-all">
              <input
                type="checkbox"
                checked={titleCollections.includes(c.id)}
                onchange={() => toggleCollection(c.id)}
                class="w-4 h-4 accent-gold"
              />
              <span class="text-sm text-zinc-200 flex-1 truncate">{c.name}</span>
              <span class="text-xs text-zinc-500 shrink-0">{c.item_count}</span>
            </label>
          {/each}
        </div>
      {/if}
      <button onclick={() => showCollections = false} class="w-full px-4 py-2.5 bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.98] text-ink font-medium rounded-lg mt-2 transition-all">
        {$t('common.close')}
      </button>
    </div>
  </div>
{/if}