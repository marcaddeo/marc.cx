<script lang="ts">
  import ArticleList from "../components/ArticleList.svelte";

  export let articles: object | null = null;
</script>

<svelte:head>
  <title>Articles | marc.cx</title>
</svelte:head>

<ArticleList {articles} />

<style lang="scss">
</style>
