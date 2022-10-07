<script context="module">
  export const hydrate = false;
</script>

<script lang="ts">
  import { onMount } from "svelte";

  export let slug: string;
  export let content: string | null = null;

  onMount(async () => {
    const res = await fetch("/api/article");
    content = await res.text();
  });
</script>

<article>
  {@html content}
</article>

<style lang="scss">
</style>
