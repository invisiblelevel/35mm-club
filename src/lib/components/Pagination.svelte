<script lang="ts">
  interface Props {
    currentPage: number;
    totalPages: number;
    onPageChange: (page: number) => void;
  }

  let { currentPage, totalPages, onPageChange }: Props = $props();
</script>

{#if totalPages > 1}
  <div class="flex items-center justify-center gap-2 mt-8">
    <button
      onclick={() => onPageChange(Math.max(1, currentPage - 1))}
      disabled={currentPage === 1}
      class="px-3 py-2 rounded-lg bg-surface border border-border text-sm text-zinc-400 hover:text-gold hover:border-gold/50 transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
    >←</button>

    {#each Array(totalPages) as _, i}
      {@const page = i + 1}
      <button
        onclick={() => onPageChange(page)}
        class="w-10 h-10 rounded-lg text-sm font-medium transition-colors {currentPage === page ? 'bg-gold text-ink' : 'bg-surface border border-border text-zinc-400 hover:text-gold hover:border-gold/50'}"
      >{page}</button>
    {/each}

    <button
      onclick={() => onPageChange(Math.min(totalPages, currentPage + 1))}
      disabled={currentPage === totalPages}
      class="px-3 py-2 rounded-lg bg-surface border border-border text-sm text-zinc-400 hover:text-gold hover:border-gold/50 transition-colors disabled:opacity-30 disabled:cursor-not-allowed"
    >→</button>
  </div>
{/if}