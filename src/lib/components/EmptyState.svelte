<script lang="ts">
  import type { Icon as LucideIcon } from 'lucide-svelte';

  interface Props {
    icon: typeof LucideIcon;
    title: string;
    hint?: string;
    ctaLabel?: string;
    ctaHref?: string;
  }

  let { icon: Icon, title, hint, ctaLabel, ctaHref }: Props = $props();
</script>

<div class="bg-surface border border-border rounded-xl p-12 text-center">
  <Icon size={48} class="mx-auto text-zinc-600 mb-4" />
  <h2 class="font-display text-xl font-semibold mb-2">{title}</h2>
  {#if hint}
    <p class="text-zinc-500 text-sm mb-6">{hint}</p>
  {/if}
  {#if ctaLabel && ctaHref}
    <a
      href={ctaHref}
      class="inline-flex items-center gap-2 px-5 py-2.5 bg-gold hover:bg-gold-light text-ink font-medium rounded-lg transition-colors"
    >{ctaLabel}</a>
  {/if}
</div>