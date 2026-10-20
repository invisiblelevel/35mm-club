<script lang="ts">
  interface Props {
    currentPage: number;
    totalPages: number;
    onPageChange: (page: number) => void;
  }

  let { currentPage, totalPages, onPageChange }: Props = $props();

  // На мобилке показываем компактно: ← 1 … 5 →
  const visiblePages = $derived.by(() => {
    if (totalPages <= 5) return Array.from({ length: totalPages }, (_, i) => i + 1);
    const pages: (number | string)[] = [];
    if (currentPage <= 3) {
      pages.push(1, 2, 3, 4, '…', totalPages);
    } else if (currentPage >= totalPages - 2) {
      pages.push(1, '…', totalPages - 3, totalPages - 2, totalPages - 1, totalPages);
    } else {
      pages.push(1, '…', currentPage - 1, currentPage, currentPage + 1, '…', totalPages);
    }
    return pages;
  });
</script>

{#if totalPages > 1}
  <div class="flex items-center justify-center gap-1.5 md:gap-2 mt-6 md:mt-8">
    <button
      onclick={() => onPageChange(Math.max(1, currentPage - 1))}
      disabled={currentPage === 1}
      class="w-9 h-9 md:w-10 md:h-10 rounded-lg bg-surface border border-border text-sm text-zinc-400 hover:text-gold hover:border-gold/50 transition-colors disabled:opacity-30 disabled:cursor-not-allowed flex items-center justify-center"
    >←</button>

    {#each visiblePages as page}
      {#if page === '…'}
        <span class="px-1 text-zinc-600">…</span>
      {:else}
        <button
          onclick={() => onPageChange(page as number)}
          class="w-9 h-9 md:w-10 md:h-10 rounded-lg text-sm font-medium transition-colors {currentPage === page ? 'bg-gold text-ink' : 'bg-surface border border-border text-zinc-400 hover:text-gold hover:border-gold/50'}"
        >{page}</button>
      {/if}
    {/each}

    <button
      onclick={() => onPageChange(Math.min(totalPages, currentPage + 1))}
      disabled={currentPage === totalPages}
      class="w-9 h-9 md:w-10 md:h-10 rounded-lg bg-surface border border-border text-sm text-zinc-400 hover:text-gold hover:border-gold/50 transition-colors disabled:opacity-30 disabled:cursor-not-allowed flex items-center justify-center"
    >→</button>
  </div>
{/if}