<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { Settings as SettingsIcon, Globe, Sun, Moon, Monitor, KeyRound, AlertTriangle, RefreshCw } from 'lucide-svelte';
  import { settings } from '$lib/settings.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';

  let showResetModal = $state(false);
  let resetting = $state(false);
  let refreshingMeta = $state(false);
  let refreshingLang = $state(false);
  let refreshMsg = $state('');
  let refreshError = $state('');

  let tmdbKey = $state(settings.tmdbKey);

  function saveTmdbKey() {
    settings.setTmdbKey(tmdbKey);
  }

  async function confirmReset() {
    resetting = true;
    try {
      await invoke('reset_all_data');
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
      showResetModal = false;
      alert($t('settings.reset_success'));
    } catch (e) {
      console.error(e);
    } finally {
      resetting = false;
    }
  }

  async function refreshMetadata() {
    if (!confirm($t('settings.refresh_metadata_confirm'))) return;
    refreshingMeta = true;
    refreshMsg = '';
    refreshError = '';
    try {
      const count = await invoke<number>('refresh_metadata', {
        apiKey: settings.tmdbKey || null,
        locale: $locale,
      });
      refreshMsg = $t('settings.refresh_done', { values: { count } });
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
    } catch (e) {
      refreshError = $t('settings.refresh_error', { values: { error: String(e) } });
    } finally {
      refreshingMeta = false;
    }
  }

  async function refreshLanguage() {
    if (!confirm($t('settings.refresh_language_confirm'))) return;
    refreshingLang = true;
    refreshMsg = '';
    refreshError = '';
    try {
      const count = await invoke<number>('refresh_titles_language', {
        apiKey: settings.tmdbKey || null,
        locale: $locale,
      });
      refreshMsg = $t('settings.refresh_done', { values: { count } });
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
    } catch (e) {
      refreshError = $t('settings.refresh_error', { values: { error: String(e) } });
    } finally {
      refreshingLang = false;
    }
  }

  function setLocale(l: string) {
    if (typeof window !== 'undefined') {
      localStorage.setItem('35mm-locale', l);
      window.location.reload();
    }
  }
</script>

<PageHeader title={$t('settings.title')} subtitle={$t('settings.subtitle')} icon={SettingsIcon} />

<div class="max-w-3xl space-y-6">
  <!-- ОБЩИЕ -->
  <section class="bg-surface border border-border rounded-xl p-6">
    <h2 class="font-display text-lg font-semibold mb-4">{$t('settings.section_general')}</h2>

    <div class="space-y-4">
      <div>
        <label class="block text-sm text-zinc-400 mb-2">{$t('settings.language')}</label>
        <div class="flex gap-2">
          <button onclick={() => setLocale('ru')} class="px-4 py-2 rounded-lg text-sm transition-colors {$locale === 'ru' ? 'bg-gold text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}">RU</button>
          <button onclick={() => setLocale('en')} class="px-4 py-2 rounded-lg text-sm transition-colors {$locale === 'en' ? 'bg-gold text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}">EN</button>
          <button onclick={() => setLocale('zh-CN')} class="px-4 py-2 rounded-lg text-sm transition-colors {$locale === 'zh-CN' ? 'bg-gold text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}">中文</button>
        </div>
      </div>
    </div>
  </section>

  <!-- ВНЕШНИЙ ВИД -->
  <section class="bg-surface border border-border rounded-xl p-6">
    <h2 class="font-display text-lg font-semibold mb-4">{$t('settings.section_appearance')}</h2>

    <div class="space-y-4">
      <div>
        <label class="block text-sm text-zinc-400 mb-2">{$t('settings.theme')}</label>
        <div class="flex gap-2">
          <button onclick={() => settings.setTheme('dark')} class="flex items-center gap-2 px-4 py-2 rounded-lg text-sm transition-colors {settings.theme === 'dark' ? 'bg-gold text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}"><Moon size={14} /> {$t('settings.theme_dark')}</button>
          <button onclick={() => settings.setTheme('light')} class="flex items-center gap-2 px-4 py-2 rounded-lg text-sm transition-colors {settings.theme === 'light' ? 'bg-gold text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}"><Sun size={14} /> {$t('settings.theme_light')}</button>
          <button onclick={() => settings.setTheme('auto')} class="flex items-center gap-2 px-4 py-2 rounded-lg text-sm transition-colors {settings.theme === 'auto' ? 'bg-gold text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}"><Monitor size={14} /> {$t('settings.theme_auto')}</button>
        </div>
      </div>

      <div>
        <label class="block text-sm text-zinc-400 mb-2">{$t('settings.scale')}</label>
        <div class="flex gap-2">
          {#each ['100', '125', '150', '200'] as s}
            <button onclick={() => settings.setScale(s as any)} class="px-4 py-2 rounded-lg text-sm transition-colors {settings.scale === s ? 'bg-gold text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}">{$t(`settings.scale_${s}`)}</button>
          {/each}
        </div>
      </div>
    </div>
  </section>

  <!-- API -->
  <section class="bg-surface border border-border rounded-xl p-6">
    <h2 class="font-display text-lg font-semibold mb-4">{$t('settings.section_api')}</h2>

    <div class="space-y-3">
      <label class="block text-sm text-zinc-400">{$t('settings.tmdb_key')}</label>
      <input
        type="password"
        bind:value={tmdbKey}
        placeholder={$t('settings.tmdb_key_placeholder')}
        class="w-full px-3 py-2 bg-ink-light border border-border rounded-lg text-zinc-100 placeholder-zinc-500 focus:border-gold focus:outline-none"
      />
      <p class="text-xs text-zinc-500">{$t('settings.tmdb_key_hint')}</p>
      <button onclick={saveTmdbKey} class="px-4 py-2 bg-gold hover:bg-gold-light text-ink font-medium rounded-lg text-sm transition-colors">{$t('settings.save')}</button>
    </div>
  </section>

  <!-- ДОПОЛНИТЕЛЬНО -->
  <section class="bg-surface border border-border rounded-xl p-6">
    <h2 class="font-display text-lg font-semibold mb-4">{$t('settings.section_advanced')}</h2>

    <div class="space-y-4">
      <div>
        <button
          onclick={refreshMetadata}
          disabled={refreshingMeta}
          class="flex items-center gap-2 px-4 py-2 rounded-lg bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold transition-colors text-sm disabled:opacity-50"
        >
          <RefreshCw size={16} class={refreshingMeta ? 'animate-spin' : ''} />
          {refreshingMeta ? $t('common.loading') : $t('settings.refresh_metadata')}
        </button>
        <p class="text-xs text-zinc-500 mt-1">{$t('settings.refresh_metadata_hint')}</p>
      </div>

      <div>
        <button
          onclick={refreshLanguage}
          disabled={refreshingLang}
          class="flex items-center gap-2 px-4 py-2 rounded-lg bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold transition-colors text-sm disabled:opacity-50"
        >
          <Globe size={16} class={refreshingLang ? 'animate-spin' : ''} />
          {refreshingLang ? $t('common.loading') : $t('settings.refresh_language')}
        </button>
        <p class="text-xs text-zinc-500 mt-1">{$t('settings.refresh_language_hint')}</p>
      </div>

      {#if refreshMsg}
        <div class="p-3 rounded-lg bg-emerald-950/50 border border-emerald-800 text-sm text-emerald-200">{refreshMsg}</div>
      {/if}
      {#if refreshError}
        <div class="p-3 rounded-lg bg-red-950/50 border border-red-800 text-sm text-red-200">{refreshError}</div>
      {/if}
    </div>
  </section>

  <!-- ОПАСНАЯ ЗОНА -->
  <section class="bg-surface border border-red-900/50 rounded-xl p-6">
    <h2 class="font-display text-lg font-semibold mb-4 flex items-center gap-2 text-red-400">
      <AlertTriangle size={18} />
      {$t('settings.section_danger')}
    </h2>

    <div class="space-y-3">
      <p class="text-sm text-zinc-400">{$t('settings.reset_description')}</p>
      <button
        onclick={() => showResetModal = true}
        class="px-4 py-2 bg-red-900/50 hover:bg-red-800/60 border border-red-800 text-red-200 rounded-lg text-sm font-medium transition-colors"
      >{$t('settings.reset_button')}</button>
    </div>
  </section>
</div>

{#if showResetModal}
  <div class="fixed inset-0 z-50 flex items-center justify-center p-5 bg-black/70 backdrop-blur-sm" onclick={(e) => e.target === e.currentTarget && (showResetModal = false)} role="presentation">
    <div class="w-full max-w-md bg-surface border border-red-900/50 rounded-xl p-6" onclick={(e) => e.stopPropagation()}>
      <h2 class="font-display text-xl font-bold mb-2 text-red-400">{$t('settings.reset_modal_title')}</h2>
      <p class="text-sm text-zinc-400 mb-4">{$t('settings.reset_modal_description')}</p>
      <div class="flex justify-end gap-2">
        <button onclick={() => showResetModal = false} class="px-4 py-2 text-sm text-zinc-400 hover:text-zinc-100">{$t('settings.reset_cancel')}</button>
        <button
          onclick={confirmReset}
          disabled={resetting}
          class="px-4 py-2 bg-red-700 hover:bg-red-600 text-white font-medium rounded-lg text-sm transition-colors disabled:opacity-50"
        >{resetting ? $t('common.loading') : $t('settings.reset_confirm')}</button>
      </div>
    </div>
  </div>
{/if}