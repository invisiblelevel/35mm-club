<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { Tv } from 'lucide-svelte';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import Pagination from '$lib/components/Pagination.svelte';
  import Spinner from '$lib/components/Spinner.svelte';

  type LibraryItem = {
    id: number; tmdb_id: number | null; title: string; type_field: string;
    year: number | null; poster_path: string | null; status: string;
    rating: number | null; added_at: string;
  };

  type DisplayItem = LibraryItem & {
    display_src: string | null;
    original_src: string | null;
  };

  let items = $state<DisplayItem[]>([]);
  let loading = $state(true);
  let currentPage = $state(1);
  const pageSize = 50;
  const TMDB_IMG = 'https://image.tmdb.org/t/p/w200';

  async function cacheCover(url: string | null): Promise<string | null> {
    if (!url) return null;
    if (!url.startsWith('http')) return url;
    try {
      return await invoke<string>('cache_cover', { url });
    } catch (e) {
      return url;
    }
  }

  function imgFallback(e: Event) {
    const img = e.currentTarget as HTMLImageElement;
    const orig = img.dataset.orig;
    if (orig && img.src !== orig) {
      img.src = orig;
      delete img.dataset.orig;
      return;
    }
    img.style.opacity = '0';
  }

  function imgLoaded(e: Event) {
    (e.currentTarget as HTMLImageElement).style.opacity = '1';
  }

  function makeDisplaySrc(poster_path: string | null): string | null {
    if (!poster_path) return null;
    if (poster_path.startsWith('http') || poster_path.startsWith('data:')) return poster_path;
    return TMDB_IMG + poster_path;
  }

  function makeOriginalSrc(poster_path: string | null): string | null {
    if (!poster_path) return null;
    if (poster_path.startsWith('data:')) return null;
    if (poster_path.startsWith('http')) return poster_path;
    return TMDB_IMG + poster_path;
  }

  async function loadItems() {
    try {
      const raw = await invoke<LibraryItem[]>('get_library', { contentType: 'tv' });

      // Сразу показываем с оригинальными URL
      items = raw.map(item => ({
        ...item,
        display_src: makeDisplaySrc(item.poster_path),
        original_src: makeOriginalSrc(item.poster_path),
      }));
      loading = false;

      // Фоном кэшируем
      raw.forEach(async (item) => {
        if (!item.poster_path) return;
        if (item.poster_path.startsWith('data:')) return;
        const fullUrl = makeOriginalSrc(item.poster_path);
        if (!fullUrl) return;
        const cached = await cacheCover(fullUrl);
        if (!cached || cached === item.poster_path) return;

        // Находим индекс и меняем ТОЛЬКО display_src
        const idx = items.findIndex(it => it.id === item.id);
        if (idx !== -1) {
          items[idx] = { ...items[idx], display_src: cached };
        }
      });
    } catch (e) {
      console.error(e);
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

<PageHeader title={$t('library.tv_title')} subtitle={$t('library.tv_subtitle')} icon={Tv} actionLabel={$t('library.empty_cta')} actionHref="/search" />

{#if loading}
  <div class="flex items-center justify-center py-20"><Spinner size={40} /></div>
{:else if items.length === 0}
  <EmptyState icon={Tv} title={$t('library.empty')} ctaLabel={$t('library.empty_cta')} ctaHref="/search" />
{:else}
  <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-3 md:gap-4">
    {#each paginatedItems as item (item.id)}
      <a href={`/title/${item.tmdb_id}?type=tv`} class="block rounded-xl bg-surface border border-border overflow-hidden hover:border-gold/50 transition-all duration-300">
        <div class="aspect-[2/3] bg-ink-light overflow-hidden relative">
          <div class="absolute inset-0 flex items-center justify-center text-zinc-600 pointer-events-none"><Tv size={48} /></div>
          {#if item.display_src}
            <img
              src={item.display_src}
              data-orig={item.original_src ?? ''}
              alt={item.title}
              loading="eager"
              decoding="sync"
              referrerpolicy="no-referrer"
              onerror={imgFallback}
              onload={imgLoaded}
              class="absolute inset-0 w-full h-full object-cover z-10"
            />
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