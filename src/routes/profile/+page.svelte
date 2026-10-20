<script lang="ts">
  import { t } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke, convertFileSrc } from '@tauri-apps/api/core';
  import { open } from '@tauri-apps/plugin-dialog';
  import { readFile } from '@tauri-apps/plugin-fs';
  import { User, Trophy, Film, Tv, Clapperboard, Camera, Pencil, Check, X } from 'lucide-svelte';
  import AchievementIcon from '$lib/components/AchievementIcon.svelte';

  type UserProfile = {
    username: string;
    avatar_path: string | null;
    remote_id: string | null;
    level: number;
    xp: number;
    total_watch_minutes: number;
    created_at: string;
    xp_to_next_level: number;
    xp_in_current_level: number;
    rank_name: string;
  };

  type LibraryItem = { id: number; status: string; };

  type Achievement = {
    id: number; code: string; title_key: string | null;
    description_key: string | null; xp_reward: number;
    tier: string; category: string;
    unlocked: boolean; unlocked_at: string | null;
  };

  let profile: UserProfile | null = $state(null);
  let loading = $state(true);
  let activeTab = $state<'movie' | 'tv' | 'anime'>('movie');
  let counts = $state({ movie: 0, tv: 0, anime: 0 });
  let completed = $state({ movie: 0, tv: 0, anime: 0 });
  let recentAchievements: Achievement[] = $state([]);
  let hoveredAch: Achievement | null = $state(null);
  let uploading = $state(false);
  let profileTooltipStyle = $state('');

  let editingName = $state(false);
  let nameInput = $state('');

  const tierColors: Record<string, { bg: string; text: string; border: string; label: string }> = {
    common:    { bg: 'bg-zinc-700/50', text: 'text-zinc-300', border: 'border-zinc-600', label: 'Common' },
    rare:      { bg: 'bg-blue-900/50', text: 'text-blue-300', border: 'border-blue-700', label: 'Rare' },
    epic:      { bg: 'bg-purple-900/50', text: 'text-purple-300', border: 'border-purple-700', label: 'Epic' },
    legendary: { bg: 'bg-amber-950/80', text: 'text-amber-300', border: 'border-amber-600', label: 'Legendary' },
  };

  const categoryColors: Record<string, string> = {
    progress: '#fbbf24',
    movie: '#f59e0b',
    tv: '#38bdf8',
    anime: '#c084fc',
    humor: '#84cc16',
    franchise: '#ef4444',
    director: '#fbbf24',
    actor: '#fda4af',
    time: '#a8a29e',
  };

  let starCount = $derived(
    profile
      ? (profile.level >= 46 ? 4 : profile.level >= 34 ? 3 : profile.level >= 27 ? 2 : profile.level >= 20 ? 1 : 0)
      : 0
  );

  let ribbonSegments = $derived(
    recentAchievements.slice(0, 8).map(a => categoryColors[a.category] || '#666')
  );

  const totalXpForLevel = (level: number) => 100 * level + 50 * level * level;
  let xpStart = $derived(profile ? (profile.level === 1 ? 0 : totalXpForLevel(profile.level - 1)) : 0);
  let xpEnd = $derived(profile ? totalXpForLevel(profile.level) : 1);
  let xpProgressPercent = $derived(profile ? Math.min(100, ((profile.xp - xpStart) / (xpEnd - xpStart)) * 100) : 0);

  async function loadProfile() {
    try {
      profile = await invoke<UserProfile>('get_profile');
      const movies = await invoke<LibraryItem[]>('get_library', { contentType: 'movie' });
      const tv = await invoke<LibraryItem[]>('get_library', { contentType: 'tv' });
      const anime = await invoke<LibraryItem[]>('get_library', { contentType: 'anime' });

      counts = { movie: movies.length, tv: tv.length, anime: anime.length };
      completed = {
        movie: movies.filter((m) => m.status === 'completed').length,
        tv: tv.filter((m) => m.status === 'completed').length,
        anime: anime.filter((m) => m.status === 'completed').length,
      };

      const allAch = await invoke<Achievement[]>('get_achievements');
      recentAchievements = allAch.filter(a => a.unlocked).slice(0, 12);
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    await loadProfile();
    const handler = () => loadProfile();
    window.addEventListener('35mm-data-changed', handler);
    return () => window.removeEventListener('35mm-data-changed', handler);
  });

  async function pickAvatar() {
    if (uploading) return;
    uploading = true;
    try {
      const file = await open({
        multiple: false,
        filters: [{ name: 'Images', extensions: ['png', 'jpg', 'jpeg', 'webp'] }],
      });
      if (!file) { uploading = false; return; }
      const path = typeof file === 'string' ? file : (file as any).path;
      if (!path) { uploading = false; return; }

      const bytes = await readFile(path);
      let binary = '';
      const len = bytes.byteLength;
      for (let i = 0; i < len; i++) {
        binary += String.fromCharCode(bytes[i]);
      }
      const base64 = btoa(binary);

      await invoke<string>('set_avatar', { imageData: base64 });
      await loadProfile();
    } catch (e) {
      alert('Avatar upload failed: ' + String(e));
    } finally {
      uploading = false;
    }
  }

  function startEditName() {
    if (!profile) return;
    nameInput = profile.username;
    editingName = true;
  }

  async function saveName() {
    if (!nameInput.trim()) return;
    try {
      await invoke('update_profile', { username: nameInput.trim() });
      editingName = false;
      await loadProfile();
    } catch (e) {
      alert('Ошибка: ' + e);
    }
  }

  const tabs = [
    { value: 'movie' as const, labelKey: 'profile.tab_movies', icon: Film },
    { value: 'tv' as const, labelKey: 'profile.tab_tv', icon: Tv },
    { value: 'anime' as const, labelKey: 'profile.tab_anime', icon: Clapperboard },
  ];

  function avatarSrc(path: string | null): string | null {
    if (!path) return null;
    try { return convertFileSrc(path); } catch { return null; }
  }

  function showProfileTooltip(e: MouseEvent, ach: Achievement) {
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    const tooltipWidth = 256;
    const padding = 8;
    const isMobile = window.innerWidth < 768;
    if (isMobile) {
      profileTooltipStyle = `top: 50%; left: 50%; transform: translate(-50%, -50%); width: 280px;`;
    } else {
      let left = rect.left + rect.width / 2 - tooltipWidth / 2;
      left = Math.max(padding, Math.min(left, window.innerWidth - tooltipWidth - padding));
      profileTooltipStyle = `top: ${rect.top - padding}px; left: ${left}px; transform: translateY(-100%);`;
    }
    hoveredAch = ach;
  }

  function hideProfileTooltip() {
    hoveredAch = null;
  }
</script>

{#if loading}
  <div class="text-center text-zinc-500 py-20">{$t('common.loading')}</div>
{:else if profile}
  <div class="max-w-5xl mx-auto space-y-6">
    <div class="relative rounded-2xl overflow-hidden bg-surface border border-border p-4 md:p-8">
      <div class="flex flex-col md:flex-row items-center md:items-start gap-4 md:gap-6">
        <button
          type="button"
          onclick={pickAvatar}
          disabled={uploading}
          class="relative w-24 h-24 md:w-28 md:h-28 rounded-full bg-ink-light border-2 border-gold/50 flex items-center justify-center shrink-0 overflow-hidden group disabled:opacity-60 cursor-pointer"
          title={$t('profile.change_avatar')}
        >
          {#if profile.avatar_path && avatarSrc(profile.avatar_path)}
            <img src={avatarSrc(profile.avatar_path)} alt="Avatar" class="w-full h-full object-cover" />
          {:else}
            <User size={48} class="text-gold" />
          {/if}
          <div class="absolute inset-0 bg-black/50 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity">
            <Camera size={24} class="text-white" />
          </div>
        </button>

        <div class="flex-1 w-full min-w-0 text-center md:text-left">
          {#if editingName}
            <div class="flex items-center justify-center md:justify-start gap-2 mb-1">
              <input
                type="text"
                bind:value={nameInput}
                onkeydown={(e) => e.key === 'Enter' && saveName()}
                class="font-display text-2xl md:text-3xl font-bold bg-ink-light border border-border rounded-lg px-3 py-1 text-zinc-100 focus:border-gold focus:outline-none w-full max-w-xs"
                maxlength="32"
              />
              <button onclick={saveName} class="w-8 h-8 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white flex items-center justify-center transition-colors shrink-0">
                <Check size={16} />
              </button>
              <button onclick={() => editingName = false} class="w-8 h-8 rounded-lg bg-surface border border-border text-zinc-400 hover:text-zinc-100 flex items-center justify-center transition-colors shrink-0">
                <X size={16} />
              </button>
            </div>
          {:else}
            <div class="flex items-baseline justify-center md:justify-start gap-2 md:gap-3 mb-1">
              <button
                onclick={startEditName}
                class="flex items-center gap-2 text-left group/edit cursor-pointer min-h-[44px] px-2 -mx-2 rounded-lg active:bg-ink-light/50 md:active:bg-transparent transition-colors"
                title="Изменить имя"
              >
                <h1 class="font-display text-2xl md:text-3xl font-bold truncate">{profile.username}</h1>
                <Pencil size={16} class="text-zinc-500 group-hover/edit:text-gold md:opacity-0 md:group-hover:opacity-100 transition-opacity shrink-0" />
              </button>
              <span class="text-sm text-gold font-medium">Lv. {profile.level}</span>
            </div>
          {/if}

          {#if starCount > 0 && !editingName}
            <div class="flex justify-center md:justify-start mb-2">
              <div class="relative" title="Генеральская кокарда">
                <div
                  class="relative flex h-7 rounded-sm overflow-hidden"
                  style="
                    border: 1px solid #2a2a2a;
                    box-shadow:
                      0 2px 4px rgba(0,0,0,0.6),
                      0 0 8px rgba(255,144,0,0.3),
                      inset 0 1px 1px rgba(255,255,255,0.2),
                      inset 0 -1px 2px rgba(0,0,0,0.4);
                  "
                >
                  {#if ribbonSegments.length === 0}
                    <div class="flex-1" style="background: linear-gradient(180deg, #555 0%, #333 50%, #222 100%);"></div>
                  {:else}
                    {#each ribbonSegments as color}
                      <div style="background: linear-gradient(180deg, {color}ee 0%, {color} 40%, {color}cc 60%, {color}88 100%); width: 14px; box-shadow: inset -1px 0 0 rgba(0,0,0,0.3);"></div>
                    {/each}
                  {/if}
                  <div class="absolute top-0 left-0 right-0 h-1/2 pointer-events-none"
                       style="background: linear-gradient(180deg, rgba(255,255,255,0.35) 0%, rgba(255,255,255,0.05) 100%);"></div>
                </div>
                <div class="absolute inset-0 flex items-center justify-center gap-0.5 pointer-events-none">
                  {#each Array(starCount) as _}
                    <span
                      style="
                        font-size: 19px;
                        line-height: 1;
                        color: #ffd700;
                        text-shadow:
                          0 1px 0 #b8860b,
                          0 2px 3px rgba(0,0,0,0.8),
                          0 0 6px rgba(255,215,0,0.6);
                      "
                    >★</span>
                  {/each}
                </div>
              </div>
            </div>
          {/if}

          <p class="text-zinc-400 mb-3">{$t(profile.rank_name)}</p>

          <div class="mb-1 max-w-md mx-auto md:mx-0">
            <div class="h-2 bg-ink-light rounded-full overflow-hidden">
              <div class="h-full bg-gradient-to-r from-gold to-gold-light transition-all duration-500" style="width: {xpProgressPercent}%"></div>
            </div>
          </div>
          <div class="flex justify-between text-xs text-zinc-500 max-w-md mx-auto md:mx-0">
            <span>{profile.xp} XP</span>
            <span>{profile.xp_to_next_level} XP до Lv. {profile.level + 1}</span>
          </div>
        </div>

        <div class="text-center md:text-right shrink-0">
          <div class="text-2xl font-bold text-gold">{Math.floor(profile.total_watch_minutes / 60)}</div>
          <div class="text-xs text-zinc-500">{$t('profile.total_hours')}</div>
        </div>
      </div>
    </div>

    <div class="flex gap-2 border-b border-border overflow-x-auto">
      {#each tabs as tab}
        <button
          onclick={() => activeTab = tab.value}
          class="flex items-center gap-2 px-4 py-3 text-sm font-medium transition-colors relative whitespace-nowrap {activeTab === tab.value ? 'text-gold' : 'text-zinc-400 hover:text-zinc-100'}"
        >
          <svelte:component this={tab.icon} size={16} />
          {$t(tab.labelKey)}
          {#if activeTab === tab.value}
            <div class="absolute bottom-0 left-0 right-0 h-0.5 bg-gold"></div>
          {/if}
        </button>
      {/each}
    </div>

    <div class="space-y-6">
      <div class="grid grid-cols-3 gap-3 md:gap-4">
        <div class="bg-surface border border-border rounded-xl p-3 md:p-5">
          <div class="text-2xl md:text-3xl font-bold">{counts[activeTab]}</div>
          <div class="text-xs text-zinc-500 mt-1">{$t('profile.stat_count')}</div>
        </div>
        <div class="bg-surface border border-border rounded-xl p-3 md:p-5">
          <div class="text-2xl md:text-3xl font-bold text-gold">{completed[activeTab]}</div>
          <div class="text-xs text-zinc-500 mt-1">{$t('profile.stat_completed')}</div>
        </div>
        <div class="bg-surface border border-border rounded-xl p-3 md:p-5">
          <div class="text-2xl md:text-3xl font-bold">{Math.floor(profile.total_watch_minutes / 60)}</div>
          <div class="text-xs text-zinc-500 mt-1">{$t('profile.stat_hours')}</div>
        </div>
      </div>

      <div class="bg-surface border border-border rounded-xl p-4 md:p-6">
        <h3 class="font-display text-lg font-semibold mb-4 flex items-center gap-2">
          <Trophy size={18} class="text-gold" />
          {$t('profile.recent_achievements')}
        </h3>

        {#if recentAchievements.length === 0}
          <div class="text-center py-6">
            <p class="text-zinc-500 text-sm">{$t('profile.no_achievements')}</p>
            <p class="text-xs text-zinc-600 mt-1">{$t('profile.no_achievements_hint')}</p>
          </div>
        {:else}
          <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-6 gap-2 md:gap-3">
            {#each recentAchievements as ach}
              {@const tier = tierColors[ach.tier] ?? tierColors.common}
              {@const isLegendary = ach.tier === 'legendary'}
              {@const isAnime = ach.category === 'anime'}

              {#if isLegendary}
                <div
                  class="group relative aspect-square rounded-xl overflow-hidden cursor-pointer transition-all duration-300 hover:-translate-y-1 flex items-center justify-center"
                  style="
                    background: linear-gradient(135deg, rgba(120, 53, 15, 0.8), rgba(161, 98, 7, 0.8));
                    border: 1.5px solid rgba(251, 191, 36, 0.8);
                    box-shadow: 0 0 15px rgba(251, 191, 36, 0.3), inset 0 0 10px rgba(251, 191, 36, 0.15);
                  "
                  onmouseenter={(e) => showProfileTooltip(e, ach)}
                  onmouseleave={hideProfileTooltip}
                  onclick={(e) => showProfileTooltip(e, ach)}
                >
                  <div class="absolute inset-0 pointer-events-none animate-pulse"
                       style="background: radial-gradient(circle at center, rgba(251,191,36,0.15) 0%, transparent 70%);"></div>
                  <div class="absolute inset-0 pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                       style="box-shadow: inset 0 0 20px rgba(251,191,36,0.5);"></div>

                  <div class="absolute top-1.5 right-1.5 px-1 py-0.5 rounded text-[7px] md:text-[8px] font-semibold uppercase tracking-wider border"
                       style="background: rgba(120, 53, 15, 0.8); color: #fde68a; border-color: rgba(251, 191, 36, 0.8);">
                    LEGENDARY
                  </div>

                  <div class="relative transition-transform duration-300 group-hover:scale-110 md:hidden">
                    <AchievementIcon code={ach.code} size={64} />
                  </div>
                  <div class="relative transition-transform duration-300 group-hover:scale-110 hidden md:block">
                    <AchievementIcon code={ach.code} size={72} />
                  </div>
                </div>
              {:else if isAnime}
                <div
                  class="group relative aspect-square rounded-xl bg-purple-950/40 border border-purple-800/60 flex items-center justify-center cursor-pointer overflow-hidden transition-all duration-300 hover:-translate-y-1 hover:border-purple-500/80 hover:shadow-[0_0_20px_rgba(168,85,247,0.25)]"
                  onmouseenter={(e) => showProfileTooltip(e, ach)}
                  onmouseleave={hideProfileTooltip}
                  onclick={(e) => showProfileTooltip(e, ach)}
                >
                  <div class="absolute inset-0 pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                       style="background: radial-gradient(circle at center, rgba(168,85,247,0.15) 0%, transparent 70%);"></div>

                  <div class="absolute top-1.5 right-1.5 px-1 py-0.5 rounded text-[7px] md:text-[8px] font-semibold uppercase tracking-wider border bg-purple-900/80 text-purple-200 border-purple-700">
                    ANIME
                  </div>

                  <div class="relative transition-transform duration-300 group-hover:scale-105 md:hidden">
                    <AchievementIcon code={ach.code} size={64} />
                  </div>
                  <div class="relative transition-transform duration-300 group-hover:scale-105 hidden md:block">
                    <AchievementIcon code={ach.code} size={72} />
                  </div>
                </div>
              {:else}
                <div
                  class="group relative aspect-square rounded-xl bg-surface border border-gold/30 flex items-center justify-center cursor-pointer overflow-hidden transition-all duration-300 hover:-translate-y-1 hover:border-gold/70 hover:shadow-[0_0_20px_rgba(255,144,0,0.25)]"
                  onmouseenter={(e) => showProfileTooltip(e, ach)}
                  onmouseleave={hideProfileTooltip}
                  onclick={(e) => showProfileTooltip(e, ach)}
                >
                  <div class="absolute inset-0 pointer-events-none opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                       style="background: radial-gradient(circle at center, rgba(255,144,0,0.08) 0%, transparent 70%);"></div>

                  <div class="absolute top-1.5 right-1.5 px-1 py-0.5 rounded text-[7px] md:text-[8px] font-semibold uppercase tracking-wider border {tier.bg} {tier.text} {tier.border}">
                    {tier.label}
                  </div>

                  <div class="relative transition-transform duration-300 group-hover:scale-105 md:hidden">
                    <AchievementIcon code={ach.code} size={64} />
                  </div>
                  <div class="relative transition-transform duration-300 group-hover:scale-105 hidden md:block">
                    <AchievementIcon code={ach.code} size={72} />
                  </div>
                </div>
              {/if}
            {/each}
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

{#if hoveredAch}
  <div
    class="fixed z-[100] p-3 rounded-lg bg-ink-light border border-border shadow-xl pointer-events-none"
    style={profileTooltipStyle}
  >
    <p class="text-sm font-bold text-zinc-100 mb-1">
      {hoveredAch.title_key ? $t(hoveredAch.title_key) : hoveredAch.code}
    </p>
    <p class="text-xs text-zinc-400 leading-relaxed">
      {hoveredAch.description_key ? $t(hoveredAch.description_key) : ''}
    </p>
    <p class="text-[10px] text-gold mt-1 font-medium">+{hoveredAch.xp_reward} XP</p>
  </div>
{/if}