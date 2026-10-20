<script lang="ts">
  import { t, locale } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { Settings as SettingsIcon, Globe, Sun, Moon, Monitor, AlertTriangle, RefreshCw, Download, Upload } from 'lucide-svelte';
  import { settings } from '$lib/settings.svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import { save, open } from '@tauri-apps/plugin-dialog';
  import { writeTextFile, readTextFile } from '@tauri-apps/plugin-fs';
  import { openUrl } from '@tauri-apps/plugin-opener';

  let showResetModal = $state(false);
  let resetting = $state(false);
  let refreshingMeta = $state(false);
  let refreshingLang = $state(false);
  let exporting = $state(false);
  let importing = $state(false);
  let refreshMsg = $state('');
  let refreshError = $state('');
  let backupMsg = $state('');

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

  async function exportData() {
    exporting = true;
    backupMsg = '';
    try {
      const json = await invoke<string>('export_data');
      const path = await save({
        defaultPath: `35mm-club-backup-${new Date().toISOString().slice(0, 10)}.json`,
        filters: [{ name: 'JSON', extensions: ['json'] }],
      });
      if (path) {
        await writeTextFile(path, json);
        backupMsg = $t('settings.export_done');
      }
    } catch (e) {
      backupMsg = $t('settings.export_error', { values: { error: String(e) } });
    } finally {
      exporting = false;
    }
  }

  async function importData() {
    if (!confirm($t('settings.import_confirm'))) return;
    importing = true;
    backupMsg = '';
    try {
      const path = await open({ multiple: false, filters: [{ name: 'JSON', extensions: ['json'] }] });
      if (!path) { importing = false; return; }
      const json = await readTextFile(path as string);
      await invoke('import_data', { jsonStr: json });
      window.dispatchEvent(new CustomEvent('35mm-data-changed'));
      backupMsg = $t('settings.import_done');
    } catch (e) {
      backupMsg = $t('settings.import_error', { values: { error: String(e) } });
    } finally {
      importing = false;
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

<div class="max-w-3xl space-y-4 md:space-y-6">
  <!-- ОБЩИЕ -->
  <section class="bg-surface border border-border rounded-xl p-4 md:p-6">
    <h2 class="font-display text-base md:text-lg font-semibold mb-3 md:mb-4">{$t('settings.section_general')}</h2>

    <div class="space-y-4">
      <div>
        <label class="block text-sm text-zinc-400 mb-2">{$t('settings.language')}</label>
        <div class="grid grid-cols-3 gap-2">
          <button onclick={() => setLocale('ru')} class="px-3 py-2.5 rounded-lg text-sm transition-all active:scale-[0.98] {$locale === 'ru' ? 'bg-gradient-to-b from-gold to-gold-dark text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}">RU</button>
          <button onclick={() => setLocale('en')} class="px-3 py-2.5 rounded-lg text-sm transition-all active:scale-[0.98] {$locale === 'en' ? 'bg-gradient-to-b from-gold to-gold-dark text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}">EN</button>
          <button onclick={() => setLocale('zh-CN')} class="px-3 py-2.5 rounded-lg text-sm transition-all active:scale-[0.98] {$locale === 'zh-CN' ? 'bg-gradient-to-b from-gold to-gold-dark text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}">中文</button>
        </div>
      </div>
    </div>
  </section>

  <!-- ВНЕШНИЙ ВИД -->
  <section class="bg-surface border border-border rounded-xl p-4 md:p-6">
    <h2 class="font-display text-base md:text-lg font-semibold mb-3 md:mb-4">{$t('settings.section_appearance')}</h2>

    <div class="space-y-4">
      <div>
        <label class="block text-sm text-zinc-400 mb-2">{$t('settings.theme')}</label>
        <div class="grid grid-cols-3 gap-2">
          <button onclick={() => settings.setTheme('dark')} class="flex items-center justify-center gap-2 px-3 py-2.5 rounded-lg text-sm transition-all active:scale-[0.98] {settings.theme === 'dark' ? 'bg-gradient-to-b from-gold to-gold-dark text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}"><Moon size={14} /> <span class="hidden sm:inline">{$t('settings.theme_dark')}</span><span class="sm:hidden">Dark</span></button>
          <button onclick={() => settings.setTheme('light')} class="flex items-center justify-center gap-2 px-3 py-2.5 rounded-lg text-sm transition-all active:scale-[0.98] {settings.theme === 'light' ? 'bg-gradient-to-b from-gold to-gold-dark text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}"><Sun size={14} /> <span class="hidden sm:inline">{$t('settings.theme_light')}</span><span class="sm:hidden">Light</span></button>
          <button onclick={() => settings.setTheme('auto')} class="flex items-center justify-center gap-2 px-3 py-2.5 rounded-lg text-sm transition-all active:scale-[0.98] {settings.theme === 'auto' ? 'bg-gradient-to-b from-gold to-gold-dark text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}"><Monitor size={14} /> <span class="hidden sm:inline">{$t('settings.theme_auto')}</span><span class="sm:hidden">Auto</span></button>
        </div>
      </div>

      <div>
        <label class="block text-sm text-zinc-400 mb-2">{$t('settings.scale')}</label>
        <div class="grid grid-cols-4 gap-2">
          {#each ['100', '125', '150', '200'] as s}
            <button onclick={() => settings.setScale(s as any)} class="px-2 py-2.5 rounded-lg text-xs md:text-sm transition-all active:scale-[0.98] {settings.scale === s ? 'bg-gradient-to-b from-gold to-gold-dark text-ink font-medium' : 'bg-ink-light border border-border text-zinc-300 hover:border-gold/50'}">{$t(`settings.scale_${s}`)}</button>
          {/each}
        </div>
      </div>
    </div>
  </section>

  <!-- API -->
  <section class="bg-surface border border-border rounded-xl p-4 md:p-6">
    <h2 class="font-display text-base md:text-lg font-semibold mb-3 md:mb-4">{$t('settings.section_api')}</h2>

    <div class="space-y-3">
      <label class="block text-sm text-zinc-400">{$t('settings.tmdb_key')}</label>
      <input
        type="password"
        bind:value={tmdbKey}
        placeholder={$t('settings.tmdb_key_placeholder')}
        class="w-full px-3 py-2 bg-ink-light border border-border rounded-lg text-zinc-100 placeholder-zinc-500 focus:border-gold focus:outline-none text-sm"
      />
      <p class="text-xs text-zinc-500">{$t('settings.tmdb_key_hint')}</p>
      <p class="text-xs text-zinc-500">{$t('settings.tmdb_key_tooltip')}</p>
      <div class="flex flex-wrap items-center gap-2">
        <button onclick={saveTmdbKey} class="px-4 py-2 bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.98] text-ink font-medium rounded-lg text-sm transition-all">{$t('settings.save')}</button>
        <button
          onclick={() => openUrl('https://www.themoviedb.org/settings/api')}
          class="text-xs text-gold hover:text-gold-light underline transition-colors"
        >{$t('settings.tmdb_key_get_link')}</button>
      </div>
    </div>
  </section>

  <!-- ДОПОЛНИТЕЛЬНО -->
  <section class="bg-surface border border-border rounded-xl p-4 md:p-6">
    <h2 class="font-display text-base md:text-lg font-semibold mb-3 md:mb-4">{$t('settings.section_advanced')}</h2>

    <div class="space-y-3">
      <div>
        <button
          onclick={refreshMetadata}
          disabled={refreshingMeta}
          class="flex items-center gap-2 w-full px-4 py-2.5 rounded-lg bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold transition-all active:scale-[0.98] text-sm disabled:opacity-50 text-left"
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
          class="flex items-center gap-2 w-full px-4 py-2.5 rounded-lg bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold transition-all active:scale-[0.98] text-sm disabled:opacity-50 text-left"
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

  <!-- РЕЗЕРВНАЯ КОПИЯ -->
  <section class="bg-surface border border-border rounded-xl p-4 md:p-6">
    <h2 class="font-display text-base md:text-lg font-semibold mb-3 md:mb-4">{$t('settings.section_backup')}</h2>

    <div class="space-y-3">
      <div>
        <button
          onclick={exportData}
          disabled={exporting}
          class="flex items-center gap-2 w-full px-4 py-2.5 rounded-lg bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold transition-all active:scale-[0.98] text-sm disabled:opacity-50 text-left"
        >
          <Download size={16} />
          {exporting ? $t('common.loading') : $t('settings.export')}
        </button>
        <p class="text-xs text-zinc-500 mt-1">{$t('settings.export_hint')}</p>
      </div>

      <div>
        <button
          onclick={importData}
          disabled={importing}
          class="flex items-center gap-2 w-full px-4 py-2.5 rounded-lg bg-ink-light border border-border text-zinc-300 hover:border-gold hover:text-gold transition-all active:scale-[0.98] text-sm disabled:opacity-50 text-left"
        >
          <Upload size={16} />
          {importing ? $t('common.loading') : $t('settings.import')}
        </button>
        <p class="text-xs text-zinc-500 mt-1">{$t('settings.import_hint')}</p>
      </div>

      {#if backupMsg}
        <div class="p-3 rounded-lg bg-emerald-950/50 border border-emerald-800 text-sm text-emerald-200">{backupMsg}</div>
      {/if}
    </div>
  </section>

  <!-- ОПАСНАЯ ЗОНА -->
  <section class="bg-surface border border-red-900/50 rounded-xl p-4 md:p-6">
    <h2 class="font-display text-base md:text-lg font-semibold mb-3 md:mb-4 flex items-center gap-2 text-red-400">
      <AlertTriangle size={18} />
      {$t('settings.section_danger')}
    </h2>

    <div class="space-y-3">
      <p class="text-sm text-zinc-400">{$t('settings.reset_description')}</p>
      <button
        onclick={() => showResetModal = true}
        class="w-full md:w-auto px-4 py-2.5 bg-red-900/50 hover:bg-red-800/60 active:scale-[0.98] border border-red-800 text-red-200 rounded-lg text-sm font-medium transition-all"
      >{$t('settings.reset_button')}</button>
    </div>
  </section>
</div>

{#if showResetModal}
  <div class="fixed inset-0 z-50 flex items-end md:items-center justify-center p-0 md:p-5 bg-black/70 backdrop-blur-sm" onclick={(e) => e.target === e.currentTarget && (showResetModal = false)} role="presentation">
    <div class="w-full max-w-md bg-surface border border-red-900/50 rounded-t-2xl md:rounded-xl p-4 md:p-6 pb-[max(1rem,env(safe-area-inset-bottom))] md:pb-6" onclick={(e) => e.stopPropagation()}>
      <h2 class="font-display text-lg md:text-xl font-bold mb-2 text-red-400">{$t('settings.reset_modal_title')}</h2>
      <p class="text-sm text-zinc-400 mb-4">{$t('settings.reset_modal_description')}</p>
      <div class="flex flex-col-reverse sm:flex-row justify-end gap-2">
        <button onclick={() => showResetModal = false} class="px-4 py-2.5 text-sm text-zinc-400 hover:text-zinc-100 transition-colors">{$t('settings.reset_cancel')}</button>
        <button
          onclick={confirmReset}
          disabled={resetting}
          class="px-4 py-2.5 bg-red-700 hover:bg-red-600 active:scale-[0.98] text-white font-medium rounded-lg text-sm transition-all disabled:opacity-50"
        >{resetting ? $t('common.loading') : $t('settings.reset_confirm')}</button>
      </div>
    </div>
  </div>
{/if}