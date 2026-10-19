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

<div class="flex items-start justify-between mb-8">
  <div class="flex items-start gap-4">
    {#if Icon}
      <div class="w-12 h-12 rounded-xl bg-surface border border-border flex items-center justify-center shrink-0">
        <Icon size={24} class="text-gold" />
      </div>
    {/if}
    <div>
      <h1 class="font-display text-3xl font-bold tracking-tight">{title}</h1>
      {#if subtitle}
        <p class="text-zinc-500 mt-1">{subtitle}</p>
      {/if}
    </div>
  </div>

  {#if actionLabel}
    {#if actionHref}
      <a
        href={actionHref}
        class="inline-flex items-center gap-2 px-4 py-2 bg-gold hover:bg-gold-light text-ink text-sm font-medium rounded-lg transition-colors"
      >{actionLabel}</a>
    {:else}
      <button
        onclick={onAction}
        class="inline-flex items-center gap-2 px-4 py-2 bg-gold hover:bg-gold-light text-ink text-sm font-medium rounded-lg transition-colors"
      >{actionLabel}</button>
    {/if}
  {/if}
</div>