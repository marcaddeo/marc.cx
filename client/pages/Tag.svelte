<script lang="ts">
  import { ArticleList, VerticalLineHeading } from "../components";;
  import type { ArticleInterface } from "../components";
  import Article from "./Article.svelte";

  export let tag: string | null = null;
  export let articles: ArticleInterface[] | null = null;
</script>

<svelte:head>
{#if tag}
  <title>#{tag} Articles | marc.cx</title>
{/if}
</svelte:head>

{#if tag}
  <VerticalLineHeading>#{tag} Articles</VerticalLineHeading>

  <ArticleList {tag} {articles} />
{/if}

<style lang="scss">
  // @todo need a bunch of margin/padding up top.
</style>
