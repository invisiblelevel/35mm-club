<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { page } from '$app/state';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { ArrowLeft, Star, Clock, Film, Check, Plus, Minus, FolderPlus } from 'lucide-svelte';

  type AnimeDetails = {
    mal_id: number;
    title: string;
    title_original: string | null;
    description: string | null;
    cover_url: string | null;
    episodes: number | null;
    duration: number | null;
    average_score: number | null;
    format: string | null;
    status: string | null;
    genres: string[];
    studio: string | null;
    year: number | null;
  };

  type CollectionOption = {
    id: number;
    name: string;
    in_collection: boolean;
  };

  let details: AnimeDetails | null = $state(null);
  let loading = $state(true);
  let error = $state('');
  let watchedEpisodes = $state(0);
  let saving = $state(false);

  let userTitleId = $state<number | null>(null);
  let userRating = $state<number | null>(null);
  let hoverRating = $state(0);

  let collections: CollectionOption[] = $state([]);
  let showCollectionPicker = $state(false);

  const malId = $derived(Number(page.params.mal_id));

  async function load() {
    loading = true;
    try {
      const [d, prog] = await Promise.all([
        invoke<AnimeDetails>('get_anime_details', { malId, locale: $locale }),
        invoke<number>('get_anime_progress', { malId }).catch(() => 0),
      ]);
      details = d;
      watchedEpisodes = prog;
      await loadUserData();
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function loadUserData() {
    try {
      const data = await invoke<{ user_title_id: number | null, status: string | null, rating: number | null }>('get_user_title_data', {
        tmdbId: malId,
        typeField: 'anime'
      });
      userTitleId = data.user_title_id;
      userRating = data.rating;
      await loadCollections();
    } catch (e) {
      console.error(e);
    }
  }

  async function loadCollections() {
    if (!userTitleId) {
      collections = [];
      return;
    }
    try {
      const all = await invoke<any[]>('get_collections');
      const userColls = await invoke<number[]>('get_user_collections_for_title', { userTitleId });
      collections = all.map(c => ({
        id: c.id,
        name: c.name,
        in_collection: userColls.includes(c.id)
      }));
    } catch (e) {
      console.error(e);
    }
  }

  async function toggleCollection(collId: number, currentlyIn: boolean) {
    if (!userTitleId) return;
    try {
      if (currentlyIn) {
        await invoke('remove_from_collection', { collectionId: collId, userTitleId });
      } else {
        await invoke('add_to_collection', { collectionId: collId, userTitleId });
      }
      await loadCollections();
    } catch (e) {
      console.error(e);
    }
  }

  async function setRating(r: number) {
    if (!userTitleId) return;
    try {
      const unlocked = await invoke<any[]>('update_rating', { userTitleId, rating: r });
      userRating = r;
      if (unlocked && unlocked.length > 0) {
        for (const ach of unlocked) {
          window.dispatchEvent(new CustomEvent('achievement-toast', { detail: ach }));
        }
      }
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
    } catch (e) {
      console.error(e);
    }
  }

  async function saveProgress(newCount: number) {
    if (!details || saving) return;
    if (newCount < 0) newCount = 0;
    if (details.episodes && newCount > details.episodes) newCount = details.episodes;

    saving = true;
    try {
      const unlocked = await invoke<any[]>('update_anime_progress', {
        malId,
        watched: newCount,
      });
      watchedEpisodes = newCount;
      if (unlocked && unlocked.length > 0) {
        for (const ach of unlocked) {
          window.dispatchEvent(new CustomEvent('achievement-toast', { detail: ach }));
        }
      }
      if (!userTitleId) {
        await loadUserData();
      }
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
    } catch (e) {
      console.error(e);
    } finally {
      saving = false;
    }
  }

  function incrementEpisode() {
    if (details?.episodes && watchedEpisodes < details.episodes) {
      saveProgress(watchedEpisodes + 1);
    }
  }

  function decrementEpisode() {
    if (watchedEpisodes > 0) {
      saveProgress(watchedEpisodes - 1);
    }
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
</script>

<div class="max-w-5xl mx-auto">
  <button onclick={() => history.back()} class="flex items-center gap-2 text-sm text-zinc-400 hover:text-gold transition-colors mb-4 md:mb-6">
    <ArrowLeft size={16} /> {$t('common.back')}
  </button>

  {#if loading}
    <div class="text-center text-zinc-500 py-20">{$t('title_page.loading')}</div>
  {:else if error}
    <div class="p-4 rounded-lg bg-red-950/50 border border-red-800 text-sm text-red-200"><pre class="whitespace-pre-wrap">{error}</pre></div>
  {:else if details}
    <div class="relative rounded-2xl overflow-hidden mb-6 md:mb-8 bg-surface border border-border">
      <div class="relative flex flex-col md:flex-row gap-4 md:gap-8 p-4 md:p-8">
        {#if details.cover_url}
          <img src={details.cover_url} alt={details.title} class="w-full md:w-56 h-48 md:h-84 object-cover rounded-xl shadow-2xl shrink-0" />
        {:else}
          <div class="w-full md:w-56 h-48 md:h-84 bg-ink-light rounded-xl flex items-center justify-center text-zinc-600 shrink-0"><Film size={48} /></div>
        {/if}

        <div class="flex-1 min-w-0 py-0 md:py-2">
          <h1 class="font-display text-2xl md:text-4xl font-bold mb-1 md:mb-2">{details.title}</h1>
          {#if details.title_original && details.title_original !== details.title}
            <p class="text-sm md:text-lg text-zinc-400 mb-3 md:mb-4">{details.title_original}</p>
          {/if}

          <div class="flex items-center gap-3 md:gap-4 text-sm text-zinc-400 mb-3 md:mb-4 flex-wrap">
            {#if details.year}<span>{details.year}</span>{/if}
            {#if details.episodes}<span>{details.episodes} {$t('title_page.episodes')}</span>{/if}
            {#if details.duration}<span class="flex items-center gap-1"><Clock size={14} /> {details.duration} {$t('title_page.minutes')}</span>{/if}
            {#if details.average_score}
              <span class="flex items-center gap-1 text-gold"><Star size={14} fill="currentColor" />{details.average_score.toFixed(1)}</span>
            {/if}
          </div>

          <!-- РЕЙТИНГ -->
          <div class="bg-ink-light border border-border rounded-xl p-3 md:p-4 mb-3 md:mb-4 max-w-md">
            <p class="text-xs text-zinc-500 uppercase tracking-wider mb-2">{$t('title_page.your_rating')}</p>
            {#if userTitleId}
              <div class="flex items-center gap-1 flex-wrap">
                {#each Array(10) as _, i}
                  {@const starValue = i + 1}
                  <div class="relative flex">
                    <button
                      onclick={() => setRating(starValue - 0.5)}
                      onmouseenter={() => hoverRating = starValue - 0.5}
                      onmouseleave={() => hoverRating = 0}
                      class="absolute left-0 top-0 w-1/2 h-full z-10"
                      aria-label="Оценка {starValue - 0.5}"
                    ></button>
                    <button
                      onclick={() => setRating(starValue)}
                      onmouseenter={() => hoverRating = starValue}
                      onmouseleave={() => hoverRating = 0}
                      class="absolute right-0 top-0 w-1/2 h-full z-10"
                      aria-label="Оценка {starValue}"
                    ></button>

                    <div class="relative pointer-events-none">
                      <Star size={16} fill="none" style="color: #52525b" class="md:hidden" />
                      <Star size={20} fill="none" style="color: #52525b" class="hidden md:block" />
                      <div
                        class="absolute inset-0 overflow-hidden pointer-events-none"
                        style="width: {Math.min(1, Math.max(0, (hoverRating || userRating || 0) - (starValue - 1))) * 100}%"
                      >
                        <Star size={16} fill="currentColor" style="color: #ff9000" class="md:hidden" />
                        <Star size={20} fill="currentColor" style="color: #ff9000" class="hidden md:block" />
                      </div>
                    </div>
                  </div>
                {/each}
                <span class="ml-2 text-sm text-zinc-400 font-medium">{userRating ? userRating.toFixed(1) : '—'}</span>
              </div>
            {:else}
              <p class="text-sm text-zinc-500">Добавь аниме в библиотеку, чтобы поставить оценку.</p>
            {/if}
          </div>

          <!-- КОЛЛЕКЦИИ -->
          {#if userTitleId}
            <div class="relative mb-3 md:mb-4">
              <button
                onclick={() => showCollectionPicker = !showCollectionPicker}
                class="flex items-center justify-center gap-2 px-4 py-2.5 rounded-lg bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold active:scale-[0.98] transition-all text-sm w-full md:w-auto"
              >
                <FolderPlus size={16} />
                {$t('title_page.add_to_collection')}
              </button>
              {#if showCollectionPicker}
                <div class="absolute top-full mt-2 left-0 right-0 md:right-auto md:w-64 bg-surface border border-border rounded-xl shadow-2xl z-20 p-2 max-h-64 overflow-y-auto">
                  {#if collections.length === 0}
                    <p class="text-xs text-zinc-500 p-2">{$t('collections.empty_hint')}</p>
                  {:else}
                    {#each collections as coll}
                      <button
                        onclick={() => toggleCollection(coll.id, coll.in_collection)}
                        class="w-full flex items-center gap-2 px-3 py-2 rounded-lg hover:bg-ink-light text-sm text-left transition-colors active:scale-[0.99]"
                      >
                        <span class="text-lg">{coll.in_collection ? '✓' : '○'}</span>
                        <span class={coll.in_collection ? 'text-gold' : 'text-zinc-300'}>{coll.name}</span>
                      </button>
                    {/each}
                  {/if}
                </div>
              {/if}
            </div>
          {/if}

          {#if details.studio}
            <div class="mb-3 md:mb-4">
              <span class="text-xs uppercase tracking-wider text-zinc-500">{$t('title_page.director').replace('Режиссёр', 'Студия')}</span>
              <p class="text-zinc-200 font-medium">{details.studio}</p>
            </div>
          {/if}

          {#if details.genres.length > 0}
            <div class="flex flex-wrap gap-2 mb-3 md:mb-4">
              {#each details.genres as genre}
                <span class="px-3 py-1 rounded-full bg-gold/10 border border-gold/30 text-xs text-gold">{genre}</span>
              {/each}
            </div>
          {/if}

          {#if details.description}
            <p class="text-zinc-400 leading-relaxed max-w-2xl line-clamp-4 text-sm">
              {@html details.description.replace(/<[^>]*>/g, '')}
            </p>
          {/if}
        </div>
      </div>
    </div>

    <!-- ПРОГРЕСС -->
    {#if details.episodes}
      <div class="bg-surface border border-border rounded-xl p-4 md:p-6 mb-6 md:mb-8">
        <h2 class="font-display text-lg md:text-xl font-bold mb-3 md:mb-4">{$t('title_page.progress')}</h2>
        <div class="flex flex-col sm:flex-row items-center gap-3 md:gap-4">
          <div class="flex items-center gap-3 md:gap-4">
            <button
              onclick={decrementEpisode}
              disabled={watchedEpisodes === 0 || saving}
              class="w-11 h-11 md:w-10 md:h-10 rounded-lg bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold active:scale-[0.95] transition-all disabled:opacity-30"
            >
              <Minus size={18} class="mx-auto" />
            </button>
            <div class="text-center">
              <div class="text-2xl md:text-3xl font-bold">
                <span class="text-gold">{watchedEpisodes}</span>
                <span class="text-zinc-500"> / {details.episodes}</span>
              </div>
              <div class="text-xs text-zinc-500 mt-1">{$t('title_page.watched')}</div>
            </div>
            <button
              onclick={incrementEpisode}
              disabled={watchedEpisodes >= details.episodes || saving}
              class="w-11 h-11 md:w-10 md:h-10 rounded-lg bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold active:scale-[0.95] transition-all disabled:opacity-30"
            >
              <Plus size={18} class="mx-auto" />
            </button>
          </div>

          {#if watchedEpisodes === details.episodes}
            <div class="flex items-center gap-2 text-emerald-400 sm:ml-4">
              <Check size={20} />
              <span class="text-sm font-medium">{$t('title_page.fully_watched')}</span>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  {/if}
</div>