<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { Tv } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
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

  const TMDB_IMG = 'https://image.tmdb.org/t/p/w500';

  async function loadItems() {
    try {
      items = await invoke<LibraryItem[]>('get_library', { contentType: 'tv' });
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
  title={$t('library.tv_title')}
  subtitle={$t('library.tv_subtitle')}
  icon={Tv}
  actionLabel={$t('library.empty_cta')}
  actionHref="/search"
/>

{#if loading}
  <div class="text-center text-zinc-500 py-12">{$t('common.loading')}</div>
{:else if items.length === 0}
  <EmptyState
    icon={Tv}
    title={$t('library.empty')}
    ctaLabel={$t('library.empty_cta')}
    ctaHref="/search"
  />
{:else}
  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-4">
    {#each paginatedItems as item (item.id)}
      <a href={`/title/${item.tmdb_id}?type=tv`} class="block rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300">
        {#if item.poster_path}
          <div class="aspect-[2/3] overflow-hidden">
            <img src={TMDB_IMG + item.poster_path} alt={item.title} class="w-full h-full object-cover" />
          </div>
        {:else}
          <div class="aspect-[2/3] bg-ink-light flex items-center justify-center text-zinc-600 text-xs">{$t('search.no_poster')}</div>
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