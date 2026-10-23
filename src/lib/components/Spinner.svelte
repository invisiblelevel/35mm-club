<script lang="ts">
  interface Props {
    size?: number;
    class?: string;
  }
  let { size = 24, class: className = '' }: Props = $props();
</script>

<div
  class="rounded-full border-2 border-gold border-t-transparent animate-spin {className}"
  style="width: {size}px; height: {size}px;"
  role="status"
  aria-label="Loading"
></div>