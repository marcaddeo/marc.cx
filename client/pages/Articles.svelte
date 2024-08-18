<script lang="ts">
  import { ArticleList, VerticalLineHeading } from "../components";
  import type { ArticleInterface } from "../components";

  export let articles: ArticleInterface[] | null = null;
</script>

<svelte:head>
  <title>Articles | marc.cx</title>
  <meta name="description" content="Marc Addeo's Blog Articles">
</svelte:head>

<div>
  <VerticalLineHeading>Articles</VerticalLineHeading>
</div>

<ArticleList {articles} />

<style lang="scss">
  div {
    margin: 3rem 0 4rem 0;
  }
</style>
