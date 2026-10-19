<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { X, Star, Check } from 'lucide-svelte';
  import { invoke } from '@tauri-apps/api/core';

  type AnimeSearchItem = {
    mal_id: number;
    title: string;
    title_original: string | null;
    description: string | null;
    cover_url: string | null;
    episodes: number | null;
    average_score: number | null;
    format: string | null;
    year: number | null;
  };

  interface Props {
    item: AnimeSearchItem;
    onClose: () => void;
  }

  let { item, onClose }: Props = $props();

  let saving = $state(false);
  let saved = $state(false);
  let error = $state('');
  let description = $state<string | null>(item.description);
  let studio = $state<string | null>(null);
  let year = $state<number | null>(item.year);

  // Если описание из поиска не пришло — дёргаем details при монтировании.
  $effect(() => {
    if (description) return;
    let cancelled = false;
    (async () => {
      try {
        const details = await invoke<any>('get_anime_details', { malId: item.mal_id, locale: $locale });
        if (cancelled) return;
        if (details.description && !description) description = details.description;
        if (details.studio) studio = details.studio;
        if (details.year != null) year = details.year;
      } catch (e) {
        console.error('Failed to fetch anime details:', e);
      }
    })();
    return () => { cancelled = true; };
  });

  const statuses = [
    { value: 'watching', key: 'status.watching' },
    { value: 'completed', key: 'status.completed' },
    { value: 'planned', key: 'status.planned' },
    { value: 'on_hold', key: 'status.on_hold' },
    { value: 'dropped', key: 'status.dropped' },
  ];

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) onClose();
  }

  // Убираем HTML-теги из Shikimori-описания ([character=...]...[/character], <br> и т.д.)
  function cleanDescription(html: string | null): string | null {
    if (!html) return null;
    return html
      .replace(/<br\s*\/?>/gi, ' ')
      .replace(/<\/?[^>]+(>|$)/g, '')
      .replace(/\[[^\]]+\]/g, '')
      .replace(/\s+/g, ' ')
      .trim();
  }

  async function saveToLibrary(status: string) {
    saving = true;
    error = '';
    try {
      const newAchievements = await invoke<any[]>('add_anime_to_library', {
        payload: {
          mal_id: item.mal_id,
          title: item.title,
          description: description,
          cover_url: item.cover_url,
          episodes: item.episodes,
          format: item.format,
          status,
          studio,
          year,
        },
      });

      if (newAchievements && newAchievements.length > 0) {
        for (const ach of newAchievements) {
          window.dispatchEvent(new CustomEvent('achievement-toast', { detail: ach }));
        }
      }

      saved = true;
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
      setTimeout(() => onClose(), 800);
    } catch (e) {
      error = String(e);
    } finally {
      saving = false;
    }
  }
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && onClose()} />

<div class="fixed inset-0 z-50 flex items-center justify-center p-5 bg-black/70 backdrop-blur-sm" onclick={handleBackdrop} role="presentation">
  <div class="relative w-full max-w-3xl rounded-xl bg-surface border border-border shadow-2xl overflow-hidden" role="dialog" aria-modal="true" onclick={(e) => e.stopPropagation()}>
    <button onclick={onClose} class="absolute top-3 right-3 z-10 w-8 h-8 rounded-md flex items-center justify-center text-zinc-400 hover:bg-ink-light hover:text-zinc-100 transition-colors" aria-label={$t('common.close')}>
      <X size={16} />
    </button>

    <div class="flex gap-6 p-6">
      {#if item.cover_url}
        <img src={item.cover_url} alt={item.title} class="w-48 h-72 object-cover rounded-lg shrink-0" />
      {:else}
        <div class="w-48 h-72 bg-ink-light rounded-lg shrink-0 flex items-center justify-center text-zinc-600 text-xs">{$t('search.no_poster')}</div>
      {/if}

      <div class="flex-1 min-w-0">
        <h2 class="font-display text-2xl font-bold mb-2">{item.title}</h2>
        {#if item.title_original && item.title_original !== item.title}
          <p class="text-sm text-zinc-500 mb-2">{item.title_original}</p>
        {/if}

        <div class="flex items-center gap-4 text-sm text-zinc-400 mb-4">
          {#if year}<span>{year}</span>{/if}
          {#if item.episodes}<span>{item.episodes} {$t('title_page.episodes')}</span>{/if}
          {#if item.average_score}
            <span class="flex items-center gap-1 text-gold"><Star size={14} fill="currentColor" />{item.average_score.toFixed(1)}</span>
          {/if}
        </div>

        {#if cleanDescription(description)}
          <p class="text-sm text-zinc-400 leading-relaxed mb-5 line-clamp-6">
            {cleanDescription(description)}
          </p>
        {/if}

        {#if saved}
          <div class="flex items-center gap-2 px-4 py-3 bg-emerald-950/50 border border-emerald-800 rounded-lg text-emerald-200">
            <Check size={18} />
            <span class="font-medium">{$t('title_modal.added')}</span>
          </div>
        {:else}
          <div>
            <p class="text-sm text-zinc-400 mb-3">{$t('title_modal.choose_status')}</p>
            <div class="flex flex-wrap gap-2">
              {#each statuses as s}
                <button onclick={() => saveToLibrary(s.value)} disabled={saving} class="px-4 py-2 rounded-lg text-sm bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold transition-colors disabled:opacity-50">{$t(s.key)}</button>
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