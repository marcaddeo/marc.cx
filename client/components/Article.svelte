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

  article {
    padding: 2rem 0 4rem 0;
    font-size: 1.1rem;

    :global {
      h1,
      h2,
      h3,
      h4,
      h5,
      h6 {
        color: $color-brand;
        text-align: justify;
        margin: 1rem 0;
      }

      p {
        text-align: justify;
        margin: 0.8rem 0;
      }

      code {
        background-color: $color-subtext;
        padding: 0.3rem 0.4rem;
      }
    }
  }
</style>
