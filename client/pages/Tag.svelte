<script lang="ts">
  import { ArticleList, VerticalLineHeading } from "../components";
  import type { ArticleInterface } from "../components";
  import Article from "./Article.svelte";
  import NotFound from "./NotFound.svelte";

  export let tag: string | null = null;
  export let articles: ArticleInterface[] | null = null;

  $: component = articles && !articles.length ? NotFound : ArticleList;
</script>

<svelte:head>
  {#if tag}
    <title>#{tag} Articles | marc.cx</title>
  {/if}
</svelte:head>

{#if articles && articles.length}
  <VerticalLineHeading>#{tag} Articles</VerticalLineHeading>
{/if}

<svelte:component this={component} {tag} bind:articles />

<style lang="scss">
  // @todo need a bunch of margin/padding up top.
</style>
