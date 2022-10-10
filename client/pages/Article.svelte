<script lang="ts">
  import { Article } from "../components";
  import NotFound from "./NotFound.svelte";

  export let slug: string;
  export let article: object | null = null;

  $: component = (article && article?.not_found) ? NotFound : Article;
</script>

<svelte:head>
{#if article?.metadata}
  <title>{article.metadata.title} | marc.cx</title>
{/if}
</svelte:head>

<svelte:component this={component} {slug} bind:article={article} />

<style lang="scss">
</style>
