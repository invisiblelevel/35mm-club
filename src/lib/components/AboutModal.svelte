<script lang="ts">
  import { t } from 'svelte-i18n';
  import { X, Heart } from 'lucide-svelte';
  import { openUrl } from '@tauri-apps/plugin-opener';

  interface Props {
    onClose: () => void;
    onOpenSupport: () => void;
  }

  let { onClose, onOpenSupport }: Props = $props();

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) onClose();
  }
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && onClose()} />

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-5 bg-black/70 backdrop-blur-sm"
  onclick={handleBackdrop}
  role="presentation"
>
  <div
    class="relative w-full max-w-md rounded-xl bg-surface border border-border shadow-2xl"
    role="dialog"
    aria-modal="true"
    onclick={(e) => e.stopPropagation()}
  >
    <button
      onclick={onClose}
      class="absolute top-3 right-3 w-8 h-8 rounded-md flex items-center justify-center text-zinc-500 hover:bg-ink-light hover:text-zinc-100 transition-colors"
      aria-label={$t('about.close')}
    >
      <X size={16} />
    </button>

    <div class="p-7 pb-6">
      <h1 class="font-display text-2xl font-bold tracking-tight mb-1">
        35<span class="text-gold">mm</span> Club
      </h1>
      <p class="text-xs text-zinc-500 mb-5">
        {$t('about.version', { values: { v: '1.2.0' } })}
      </p>

      <p class="text-sm leading-relaxed text-zinc-400">
        {$t('about.description')}
      </p>

      <div class="h-px bg-border my-5"></div>

      <button
        onclick={onOpenSupport}
        class="w-full flex items-center justify-center gap-2 px-4 py-3 bg-ink-light border border-border rounded-lg text-sm font-medium text-zinc-300 hover:border-gold hover:text-gold transition-colors cursor-pointer"
      >
        <Heart size={16} />
        {$t('about.support')}
      </button>

      <div class="h-px bg-border my-5"></div>

      <div class="text-center">
	  <p class="text-[10px] text-zinc-600 mt-2">
  Achievement icons:
  <button
    type="button"
    onclick={() => openUrl('https://game-icons.net/')}
    class="underline hover:text-zinc-400"
  >Game Icons</button>
  (CC BY 3.0)
</p>
        <p class="text-[11px] text-zinc-500 mb-1">{$t('about.license')}</p>
        <p class="text-[11px] text-zinc-500">{$t('about.copyright')}</p>
      </div>
    </div>
  </div>
</div>