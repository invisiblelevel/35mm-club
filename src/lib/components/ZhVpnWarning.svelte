<script lang="ts">
  import { t } from 'svelte-i18n';
  import { Globe, X } from 'lucide-svelte';

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  function dismiss() {
    localStorage.setItem('35mm-zh-vpn-warned', '1');
    onClose();
  }

  function onKeydown(e: KeyboardEvent) {
    if (e.key === 'Escape') dismiss();
  }
</script>

<svelte:window onkeydown={onKeydown} />

<div
  class="fixed inset-0 z-[100] flex items-center justify-center p-4 bg-black/70 backdrop-blur-sm"
  role="dialog"
  aria-modal="true"
  tabindex="-1"
  onclick={(e) => { if (e.target === e.currentTarget) dismiss(); }}
>
  <div class="w-full max-w-md rounded-2xl bg-surface border border-border shadow-2xl shadow-black/60 overflow-hidden">
    <div class="relative p-6 pb-4 border-b border-border">
      <button
        onclick={dismiss}
        class="absolute top-4 right-4 w-8 h-8 flex items-center justify-center rounded-lg text-zinc-500 hover:text-zinc-100 hover:bg-ink-light transition-colors"
        aria-label={$t('common.close')}
      >
        <X size={18} />
      </button>

      <div class="flex items-center gap-3">
        <div class="w-11 h-11 rounded-xl bg-gradient-to-b from-gold to-gold-dark flex items-center justify-center shrink-0">
          <Globe size={22} class="text-ink" />
        </div>
        <h2 class="font-display text-lg font-bold text-zinc-100">
          {$t('zh_vpn_warning.title')}
        </h2>
      </div>
    </div>

    <div class="p-6 space-y-3">
      <p class="text-sm text-zinc-300 leading-relaxed">
        {$t('zh_vpn_warning.body')}
      </p>
      <p class="text-xs text-zinc-500 leading-relaxed">
        {$t('zh_vpn_warning.hint')}
      </p>
    </div>

    <div class="p-4 pt-0">
      <button
        onclick={dismiss}
        class="w-full py-2.5 rounded-lg bg-gradient-to-b from-gold to-gold-dark hover:brightness-110 active:scale-[0.98] text-ink text-sm font-medium transition-all"
      >
        {$t('zh_vpn_warning.button')}
      </button>
    </div>
  </div>
</div>