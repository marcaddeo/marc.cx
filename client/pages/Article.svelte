<script lang="ts">
  import { Article } from "../components";

  export let slug: string;
  export let article: object | null = null;
</script>

<svelte:head>
{#if article}
  <title>{article.metadata.title} | marc.cx</title>
{/if}
</svelte:head>

<Article bind:article={article} {slug} />

<style lang="scss">
</style>
