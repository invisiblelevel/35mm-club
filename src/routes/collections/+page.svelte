<script lang="ts">
  import { t } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { FolderHeart, Plus, X, Trash2, Film, Tv, Clapperboard, Image } from 'lucide-svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';

  type Collection = {
    id: number;
    name: string;
    description: string | null;
    created_at: string;
    item_count: number;
    preview_posters: string[];
  };

  type CollectionItem = {
    user_title_id: number;
    title: string;
    poster_url: string | null;
    type_field: string;
    tmdb_id: number | null;
    year: number | null;
    status: string;
  };

  let collections: Collection[] = $state([]);
  let loading = $state(true);
  let showCreate = $state(false);
  let newName = $state('');
  let newDesc = $state('');
  let creating = $state(false);

  let openedCollection: Collection | null = $state(null);
  let items: CollectionItem[] = $state([]);
  let loadingItems = $state(false);

  async function loadCollections() {
    loading = true;
    try {
      collections = await invoke<Collection[]>('get_collections');
    } catch (e) { console.error(e); } finally { loading = false; }
  }

  onMount(loadCollections);

  async function createCollection() {
    if (!newName.trim() || creating) return;
    creating = true;
    try {
      await invoke('create_collection', { name: newName, description: newDesc || null });
      newName = '';
      newDesc = '';
      showCreate = false;
      await loadCollections();
    } catch (e) { console.error(e); } finally { creating = false; }
  }

  async function deleteCollection(id: number) {
    if (!confirm($t('collections.delete_confirm'))) return;
    try {
      await invoke('delete_collection', { collectionId: id });
      if (openedCollection?.id === id) openedCollection = null;
      await loadCollections();
    } catch (e) { console.error(e); }
  }

  async function openCollection(c: Collection) {
    openedCollection = c;
    loadingItems = true;
    try {
      items = await invoke<CollectionItem[]>('get_collection_items', { collectionId: c.id });
    } catch (e) { console.error(e); } finally { loadingItems = false; }
  }

  async function removeItem(userTitleId: number) {
    if (!openedCollection) return;
    try {
      await invoke('remove_from_collection', { collectionId: openedCollection.id, userTitleId });
      items = items.filter(i => i.user_title_id !== userTitleId);
      await loadCollections();
    } catch (e) { console.error(e); }
  }

  function typeIcon(type: string) {
    if (type === 'movie') return Film;
    if (type === 'tv') return Tv;
    return Clapperboard;
  }
</script>

<PageHeader
  title={$t('collections.title')}
  subtitle={$t('collections.subtitle')}
  icon={FolderHeart}
/>

<div class="mb-6 flex justify-end">
  <button
    onclick={() => showCreate = true}
    class="flex items-center gap-2 px-4 py-2 bg-gold hover:bg-gold-light text-ink font-medium rounded-lg transition-colors"
  >
    <Plus size={18} />
    {$t('collections.create')}
  </button>
</div>

{#if loading}
  <div class="text-center text-zinc-500 py-12">{$t('common.loading')}</div>
{:else if collections.length === 0}
  <div class="bg-surface border border-border rounded-xl p-12 text-center">
    <FolderHeart size={48} class="mx-auto text-zinc-600 mb-4" />
    <p class="text-zinc-500">{$t('collections.empty')}</p>
  </div>
{:else}
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
    {#each collections as c (c.id)}
      <div
        onclick={() => openCollection(c)}
        onkeydown={(e) => e.key === 'Enter' && openCollection(c)}
        role="button"
        tabindex="0"
        class="text-left bg-surface border border-border rounded-xl overflow-hidden hover:border-gold/50 transition-colors group cursor-pointer flex flex-col"
      >
        <!-- УМНАЯ СЕТКА ПРЕВЬЮ -->
        <div class="aspect-[2/1] bg-ink-light relative overflow-hidden shrink-0">
          {#if c.preview_posters.length > 0}
            <div class="w-full h-full grid {c.preview_posters.length === 1 ? 'grid-cols-1' : 'grid-cols-2'}">
              {#each c.preview_posters.slice(0, 4) as poster}
                <img src={poster} alt="" class="w-full h-full object-cover" />
              {/each}
            </div>
          {:else}
            <div class="w-full h-full flex items-center justify-center text-zinc-700">
              <Image size={40} />
            </div>
          {/if}
          <div class="absolute inset-0 bg-gradient-to-t from-surface via-transparent to-transparent opacity-80 pointer-events-none"></div>
        </div>

        <div class="p-4 relative flex-1">
          <div class="flex items-start justify-between mb-1">
            <h3 class="font-display text-lg font-semibold text-zinc-100 truncate">{c.name}</h3>
            <button
              onclick={(e) => { e.stopPropagation(); deleteCollection(c.id); }}
              class="opacity-0 group-hover:opacity-100 text-zinc-500 hover:text-red-400 transition-all shrink-0 ml-2"
              aria-label={$t('collections.delete_aria')}
            >
              <Trash2 size={16} />
            </button>
          </div>
          {#if c.description}
            <p class="text-xs text-zinc-500 line-clamp-2 mb-2">{c.description}</p>
          {/if}
          <p class="text-xs text-gold">{c.item_count} {c.item_count === 1 ? $t('collections.item_one') : $t('collections.item_many')}</p>
        </div>
      </div>
    {/each}
  </div>
{/if}

{#if showCreate}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-5 bg-black/70 backdrop-blur-sm" onclick={(e) => e.target === e.currentTarget && (showCreate = false)} role="presentation">
    <div class="w-full max-w-md bg-surface border border-border rounded-xl p-6" onclick={(e) => e.stopPropagation()}>
      <div class="flex items-center justify-between mb-4">
        <h2 class="font-display text-xl font-bold">{$t('collections.modal_title')}</h2>
        <button onclick={() => showCreate = false} class="text-zinc-500 hover:text-zinc-300"><X size={20} /></button>
      </div>
      <input
        type="text"
        bind:value={newName}
        placeholder={$t('collections.field_name')}
        class="w-full px-3 py-2 mb-3 bg-ink-light border border-border rounded-lg text-zinc-100 placeholder-zinc-500 focus:border-gold focus:outline-none"
      />
      <textarea
        bind:value={newDesc}
        placeholder={$t('collections.field_description')}
        rows="2"
        class="w-full px-3 py-2 mb-4 bg-ink-light border border-border rounded-lg text-zinc-100 placeholder-zinc-500 focus:border-gold focus:outline-none resize-none"
      ></textarea>
      <div class="flex justify-end gap-2">
        <button onclick={() => showCreate = false} class="px-4 py-2 text-sm text-zinc-400 hover:text-zinc-100">{$t('collections.cancel')}</button>
        <button
          onclick={createCollection}
          disabled={creating || !newName.trim()}
          class="px-4 py-2 bg-gold hover:bg-gold-light text-ink font-medium rounded-lg transition-colors disabled:opacity-50"
        >{$t('collections.save')}</button>
      </div>
    </div>
  </div>
{/if}

{#if openedCollection}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-5 bg-black/70 backdrop-blur-sm" onclick={(e) => e.target === e.currentTarget && (openedCollection = null)} role="presentation">
    <div class="w-full max-w-3xl bg-surface border border-border rounded-xl p-6 max-h-[80vh] overflow-y-auto" onclick={(e) => e.stopPropagation()}>
      <div class="flex items-center justify-between mb-4">
        <div>
          <h2 class="font-display text-2xl font-bold">{openedCollection.name}</h2>
          {#if openedCollection.description}
            <p class="text-sm text-zinc-500 mt-1">{openedCollection.description}</p>
          {/if}
        </div>
        <button onclick={() => openedCollection = null} class="text-zinc-500 hover:text-zinc-300"><X size={24} /></button>
      </div>

      {#if loadingItems}
        <div class="text-center text-zinc-500 py-8">{$t('common.loading')}</div>
      {:else if items.length === 0}
        <div class="text-center text-zinc-500 py-8">
          <p>{$t('collections.empty_title')}</p>
          <p class="text-xs text-zinc-600 mt-1">{$t('collections.empty_hint')}</p>
        </div>
      {:else}
        <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-5 gap-3">
          {#each items as item (item.user_title_id)}
            <div class="relative group">
              <a href={`/title/${item.tmdb_id}?type=${item.type_field}`} class="block">
                {#if item.poster_url}
                  <img src={item.poster_url} alt={item.title} class="w-full aspect-[2/3] object-cover rounded-lg" />
                {:else}
                  <div class="w-full aspect-[2/3] bg-ink-light rounded-lg flex items-center justify-center">
                    <svelte:component this={typeIcon(item.type_field)} size={32} class="text-zinc-600" />
                  </div>
                {/if}
                <p class="text-xs text-zinc-300 mt-1 truncate">{item.title}</p>
              </a>
              <button
                onclick={() => removeItem(item.user_title_id)}
                class="absolute top-1 right-1 w-7 h-7 rounded-full bg-red-950/80 border border-red-800 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity text-red-300 hover:text-red-100"
                aria-label={$t('collections.remove_aria')}
              >
                <X size={14} />
              </button>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </div>
{/if}