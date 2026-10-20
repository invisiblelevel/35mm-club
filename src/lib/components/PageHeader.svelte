<script lang="ts">
  import type { Component } from 'svelte';
  import type { Icon as LucideIcon } from 'lucide-svelte';

  interface Props {
    title: string;
    subtitle?: string;
    icon?: typeof LucideIcon;
    actionLabel?: string;
    actionHref?: string;
    onAction?: () => void;
  }

  let { title, subtitle, icon: Icon, actionLabel, actionHref, onAction }: Props = $props();
</script>

<div class="mb-6 md:mb-8">
  <div class="flex items-start justify-between gap-3 flex-wrap">
    <div class="flex items-start gap-3 md:gap-4 min-w-0 flex-1">
      {#if Icon}
        <div class="w-10 h-10 md:w-12 md:h-12 rounded-xl bg-surface border border-border flex items-center justify-center shrink-0">
          <Icon size={20} class="text-gold md:hidden" />
          <Icon size={24} class="text-gold hidden md:block" />
        </div>
      {/if}
      <div class="min-w-0 flex-1">
        <h1 class="font-display text-2xl md:text-3xl font-bold tracking-tight truncate">{title}</h1>
        {#if subtitle}
          <p class="text-sm md:text-base text-zinc-500 mt-1 line-clamp-2 md:line-clamp-none">{subtitle}</p>
        {/if}
      </div>
    </div>

    {#if actionLabel}
      {#if actionHref}
        <a
          href={actionHref}
          class="inline-flex items-center justify-center gap-2 w-full sm:w-auto px-4 py-2.5 bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.98] text-ink text-sm font-medium rounded-lg transition-all shrink-0"
        >{actionLabel}</a>
      {:else}
        <button
          onclick={onAction}
          class="inline-flex items-center justify-center gap-2 w-full sm:w-auto px-4 py-2.5 bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.98] text-ink text-sm font-medium rounded-lg transition-all shrink-0"
        >{actionLabel}</button>
      {/if}
    {/if}
  </div>
</div>