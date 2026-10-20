<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { Clapperboard, Search } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import Pagination from '$lib/components/Pagination.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';

  type LibraryItem = {
    id: number; tmdb_id: number | null; title: string; type_field: string;
    year: number | null; poster_path: string | null; status: string;
    rating: number | null; added_at: string;
  };

  let items: LibraryItem[] = $state([]);
  let loading = $state(true);
  let currentPage = $state(1);
  const pageSize = 50;

  async function loadItems() {
    try {
      items = await invoke<LibraryItem[]>('get_library', { contentType: 'anime' });
    } catch (e) { console.error(e); } finally { loading = false; }
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

  function imgFallback(e: Event) {
    const img = e.currentTarget as HTMLImageElement;
    img.style.display = 'none';
  }
</script>

<PageHeader title={$t('library.anime_title')} subtitle={$t('library.anime_subtitle')} icon={Clapperboard} actionLabel="Найти аниме" actionHref="/search" />

{#if loading}
  <div class="text-center text-zinc-500 py-12">{$t('common.loading')}</div>
{:else if items.length === 0}
  <EmptyState
    icon={Clapperboard}
    title={$t('library.empty')}
    hint={$t('library.anime_subtitle')}
    ctaLabel={$t('library.empty_cta')}
    ctaHref="/search"
  />
{:else}
  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3 md:gap-4">
    {#each paginatedItems as item, i (item.id)}
      <a href={`/anime/${item.tmdb_id}`} class="block rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300">
        <div class="aspect-[2/3] bg-ink-light overflow-hidden">
          {#if item.poster_path}
            <img src={item.poster_path} alt={item.title} loading={i < 6 ? 'eager' : 'lazy'} decoding="async" fetchpriority={i < 6 ? 'high' : 'low'} onerror={imgFallback} class="w-full h-full object-cover" />
          {:else}
            <div class="w-full h-full flex items-center justify-center text-zinc-600 text-xs">Нет постера</div>
          {/if}
        </div>
        <div class="p-2 md:p-3">
          <p class="text-xs md:text-sm font-medium truncate">{item.title}</p>
          <div class="flex items-center justify-between mt-1">
            <span class="text-[10px] md:text-xs text-zinc-500">{item.year ?? '—'}</span>
            <span class="text-[9px] md:text-[10px] px-1.5 py-0.5 rounded bg-gold/10 text-gold uppercase">{$t('status.' + item.status)}</span>
          </div>
        </div>
      </a>
    {/each}
  </div>
  <Pagination currentPage={currentPage} totalPages={totalPages} onPageChange={(p) => currentPage = p} />
{/if}