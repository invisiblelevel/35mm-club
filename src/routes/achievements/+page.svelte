<script lang="ts">
  import { t } from 'svelte-i18n';
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { Trophy, Lock, Star, ChevronLeft, ChevronRight } from 'lucide-svelte';
  import PageHeader from '$lib/components/PageHeader.svelte';
  import AchievementIcon from '$lib/components/AchievementIcon.svelte';
  import EmptyState from '$lib/components/EmptyState.svelte';

  type Achievement = {
    id: number;
    code: string;
    category: string;
    icon: string | null;
    xp_reward: number;
    scope: string;
    tier: string;
    is_secret: boolean;
    title_key: string | null;
    description_key: string | null;
    unlocked: boolean;
    unlocked_at: string | null;
  };

  let achievements: Achievement[] = $state([]);
  let loading = $state(true);
  let hoveredAch: Achievement | null = $state(null);
  let currentPage = $state(1);
  let archivePage = $state(1);
  const pageSize = 30;
  let tooltipStyle = $state('');

  const tierColors: Record<string, { bg: string; text: string; border: string; label: string }> = {
    common:    { bg: 'bg-zinc-700/50', text: 'text-zinc-300', border: 'border-zinc-600', label: 'Common' },
    rare:      { bg: 'bg-blue-900/50', text: 'text-blue-300', border: 'border-blue-700', label: 'Rare' },
    epic:      { bg: 'bg-purple-900/50', text: 'text-purple-300', border: 'border-purple-700', label: 'Epic' },
    legendary: { bg: 'bg-amber-950/80', text: 'text-amber-300', border: 'border-amber-600', label: 'Legendary' },
  };

  onMount(() => {
    (async () => {
      try {
        achievements = await invoke<Achievement[]>('get_achievements');
      } catch (e) {
        console.error(e);
      } finally {
        loading = false;
      }
    })();

    const handler = (e: MouseEvent) => {
      const target = e.target as HTMLElement;
      if (!target.closest('[data-ach-card]')) hideTooltip();
    };
    document.addEventListener('click', handler);
    return () => document.removeEventListener('click', handler);
  });

  const unlocked = $derived(achievements.filter(a => a.unlocked));
  const locked = $derived(achievements.filter(a => !a.unlocked));

  const totalPages = $derived(Math.max(1, Math.ceil(unlocked.length / pageSize)));
  const paginatedUnlocked = $derived(unlocked.slice((currentPage - 1) * pageSize, currentPage * pageSize));

  const archiveTotalPages = $derived(Math.max(1, Math.ceil(locked.length / pageSize)));
  const paginatedLocked = $derived(locked.slice((archivePage - 1) * pageSize, archivePage * pageSize));

  function showTooltip(e: MouseEvent, ach: Achievement) {
    if (hoveredAch?.id === ach.id) {
      hideTooltip();
      return;
    }
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    const tooltipWidth = 288;
    const padding = 8;
    let left = rect.left + rect.width / 2 - tooltipWidth / 2;
    left = Math.max(padding, Math.min(left, window.innerWidth - tooltipWidth - padding));
    const isMobile = window.innerWidth < 768;
    const top = isMobile ? rect.bottom + padding : rect.top - padding;
    const transform = isMobile ? 'translateY(0)' : 'translateY(-100%)';
    tooltipStyle = `top: ${top}px; left: ${left}px; transform: ${transform};`;
    hoveredAch = ach;
  }

  function hideTooltip() {
    hoveredAch = null;
  }
</script>

<PageHeader
  title={$t('achievements.title')}
  subtitle={$t('achievements.subtitle')}
  icon={Trophy}
/>

{#if loading}
  <div class="text-center text-zinc-500 py-12">{$t('common.loading')}</div>
{:else if achievements.length === 0}
  <EmptyState
    icon={Trophy}
    title={$t('achievements.empty')}
    hint={$t('achievements.empty_hint')}
  />
{:else}
  <div class="space-y-6 md:space-y-8">
    {#if unlocked.length > 0}
      <section>
        <h2 class="font-display text-lg md:text-xl font-semibold mb-3 md:mb-4 flex items-center gap-2">
          <Star size={18} class="text-gold" />
          {$t('achievements.showcase')}
          <span class="text-sm font-normal text-zinc-500">({unlocked.length})</span>
        </h2>
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-2 md:gap-4">
          {#each paginatedUnlocked as ach}
            {@const tier = tierColors[ach.tier] ?? tierColors.common}
            {@const isAnime = ach.category === 'anime'}
            {@const isLegendary = ach.tier === 'legendary'}
            {#if isLegendary}
              <div
                data-ach-card
                class="group relative rounded-2xl md:rounded-3xl p-2 md:p-4 text-center overflow-hidden transition-all duration-300 hover:-translate-y-1 cursor-pointer"
                style="
                  background: linear-gradient(135deg, rgba(120, 53, 15, 0.8), rgba(161, 98, 7, 0.8));
                  border: 2px solid rgba(251, 191, 36, 0.8);
                  box-shadow: 0 0 25px rgba(251, 191, 36, 0.4), inset 0 0 15px rgba(251, 191, 36, 0.2);
                "
                onclick={(e) => showTooltip(e, ach)}
                onmouseenter={(e) => { if (window.innerWidth >= 768) showTooltip(e, ach); }}
                onmouseleave={() => { if (window.innerWidth >= 768) hideTooltip(); }}
              >
                <div class="absolute inset-0 pointer-events-none animate-pulse"
                     style="background: radial-gradient(circle at center, rgba(251,191,36,0.15) 0%, transparent 70%);"></div>

                <div class="absolute top-1 md:top-2 right-1 md:right-2 px-1 md:px-2 py-0.5 rounded text-[8px] md:text-[9px] font-bold uppercase tracking-wider border"
                     style="background: rgba(120, 53, 15, 0.8); color: #fde68a; border-color: rgba(251, 191, 36, 0.8);">
                  Legendary
                </div>

                <div class="relative mx-auto mb-1.5 md:mb-3 flex justify-center transition-transform duration-300 group-hover:scale-110">
                  <div class="md:hidden"><AchievementIcon code={ach.code} size={56} /></div>
                  <div class="hidden md:block"><AchievementIcon code={ach.code} size={90} /></div>
                </div>
                <p class="relative text-[11px] md:text-sm font-bold text-yellow-100 mb-0.5 md:mb-1 line-clamp-2 md:line-clamp-none">
                  {ach.title_key ? $t(ach.title_key) : ach.code}
                </p>
                <p class="relative text-[10px] md:text-xs text-yellow-200/80 line-clamp-2 md:line-clamp-3">
                  {ach.description_key ? $t(ach.description_key) : ''}
                </p>
              </div>
            {:else if isAnime}
              <div
                data-ach-card
                class="group relative rounded-2xl md:rounded-3xl p-2 md:p-4 text-center overflow-hidden transition-all duration-300 hover:-translate-y-1 cursor-pointer"
                style="
                  background: linear-gradient(135deg, rgba(76, 5, 110, 0.6), rgba(131, 24, 67, 0.6));
                  border: 1px solid rgba(192, 132, 252, 0.4);
                  box-shadow: 0 0 12px rgba(192, 132, 252, 0.15);
                "
                onclick={(e) => showTooltip(e, ach)}
                onmouseenter={(e) => { if (window.innerWidth >= 768) showTooltip(e, ach); }}
                onmouseleave={() => { if (window.innerWidth >= 768) hideTooltip(); }}
              >
                <div class="absolute top-1 md:top-2 right-1 md:right-2 px-1 md:px-1.5 py-0.5 rounded text-[8px] md:text-[9px] font-semibold uppercase tracking-wider border"
                     style="background: rgba(88, 28, 135, 0.5); color: #d8b4fe; border-color: rgba(168, 85, 247, 0.6);">
                  {tier.label}
                </div>

                <div class="relative mx-auto mb-1.5 md:mb-3 flex justify-center transition-transform duration-300 group-hover:scale-105">
                  <div class="md:hidden"><AchievementIcon code={ach.code} size={56} /></div>
                  <div class="hidden md:block"><AchievementIcon code={ach.code} size={90} /></div>
                </div>
                <p class="relative text-[11px] md:text-sm font-medium text-zinc-100 mb-0.5 md:mb-1 line-clamp-2 md:line-clamp-none">
                  {ach.title_key ? $t(ach.title_key) : ach.code}
                </p>
                <p class="relative text-[10px] md:text-xs text-zinc-400 line-clamp-2 md:line-clamp-3">
                  {ach.description_key ? $t(ach.description_key) : ''}
                </p>
              </div>
            {:else}
              <div
                data-ach-card
                class="group relative bg-surface border border-gold/40 rounded-xl p-2 md:p-4 text-center overflow-hidden transition-all duration-300 hover:-translate-y-1 hover:border-gold/70 cursor-pointer"
                onclick={(e) => showTooltip(e, ach)}
                onmouseenter={(e) => { if (window.innerWidth >= 768) showTooltip(e, ach); }}
                onmouseleave={() => { if (window.innerWidth >= 768) hideTooltip(); }}
              >
                <div class="absolute top-1 md:top-2 right-1 md:right-2 px-1 md:px-1.5 py-0.5 rounded text-[8px] md:text-[9px] font-semibold uppercase tracking-wider border {tier.bg} {tier.text} {tier.border}">
                  {tier.label}
                </div>

                <div class="relative mx-auto mb-1.5 md:mb-3 flex justify-center transition-transform duration-300 group-hover:scale-105">
                  <div class="md:hidden"><AchievementIcon code={ach.code} size={56} /></div>
                  <div class="hidden md:block"><AchievementIcon code={ach.code} size={90} /></div>
                </div>
                <p class="relative text-[11px] md:text-sm font-medium text-zinc-100 mb-0.5 md:mb-1 line-clamp-2 md:line-clamp-none">
                  {ach.title_key ? $t(ach.title_key) : ach.code}
                </p>
                <p class="relative text-[10px] md:text-xs text-zinc-500 line-clamp-2 md:line-clamp-3">
                  {ach.description_key ? $t(ach.description_key) : ''}
                </p>
              </div>
            {/if}
          {/each}
        </div>

        {#if totalPages > 1}
          <div class="flex items-center justify-center gap-1.5 md:gap-2 mt-6">
            <button
              onclick={() => currentPage = Math.max(1, currentPage - 1)}
              disabled={currentPage === 1}
              class="w-9 h-9 md:w-10 md:h-10 rounded-lg bg-surface border border-border text-zinc-400 hover:text-gold hover:border-gold/50 transition-colors disabled:opacity-30 disabled:cursor-not-allowed flex items-center justify-center"
            >
              <ChevronLeft size={16} />
            </button>
            {#each Array(totalPages) as _, i}
              {@const page = i + 1}
              <button
                onclick={() => currentPage = page}
                class="w-9 h-9 md:w-10 md:h-10 rounded-lg text-sm font-medium transition-colors {currentPage === page ? 'bg-gold text-ink' : 'bg-surface border border-border text-zinc-400 hover:text-gold hover:border-gold/50'}"
              >{page}</button>
            {/each}
            <button
              onclick={() => currentPage = Math.min(totalPages, currentPage + 1)}
              disabled={currentPage === totalPages}
              class="w-9 h-9 md:w-10 md:h-10 rounded-lg bg-surface border border-border text-zinc-400 hover:text-gold hover:border-gold/50 transition-colors disabled:opacity-30 disabled:cursor-not-allowed flex items-center justify-center"
            >
              <ChevronRight size={16} />
            </button>
          </div>
        {/if}
      </section>
    {/if}

    {#if locked.length > 0}
      <section>
        <h2 class="font-display text-lg md:text-xl font-semibold mb-3 md:mb-4 flex items-center gap-2">
          <Lock size={18} class="text-zinc-500" />
          {$t('achievements.archive')}
          <span class="text-sm font-normal text-zinc-500">({locked.length})</span>
        </h2>
        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-6 gap-2 md:gap-4">
          {#each paginatedLocked as ach}
            {@const tier = tierColors[ach.tier] ?? tierColors.common}
            {@const isAnime = ach.category === 'anime'}
            {#if isAnime}
              <div class="relative rounded-2xl md:rounded-3xl p-2 md:p-4 text-center opacity-60 grayscale overflow-hidden"
                   style="background: linear-gradient(135deg, rgba(76, 5, 110, 0.6), rgba(131, 24, 67, 0.6)); border: 1px solid rgba(192, 132, 252, 0.4);">
                <div class="absolute top-1 md:top-2 right-1 md:right-2 px-1 md:px-1.5 py-0.5 rounded text-[8px] md:text-[9px] font-semibold uppercase tracking-wider border"
                     style="background: rgba(88, 28, 135, 0.5); color: #d8b4fe; border-color: rgba(168, 85, 247, 0.6);">
                  {tier.label}
                </div>
                <div class="mx-auto mb-1.5 md:mb-3 flex justify-center">
                  <div class="md:hidden"><AchievementIcon code={ach.code} size={56} /></div>
                  <div class="hidden md:block"><AchievementIcon code={ach.code} size={90} /></div>
                </div>
                <p class="text-[11px] md:text-sm font-medium text-zinc-400 mb-0.5 md:mb-1 line-clamp-2 md:line-clamp-none">
                  {ach.is_secret ? $t('achievements.secret') : (ach.title_key ? $t(ach.title_key) : ach.code)}
                </p>
              </div>
            {:else}
              <div class="relative bg-surface border border-border rounded-xl p-2 md:p-4 text-center opacity-60 grayscale overflow-hidden">
                <div class="absolute top-1 md:top-2 right-1 md:right-2 px-1 md:px-1.5 py-0.5 rounded text-[8px] md:text-[9px] font-semibold uppercase tracking-wider border {tier.bg} {tier.text} {tier.border}">
                  {tier.label}
                </div>
                <div class="mx-auto mb-1.5 md:mb-3 flex justify-center">
                  <div class="md:hidden"><AchievementIcon code={ach.code} size={56} /></div>
                  <div class="hidden md:block"><AchievementIcon code={ach.code} size={90} /></div>
                </div>
                <p class="text-[11px] md:text-sm font-medium text-zinc-400 mb-0.5 md:mb-1 line-clamp-2 md:line-clamp-none">
                  {ach.is_secret ? $t('achievements.secret') : (ach.title_key ? $t(ach.title_key) : ach.code)}
                </p>
              </div>
            {/if}
          {/each}
        </div>

        {#if archiveTotalPages > 1}
          <div class="flex items-center justify-center gap-1.5 md:gap-2 mt-6">
            <button
              onclick={() => archivePage = Math.max(1, archivePage - 1)}
              disabled={archivePage === 1}
              class="w-9 h-9 md:w-10 md:h-10 rounded-lg bg-surface border border-border text-zinc-400 hover:text-gold hover:border-gold/50 transition-colors disabled:opacity-30 disabled:cursor-not-allowed flex items-center justify-center"
            >
              <ChevronLeft size={16} />
            </button>
            {#each Array(archiveTotalPages) as _, i}
              {@const page = i + 1}
              <button
                onclick={() => archivePage = page}
                class="w-9 h-9 md:w-10 md:h-10 rounded-lg text-sm font-medium transition-colors {archivePage === page ? 'bg-gold text-ink' : 'bg-surface border border-border text-zinc-400 hover:text-gold hover:border-gold/50'}"
              >{page}</button>
            {/each}
            <button
              onclick={() => archivePage = Math.min(archiveTotalPages, archivePage + 1)}
              disabled={archivePage === archiveTotalPages}
              class="w-9 h-9 md:w-10 md:h-10 rounded-lg bg-surface border border-border text-zinc-400 hover:text-gold hover:border-gold/50 transition-colors disabled:opacity-30 disabled:cursor-not-allowed flex items-center justify-center"
            >
              <ChevronRight size={16} />
            </button>
          </div>
        {/if}
      </section>
    {/if}
  </div>
{/if}

{#if hoveredAch}
  <div
    class="fixed z-[100] w-64 md:w-72 p-3 rounded-lg bg-ink-light border border-border shadow-xl pointer-events-none"
    style={tooltipStyle}
  >
    <p class="text-sm font-bold text-zinc-100 mb-1">
      {hoveredAch.title_key ? $t(hoveredAch.title_key) : hoveredAch.code}
    </p>
    <p class="text-xs text-zinc-400 leading-relaxed">
      {hoveredAch.description_key ? $t(hoveredAch.description_key) : ''}
    </p>
    <p class="text-[10px] text-gold mt-1 font-medium">+{hoveredAch.xp_reward} XP</p>
    {#if hoveredAch.unlocked_at}
      <p class="text-[10px] text-zinc-500 mt-1">
        {$t('achievements.unlocked_at', { values: { date: hoveredAch.unlocked_at } })}
      </p>
    {/if}
  </div>
{/if}