<script lang="ts">
  import { t } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { BookOpen, Star, Film, Tv, Clapperboard, Pencil, Check, X, Repeat } from 'lucide-svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';

  type LibraryDiaryEntry = {
    user_title_id: number;
    title: string;
    poster_url: string | null;
    type_field: string;
    tmdb_id: number | null;
    status: string;
    rating: number | null;
    notes: string | null;
    added_at: string;
  };

  type RewatchDiaryEntry = {
    id: number;
    watched_at: string;
    rating: number | null;
    notes: string | null;
    title: string;
    poster_url: string | null;
    type_field: string;
    tmdb_id: number | null;
  };

  const statusTabs: { value: string | null; label: string }[] = [
    { value: null, label: 'diary.tab_all' },
    { value: 'watching', label: 'status.watching' },
    { value: 'completed', label: 'status.completed' },
    { value: 'planned', label: 'status.planned' },
    { value: 'on_hold', label: 'status.on_hold' },
    { value: 'dropped', label: 'status.dropped' },
  ];

  let activeTab = $state<string | null>(null);
  let activeView = $state<'library' | 'rewatches'>('library');
  let entries: LibraryDiaryEntry[] = $state([]);
  let rewatches: RewatchDiaryEntry[] = $state([]);
  let loading = $state(true);
  let editingId = $state<number | null>(null);
  let editingNotes = $state('');

  async function loadDiary() {
    loading = true;
    try {
      if (activeView === 'library') {
        entries = await invoke<LibraryDiaryEntry[]>('get_library_diary', { status: activeTab });
      } else {
        rewatches = await invoke<RewatchDiaryEntry[]>('get_all_rewatches');
      }
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  $effect(() => {
    void activeTab;
    void activeView;
    loadDiary();
  });

  function startEdit(entry: LibraryDiaryEntry) {
    editingId = entry.user_title_id;
    editingNotes = entry.notes ?? '';
  }

  function cancelEdit() {
    editingId = null;
    editingNotes = '';
  }

  async function saveNotes(entry: LibraryDiaryEntry) {
    try {
      const trimmed = editingNotes.trim() || null;
      await invoke('update_title_notes', {
        userTitleId: entry.user_title_id,
        notes: trimmed,
      });
      entry.notes = trimmed;
      editingId = null;
      editingNotes = '';
    } catch (e) {
      console.error(e);
    }
  }

  function typeIcon(type: string) {
    if (type === 'movie') return Film;
    if (type === 'tv') return Tv;
    return Clapperboard;
  }

  function getTitleUrl(entry: { type_field: string; tmdb_id: number | null }): string {
    if (entry.type_field === 'anime') return `/anime/${entry.tmdb_id}`;
    return `/title/${entry.tmdb_id}?type=${entry.type_field}`;
  }

  function formatDate(iso: string): string {
    try {
      const d = new Date(iso.replace(' ', 'T') + 'Z');
      return d.toLocaleDateString();
    } catch { return iso; }
  }
</script>

<PageHeader title={$t('diary.title')} subtitle={$t('diary.subtitle')} icon={BookOpen} />

<div class="flex gap-1 mb-4">
  <button
    onclick={() => activeView = 'library'}
    class="px-4 py-2 rounded-lg text-sm font-medium transition-colors {activeView === 'library' ? 'bg-gold text-ink' : 'bg-surface border border-border text-zinc-400 hover:text-zinc-100'}"
  >{$t('diary.title')}</button>
  <button
    onclick={() => activeView = 'rewatches'}
    class="px-4 py-2 rounded-lg text-sm font-medium transition-colors flex items-center gap-1.5 {activeView === 'rewatches' ? 'bg-gold text-ink' : 'bg-surface border border-border text-zinc-400 hover:text-zinc-100'}"
  >
    <Repeat size={14} />
    {$t('rewatch.diary_tab')}
  </button>
</div>

{#if activeView === 'library'}
  <div class="flex gap-2 border-b border-border overflow-x-auto mb-4 md:mb-6">
    {#each statusTabs as tab}
      <button
        onclick={() => activeTab = tab.value}
        class="px-4 py-3 text-sm font-medium transition-colors relative whitespace-nowrap {activeTab === tab.value ? 'text-gold' : 'text-zinc-400 hover:text-zinc-100'}"
      >
        {$t(tab.label)}
        {#if activeTab === tab.value}
          <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-gold"></div>
        {/if}
      </button>
    {/each}
  </div>
{/if}

{#if loading}
  <div class="text-center text-zinc-500 py-20">{$t('common.loading')}</div>
{:else if activeView === 'library'}
  {#if entries.length === 0}
    <EmptyState icon={BookOpen} title={$t('diary.empty')} hint={$t('diary.empty_hint')} />
  {:else}
    <div class="space-y-2 md:space-y-3">
      {#each entries as entry (entry.user_title_id)}
        {@const TypeIcon = typeIcon(entry.type_field)}
        <div class="bg-surface border border-border rounded-xl p-3 md:p-4">
          <div class="flex gap-3 md:gap-4">
            <a href={getTitleUrl(entry)} class="shrink-0">
              {#if entry.poster_url}
                <img src={entry.poster_url} alt={entry.title} loading="lazy" decoding="async" class="w-14 h-20 md:w-16 md:h-24 object-cover rounded-md" />
              {:else}
                <div class="w-14 h-20 md:w-16 md:h-24 bg-ink-light rounded-md flex items-center justify-center">
                  <svelte:component this={TypeIcon} size={20} class="text-zinc-600" />
                </div>
              {/if}
            </a>
            <div class="flex-1 min-w-0">
              <div class="flex items-start justify-between gap-2 mb-1">
                <div class="min-w-0 flex-1">
                  <a href={getTitleUrl(entry)} class="font-medium text-zinc-100 hover:text-gold transition-colors truncate block text-sm md:text-base">{entry.title}</a>
                  <div class="flex items-center gap-2 mt-1 flex-wrap">
                    <span class="text-[10px] px-1.5 py-0.5 rounded bg-gold/10 text-gold uppercase">{$t('status.' + entry.status)}</span>
                    {#if entry.rating}
                      <span class="flex items-center gap-1 text-[11px] text-gold"><Star size={11} fill="currentColor" />{entry.rating.toFixed(1)}</span>
                    {/if}
                  </div>
                </div>
                {#if editingId === entry.user_title_id}
                  <div class="flex gap-1 shrink-0">
                    <button onclick={() => saveNotes(entry)} class="w-8 h-8 rounded-md bg-emerald-600 hover:bg-emerald-500 text-white flex items-center justify-center"><Check size={14} /></button>
                    <button onclick={cancelEdit} class="w-8 h-8 rounded-md bg-surface border border-border text-zinc-400 hover:text-zinc-100 flex items-center justify-center"><X size={14} /></button>
                  </div>
                {:else}
                  <button onclick={() => startEdit(entry)} class="w-8 h-8 rounded-md text-zinc-500 hover:text-gold transition-colors shrink-0 flex items-center justify-center"><Pencil size={14} /></button>
                {/if}
              </div>
              {#if editingId === entry.user_title_id}
                <textarea bind:value={editingNotes} class="w-full mt-2 px-3 py-2 bg-ink-light border border-border rounded-lg text-zinc-100 placeholder-zinc-500 focus:border-gold focus:outline-none resize-none text-sm" rows="3" placeholder={$t('diary.notes_placeholder')}></textarea>
              {:else if entry.notes}
                <p class="mt-2 text-xs md:text-sm text-zinc-400 whitespace-pre-wrap line-clamp-3">{entry.notes}</p>
              {:else}
                <p class="mt-2 text-xs text-zinc-600 italic">{$t('diary.notes_empty')}</p>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
{:else}
  {#if rewatches.length === 0}
    <EmptyState icon={Repeat} title={$t('rewatch.empty')} hint={$t('rewatch.empty_hint')} />
  {:else}
    <div class="space-y-2 md:space-y-3">
      {#each rewatches as rw (rw.id)}
        {@const TypeIcon = typeIcon(rw.type_field)}
        <div class="bg-surface border border-border rounded-xl p-3 md:p-4">
          <div class="flex gap-3 md:gap-4">
            <a href={getTitleUrl(rw)} class="shrink-0">
              {#if rw.poster_url}
                <img src={rw.poster_url} alt={rw.title} loading="lazy" decoding="async" class="w-14 h-20 md:w-16 md:h-24 object-cover rounded-md" />
              {:else}
                <div class="w-14 h-20 md:w-16 md:h-24 bg-ink-light rounded-md flex items-center justify-center">
                  <svelte:component this={TypeIcon} size={20} class="text-zinc-600" />
                </div>
              {/if}
            </a>
            <div class="flex-1 min-w-0">
              <a href={getTitleUrl(rw)} class="font-medium text-zinc-100 hover:text-gold transition-colors truncate block text-sm md:text-base">{rw.title}</a>
              <div class="flex items-center gap-2 mt-1 flex-wrap">
                <span class="text-[10px] px-1.5 py-0.5 rounded bg-gold/10 text-gold uppercase flex items-center gap-1">
                  <Repeat size={10} />{$t('diary.rewatch_badge')}
                </span>
                <span class="text-[11px] text-zinc-500">{formatDate(rw.watched_at)}</span>
                {#if rw.rating}
                  <span class="flex items-center gap-1 text-[11px] text-gold"><Star size={11} fill="currentColor" />{rw.rating.toFixed(1)}</span>
                {/if}
              </div>
              {#if rw.notes}
                <p class="mt-2 text-xs md:text-sm text-zinc-400 whitespace-pre-wrap line-clamp-3">{rw.notes}</p>
              {/if}
            </div>
          </div>
        </div>
      {/each}
    </div>
  {/if}
{/if}