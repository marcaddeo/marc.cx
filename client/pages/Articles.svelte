<script lang="ts">
  import { ArticleList, SpecialHeading } from "../components";

  export let articles: object | null = null;
</script>

<svelte:head>
  <title>Articles | marc.cx</title>
</svelte:head>

<SpecialHeading>Articles</SpecialHeading>

<ArticleList {articles} />

<style lang="scss">
  // @todo need a bunch of margin/padding up top.
</style>
