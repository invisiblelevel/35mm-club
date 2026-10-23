<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { X, Star, Check, Play, Clock, Pause, Trash2 } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { settings } from '$lib/settings.svelte';
  import { goto } from '$app/navigation';

  export type MovieItem = {
    tmdb_id: number;
    title: string;
    overview: string | null;
    poster_url: string | null;
    year: number | null;
    rating: number | null;
    media_type: string;
  };

  type Achievement = {
    id: number;
    code: string;
    title_key: string | null;
    description_key: string | null;
    xp_reward: number;
  };

  interface Props {
    item: MovieItem;
    onClose: () => void;
  }

  let { item, onClose }: Props = $props();

  let saving = $state(false);
  let saved = $state(false);
  let error = $state('');

  const statuses = [
    { value: 'watching', key: 'status.watching', icon: Play },
    { value: 'completed', key: 'status.completed', icon: Check },
    { value: 'planned', key: 'status.planned', icon: Clock },
    { value: 'on_hold', key: 'status.on_hold', icon: Pause },
    { value: 'dropped', key: 'status.dropped', icon: Trash2 },
  ];

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) onClose();
  }

  function imgError(e: Event) {
    const img = e.currentTarget as HTMLImageElement;
    img.style.opacity = '0';
  }

  function displayPoster(): string | null {
    if (!item.poster_url) return null;
    if (item.poster_url.startsWith('data:')) return item.poster_url;
    return item.poster_url.replace('/w500/', '/w342/');
  }

  // ГЛАВНОЕ: получаем путь к постеру для сохранения в базу
  async function getPosterPathForDb(): Promise<string | null> {
    // Если poster_url — нормальная ссылка (не data:), берём из неё
    if (item.poster_url && !item.poster_url.startsWith('data:')) {
      if (item.poster_url.startsWith('https://image.tmdb.org/t/p/w500')) {
        return item.poster_url.replace('https://image.tmdb.org/t/p/w500', '');
      }
      return item.poster_url;
    }

    // Иначе идём в TMDB и спрашиваем заново
    try {
      const details = await invoke<any>('get_title_details', {
        tmdbId: item.tmdb_id,
        mediaType: item.media_type === 'tv' ? 'tv' : 'movie',
        apiKey: settings.tmdbKey || null,
        locale: $locale,
      });
      if (details.poster_url) {
        return details.poster_url.replace('https://image.tmdb.org/t/p/w500', '');
      }
    } catch (e) {
      console.error('Failed to fetch poster:', e);
    }
    return null;
  }

  async function saveToLibrary(status: string) {
    saving = true;
    error = '';
    try {
      // Сохраняем путь к постеру
      const posterPath = await getPosterPathForDb();

      let runtime: number | null = null;
      let cast_ids: number[] = [];
      let director_ids: number[] = [];
      let collection_id: number | null = null;
      let producer_ids: number[] = [];
      let production_company_ids: number[] = [];
      let keyword_ids: number[] = [];

      try {
        const details = await invoke<any>('get_title_details', {
          tmdbId: item.tmdb_id,
          mediaType: item.media_type === 'tv' ? 'tv' : 'movie',
          apiKey: settings.tmdbKey || null,
          locale: $locale,
        });
        runtime = details.runtime ?? null;
        cast_ids = details.cast_ids || [];
        director_ids = details.director_ids || [];
        collection_id = details.collection_id ?? null;
        producer_ids = details.producer_ids || [];
        production_company_ids = details.production_company_ids || [];
        keyword_ids = details.keyword_ids || [];
      } catch (e) {
        console.error('Failed to fetch details:', e);
      }

      const newAchievements = await invoke<any[]>('add_to_library', {
        payload: {
          tmdb_id: item.tmdb_id,
          type: item.media_type === 'tv' ? 'tv' : 'movie',
          title: item.title,
          original_title: null,
          year: item.year,
          poster_path: posterPath,
          overview: item.overview,
          imdb_rating: item.rating,
          runtime,
          status,
          cast_ids,
          director_ids,
          collection_id,
          producer_ids,
          production_company_ids,
          keyword_ids,
          locale: $locale,
        },
      });

      if (item.media_type === 'tv' && status === 'completed') {
        try {
          const unlockedFromEpisodes = await invoke<Achievement[]>('mark_all_episodes_watched', {
            tmdbId: item.tmdb_id,
            apiKey: settings.tmdbKey || null,
            locale: $locale,
          });
          if (unlockedFromEpisodes && unlockedFromEpisodes.length > 0) {
            for (const ach of unlockedFromEpisodes) {
              window.dispatchEvent(new CustomEvent('achievement-toast', { detail: ach }));
            }
          }
        } catch (e) {
          console.error('Failed to mark all episodes:', e);
        }
      }

      if (newAchievements && newAchievements.length > 0) {
        for (const ach of newAchievements) {
          window.dispatchEvent(new CustomEvent('achievement-toast', { detail: ach }));
        }
      }

      saved = true;
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));

      if (item.media_type === 'tv' && status === 'watching') {
        setTimeout(() => {
          goto(`/title/${item.tmdb_id}?type=tv`);
          onClose();
        }, 500);
      } else {
        setTimeout(() => onClose(), 800);
      }
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }

  const poster = $derived(displayPoster());
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && onClose()} />

<div class="fixed inset-0 z-50 flex items-end md:items-center justify-center p-0 md:p-5 bg-black/70 backdrop-blur-sm" onclick={handleBackdrop} role="presentation">
  <div class="relative w-full max-w-3xl max-h-[92vh] md:max-h-[90vh] overflow-y-auto rounded-t-2xl md:rounded-xl bg-surface border border-border shadow-2xl pb-[env(safe-area-inset-bottom)] md:pb-0" role="dialog" aria-modal="true" onclick={(e) => e.stopPropagation()}>
    <button onclick={onClose} class="absolute top-3 right-3 z-10 w-8 h-8 rounded-md flex items-center justify-center text-zinc-400 hover:bg-ink-light hover:text-zinc-100 transition-colors bg-surface/80 backdrop-blur" aria-label={$t('common.close')}>
      <X size={16} />
    </button>

    <div class="flex flex-col md:flex-row gap-4 md:gap-6 p-4 md:p-6">
      {#if poster}
        <img
          src={poster}
          alt={item.title}
          decoding="sync"
          referrerpolicy="no-referrer"
          onerror={imgError}
          class="w-full md:w-48 h-48 md:h-72 object-cover rounded-lg shrink-0"
        />
      {:else}
        <div class="w-full md:w-48 h-48 md:h-72 bg-ink-light rounded-lg shrink-0 flex items-center justify-center text-zinc-600 text-xs">{$t('search.no_poster')}</div>
      {/if}

      <div class="flex-1 min-w-0">
        <h2 class="font-display text-xl md:text-2xl font-bold mb-2 pr-8">{item.title}</h2>

        <div class="flex items-center gap-4 text-sm text-zinc-400 mb-4 flex-wrap">
          {#if item.year}<span>{item.year}</span>{/if}
          {#if item.rating}
            <span class="flex items-center gap-1 text-gold"><Star size={14} fill="currentColor" /> {item.rating.toFixed(1)}</span>
          {/if}
          <span class="uppercase text-xs">{item.media_type}</span>
        </div>

        {#if item.overview}
          <p class="text-sm text-zinc-400 leading-relaxed mb-6 line-clamp-6">{item.overview}</p>
        {/if}

        {#if saved}
          <div class="flex items-center gap-2 px-4 py-3 bg-emerald-950/50 border border-emerald-800 rounded-lg text-emerald-200">
            <Check size={18} />
            <span class="font-medium">{$t('title_modal.added')}</span>
          </div>
        {:else}
          <div>
            <p class="text-sm text-zinc-400 mb-3">{$t('title_modal.choose_status')}</p>
            <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
              {#each statuses as s}
                <button
                  onclick={() => saveToLibrary(s.value)}
                  disabled={saving}
                  class="flex items-center justify-center gap-2 px-4 py-2.5 rounded-full bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.96] text-ink font-medium text-sm transition-all disabled:opacity-50 shadow-lg shadow-gold/20"
                >
                  <svelte:component this={s.icon} size={16} />
                  <span>{$t(s.key)}</span>
                </button>
              {/each}
            </div>
            {#if saving}<p class="text-xs text-zinc-500 mt-3">{$t('common.loading')}</p>{/if}
          </div>
        {/if}

        {#if error}
          <div class="mt-4 p-3 rounded-lg bg-red-950/50 border border-red-800 text-xs text-red-200"><pre class="whitespace-pre-wrap">{error}</pre></div>
        {/if}
      </div>
    </div>
  </div>
</div>