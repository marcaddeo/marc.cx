<script lang="ts">
  import { onMount } from "svelte";

  export let slug: string;
  export let article: object | null = null;

  onMount(async () => {
    const res = await fetch(`/api/article/${slug}`);
    article = await res.json();
  });
</script>

<article>
{#if article}
  {@html article.html}
{/if}
</article>

<style lang="scss">
</style>
