<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { Clapperboard, Search } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import Pagination from '$lib/components/Pagination.svelte';

  type LibraryItem = {
    id: number;
    tmdb_id: number | null;
    title: string;
    type_field: string;
    year: number | null;
    poster_path: string | null;
    status: string;
    rating: number | null;
    added_at: string;
  };

  let items: LibraryItem[] = $state([]);
  let loading = $state(true);
  let currentPage = $state(1);
  const pageSize = 50;

  async function loadItems() {
    try {
      items = await invoke<LibraryItem[]>('get_library', { contentType: 'anime' });
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  onMount(loadItems);

  let lastLocale = $state('');
  $effect(() => {
    const l = $locale;
    if (lastLocale === '') { lastLocale = l; return; }
    if (lastLocale !== l) { lastLocale = l; loadItems(); }
  });

  const totalPages = $derived(Math.max(1, Math.ceil(items.length / pageSize)));
  const paginatedItems = $derived(items.slice((currentPage - 1) * pageSize, currentPage * pageSize));
</script>

<PageHeader
  title={$t('library.anime_title')}
  subtitle={$t('library.anime_subtitle')}
  icon={Clapperboard}
  actionLabel="Найти аниме"
  actionHref="/search"
/>

{#if loading}
  <div class="text-center text-zinc-500 py-12">{$t('common.loading')}</div>
{:else if items.length === 0}
  <div class="bg-surface border border-border rounded-xl p-12 text-center">
    <Clapperboard size={48} class="mx-auto text-zinc-600 mb-4" />
    <p class="text-zinc-500 mb-4">Аниме ещё не добавлены</p>
    <a href="/search" class="inline-flex items-center gap-2 px-5 py-2.5 bg-gold hover:bg-gold-light text-ink font-medium rounded-lg transition-colors">
      <Search size={18} />
      Найти аниме
    </a>
  </div>
{:else}
  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
    {#each paginatedItems as item (item.id)}
      <a href={`/anime/${item.tmdb_id}`} class="block rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300">
        {#if item.poster_path}
          <div class="aspect-[2/3] overflow-hidden">
            <img src={item.poster_path} alt={item.title} class="w-full h-full object-cover" />
          </div>
        {:else}
          <div class="aspect-[2/3] bg-ink-light flex items-center justify-center text-zinc-600 text-xs">Нет постера</div>
        {/if}
        <div class="p-3">
          <p class="text-sm font-medium truncate">{item.title}</p>
          <div class="flex items-center justify-between mt-1">
            <span class="text-xs text-zinc-500">{item.year ?? '—'}</span>
            <span class="text-[10px] px-1.5 py-0.5 rounded bg-gold/10 text-gold uppercase">{$t('status.' + item.status)}</span>
          </div>
        </div>
      </a>
    {/each}
  </div>

  <Pagination currentPage={currentPage} totalPages={totalPages} onPageChange={(p) => currentPage = p} />
{/if}