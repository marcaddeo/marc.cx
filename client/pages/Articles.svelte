<script lang="ts">
  import { ArticleList, SpecialHeading } from "../components";
  import type { ArticleInterface } from "../components"

  export let articles: ArticleInterface[] | null = null;
</script>

<svelte:head>
  <title>Articles | marc.cx</title>
</svelte:head>

<SpecialHeading>Articles</SpecialHeading>

<ArticleList {articles} />

<style lang="scss">
  // @todo need a bunch of margin/padding up top.
</style>
