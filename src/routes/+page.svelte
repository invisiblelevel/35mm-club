<script lang="ts">
  import { onMount } from 'svelte';
  import { goto } from '$app/navigation';

  onMount(() => {
    goto('/profile', { replaceState: true });
  });
</script>

<div class="text-center text-zinc-500 py-20">Загрузка...</div>