<script lang="ts">
  import { t } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { BookOpen, Star } from 'lucide-svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';
  import Pagination from '$lib/components/Pagination.svelte';

  type DiaryEntry = {
    id: number;
    watched_at: string;
    rating: number | null;
    notes: string | null;
    is_rewatch: boolean;
    title: string;
    poster_url: string | null;
    type_field: string;
    tmdb_id: number;
  };

  let entries: DiaryEntry[] = $state([]);
  let loading = $state(true);
  let currentPage = $state(1);
  const PER_PAGE = 100;

  async function loadDiary() {
    try {
      entries = await invoke<DiaryEntry[]>('get_diary');
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  let totalPages = $derived(Math.max(1, Math.ceil(entries.length / PER_PAGE)));
  let paginatedEntries = $derived(entries.slice((currentPage - 1) * PER_PAGE, currentPage * PER_PAGE));

  function formatDate(dateStr: string) {
    try {
      const d = new Date(dateStr.replace(' ', 'T') + 'Z');
      return d.toLocaleDateString('ru-RU', { day: 'numeric', month: 'long', year: 'numeric' });
    } catch {
      return dateStr;
    }
  }

  onMount(loadDiary);
</script>

<PageHeader title={$t('diary.title')} subtitle={$t('diary.subtitle')} icon={BookOpen} />

{#if loading}
  <div class="text-center text-zinc-500 py-20">{$t('common.loading')}</div>
{:else if entries.length === 0}
  <EmptyState
    icon={BookOpen}
    title={$t('diary.empty')}
    hint={$t('diary.empty_hint')}
  />
{:else}
  <div class="space-y-3">
    {#each paginatedEntries as entry}
      <div class="flex gap-4 p-4 bg-surface border border-border rounded-xl">
        {#if entry.poster_url}
          <img src={entry.poster_url} alt={entry.title} class="w-16 h-24 object-cover rounded-md shrink-0" />
        {:else}
          <div class="w-16 h-24 bg-ink-light rounded-md shrink-0 flex items-center justify-center text-zinc-600 text-[10px]">Нет</div>
        {/if}

        <div class="flex-1 min-w-0">
          <div class="flex items-start justify-between gap-2">
            <div class="min-w-0">
              <p class="text-xs text-zinc-500 mb-1">{formatDate(entry.watched_at)}</p>
              <h3 class="font-medium truncate">{entry.title}</h3>
            </div>
            {#if entry.rating}
              <div class="flex items-center gap-1 text-gold shrink-0">
                <Star size={14} fill="currentColor" />
                <span class="text-sm font-medium">{entry.rating.toFixed(1)}</span>
              </div>
            {/if}
          </div>

          {#if entry.is_rewatch}
            <span class="inline-block mt-2 text-[10px] px-2 py-0.5 rounded bg-gold/10 border border-gold/30 text-gold uppercase tracking-wider">Пересмотр</span>
          {/if}

          {#if entry.notes}
            <p class="text-sm text-zinc-400 mt-2 line-clamp-2">{entry.notes}</p>
          {/if}
        </div>
      </div>
    {/each}
  </div>

  <Pagination
    currentPage={currentPage}
    totalPages={totalPages}
    onPageChange={(p) => currentPage = p}
  />
{/if}