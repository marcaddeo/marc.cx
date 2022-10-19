<script lang="ts">
  import { onMount } from "svelte";
  import type { ArticleInterface } from "./types";

  export let slug: string;
  export let article: ArticleInterface | null = null;

  onMount(async () => {
    const res = await fetch(`/api/article/${slug}`, {
      headers: {
        Accept: "application/json",
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
  :global {
    pre code {
      border-radius: 15px;
    }

    noscript pre code {
      display: block;
      overflow-x: auto;
      padding: 1rem;
      background-color: $color-noscript-code-bg;
      color: $color-noscript-code-text;
      font-family: $font-fira-code;
    }
  }
</style>
