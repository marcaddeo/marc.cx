<script lang="ts">
  import { onMount } from "svelte";

  export let slug: string;
  export let article: object | null = null;

  onMount(async () => {
    const res = await fetch(`/api/article/${slug}`, {
      headers: {
        "Accept": "application/json",
      },
    });
    if (res.ok) {
      article = await res.json();
    } else {
      article = {"not_found": true};
    }
  });
</script>

<article>
{#if article}
  {@html article.html}
{/if}
</article>

<style lang="scss">
</style>
