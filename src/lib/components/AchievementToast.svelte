<script lang="ts">
  import { t } from 'svelte-i18n';
  import { Trophy, X } from 'lucide-svelte';

  export type ToastData = {
    code: string;
    title_key: string | null;
    description_key: string | null;
    xp_reward: number;
  };

  interface Props {
    toast: ToastData;
    onClose: () => void;
  }

  let { toast, onClose }: Props = $props();

  $effect(() => {
    // Зависимость от toast.code — если тост меняется, таймер перезапускается
    const code = toast.code;
    void code;
    const timer = setTimeout(() => {
      onClose();
    }, 5000);
    return () => clearTimeout(timer);
  });
</script>

<div class="fixed bottom-6 right-6 z-50 animate-slide-in">
  <div class="bg-[#1a1a1a] border border-gold/40 rounded-xl p-4 shadow-2xl flex items-start gap-4 max-w-sm">
    <div class="w-12 h-12 rounded-full bg-gold/10 border border-gold/30 flex items-center justify-center shrink-0">
      <Trophy size={24} class="text-gold" />
    </div>
    <div class="flex-1 min-w-0">
      <p class="text-xs text-gold font-medium mb-1 uppercase tracking-wider">Ачивка получена!</p>
      <p class="text-sm font-bold text-zinc-100 mb-1 truncate">
        {toast.title_key ? $t(toast.title_key) : toast.code}
      </p>
      <p class="text-xs text-zinc-400 line-clamp-2">
        {toast.description_key ? $t(toast.description_key) : ''}
      </p>
      <p class="text-xs text-gold mt-2 font-medium">+{toast.xp_reward} XP</p>
    </div>
    <button onclick={onClose} class="text-zinc-500 hover:text-zinc-300 transition-colors">
      <X size={16} />
    </button>
  </div>
</div>

<style>
  @keyframes slide-in {
    from { opacity: 0; transform: translateX(100%) translateY(20px); }
    to { opacity: 1; transform: translateX(0) translateY(0); }
  }
  .animate-slide-in { animation: slide-in 0.4s ease-out forwards; }
</style>