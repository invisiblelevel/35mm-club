<script lang="ts">
  import { t } from 'svelte-i18n';
  import { X } from 'lucide-svelte';

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  let copiedBtc = $state(false);
  let copiedUsdt = $state(false);

  const BTC = 'bc1q2ka70s4vtmrskandqj8l4d6n3kdxyxa7kf3wf7';
  const USDT = 'TUjY9p6oxKmeCQwNZwMfHqHdXuabaHpgT7';

  function handleBackdrop(e: MouseEvent) {
    if (e.target === e.currentTarget) onClose();
  }

  async function copy(value: string, type: 'btc' | 'usdt') {
    try {
      await navigator.clipboard.writeText(value);
      if (type === 'btc') {
        copiedBtc = true;
        setTimeout(() => (copiedBtc = false), 2000);
      } else {
        copiedUsdt = true;
        setTimeout(() => (copiedUsdt = false), 2000);
      }
    } catch {
      // ignore
    }
  }
</script>

<svelte:window onkeydown={(e) => e.key === 'Escape' && onClose()} />

<div
  class="fixed inset-0 z-50 flex items-center justify-center p-5 bg-black/70 backdrop-blur-sm"
  onclick={handleBackdrop}
  role="presentation"
>
  <div
    class="relative w-full max-w-lg rounded-xl bg-surface border border-border shadow-2xl"
    role="dialog"
    aria-modal="true"
    onclick={(e) => e.stopPropagation()}
  >
    <button
      onclick={onClose}
      class="absolute top-3 right-3 w-8 h-8 rounded-md flex items-center justify-center text-zinc-500 hover:bg-ink-light hover:text-zinc-100 transition-colors"
    >
      <X size={16} />
    </button>

    <div class="p-7 pb-6">
      <h1 class="font-display text-lg font-bold mb-1">
        {$t('support.title')}
      </h1>
      <p class="text-xs text-zinc-500 italic mb-6">
        {$t('support.subtitle')}
      </p>

      <div class="space-y-4">
        <div>
          <div class="text-xs text-zinc-400 mb-1.5">BTC</div>
          <div class="flex gap-2">
            <input
              type="text"
              readonly
              value={BTC}
              class="flex-1 px-3 py-2 rounded-md text-xs font-mono bg-ink-light text-zinc-100 border border-border focus:outline-none"
            />
            <button
              onclick={() => copy(BTC, 'btc')}
              class="px-4 py-2 rounded-md text-xs font-medium bg-gold hover:bg-gold-light text-ink transition-colors"
            >
              {copiedBtc ? $t('support.copied') : $t('support.copy')}
            </button>
          </div>
        </div>

        <div>
          <div class="text-xs text-zinc-400 mb-1.5">USDT (TRC-20)</div>
          <div class="flex gap-2">
            <input
              type="text"
              readonly
              value={USDT}
              class="flex-1 px-3 py-2 rounded-md text-xs font-mono bg-ink-light text-zinc-100 border border-border focus:outline-none"
            />
            <button
              onclick={() => copy(USDT, 'usdt')}
              class="px-4 py-2 rounded-md text-xs font-medium bg-gold hover:bg-gold-light text-ink transition-colors"
            >
              {copiedUsdt ? $t('support.copied') : $t('support.copy')}
            </button>
          </div>
        </div>
      </div>

      <div class="h-px bg-border my-5"></div>

      <p class="text-center text-xs text-zinc-500">
        {$t('support.thanks')}
      </p>
    </div>
  </div>
</div>