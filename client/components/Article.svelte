<script lang="ts">
  import { onMount } from "svelte";
  import type { IArticle } from "./types";

  export let slug: string;
  export let article: IArticle | null = null;

  onMount(async () => {
    const res = await fetch(`/api/article/${slug}`, {
      headers: {
        "Accept": "application/json",
      },
    });

    article = await res.json();
  });
</script>

<article>
{#if article?.html}
  {@html article.html}
{/if}
</article>

<style lang="scss">
</style>
