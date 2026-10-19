<script lang="ts">
  import { t } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { Search, Star, Film } from 'lucide-svelte';
  import AnimeModal from '$lib/components/AnimeModal.svelte';

  type AnimeSearchItem = {
    anilist_id: number;
    title: string;
    title_romaji: string | null;
    title_native: string | null;
    description: string | null;
    cover_url: string | null;
    episodes: number | null;
    format: string | null;
    status: string | null;
    average_score: number | null;
    year: number | null;
  };

  let query = $state('');
  let results: AnimeSearchItem[] = $state([]);
  let loading = $state(false);
  let error = $state('');
  let selected: AnimeSearchItem | null = $state(null);

  async function runSearch() {
    if (!query.trim()) { results = []; return; }
    loading = true;
    error = '';
    try {
      results = await invoke<AnimeSearchItem[]>('search_anime', { query });
    } catch (e) {
      error = String(e);
      results = [];
    } finally {
      loading = false;
    }
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === 'Enter') runSearch();
  }
</script>

<div class="space-y-6">
  <h1 class="font-display text-3xl font-bold">Поиск аниме</h1>

  <div class="flex gap-2">
    <div class="relative flex-1">
      <Search size={18} class="absolute left-3 top-1/2 -translate-y-1/2 text-zinc-500" />
      <input
        class="w-full pl-10 pr-4 py-3 bg-surface border border-border rounded-lg text-zinc-100 placeholder-zinc-500 focus:border-gold focus:outline-none transition-colors"
        placeholder="Название аниме..."
        bind:value={query}
        onkeydown={onKey}
      />
    </div>
    <button
      class="px-8 py-3 bg-gold hover:bg-gold-light text-ink font-medium rounded-lg transition-colors disabled:opacity-50"
      onclick={runSearch}
      disabled={loading}
    >{loading ? '...' : 'Найти'}</button>
  </div>

  {#if error}
    <div class="p-4 rounded-lg bg-red-950/50 border border-red-800 text-sm text-red-200">
      <pre class="whitespace-pre-wrap">{error}</pre>
    </div>
  {/if}

  {#if results.length > 0}
    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
      {#each results as item (item.anilist_id)}
        <button
          type="button"
          onclick={() => selected = item}
          class="group text-left rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300 cursor-pointer"
        >
          {#if item.cover_url}
            <div class="aspect-[2/3] overflow-hidden">
              <img src={item.cover_url} alt={item.title} class="w-full h-full object-cover group-hover:scale-105 transition-transform duration-500" />
            </div>
          {:else}
            <div class="aspect-[2/3] bg-ink-light flex items-center justify-center text-zinc-600"><Film size={48} /></div>
          {/if}
          <div class="p-3">
            <p class="text-sm font-medium truncate">{item.title}</p>
            <div class="flex items-center justify-between mt-1">
              <span class="text-xs text-zinc-500">{item.year ?? '—'}</span>
              {#if item.average_score}
                <span class="flex items-center gap-1 text-xs text-gold"><Star size={12} fill="currentColor" />{(item.average_score / 10).toFixed(1)}</span>
              {/if}
            </div>
          </div>
        </button>
      {/each}
    </div>
  {/if}

  {#if query.trim() && !loading && results.length === 0 && !error}
    <p class="text-center text-zinc-500 mt-12">Ничего не найдено</p>
  {/if}
</div>

{#if selected}
  <AnimeModal item={selected} onClose={() => { selected = null; }} />
{/if}