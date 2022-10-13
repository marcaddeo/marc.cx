<script lang="ts">
  import { Article } from "../components";
  import type { IArticle } from "../components";
  import NotFound from "./NotFound.svelte";

  export let slug: string;
  export let article: IArticle | null = null;

  $: component = (article && article?.error) ? NotFound : Article;
</script>

<svelte:head>
{#if article?.metadata}
  <title>{article.metadata.title} | marc.cx</title>
{/if}
</svelte:head>

<svelte:component this={component} {slug} bind:article={article} />

<style lang="scss">
</style>
