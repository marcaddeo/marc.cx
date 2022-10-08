<script lang="ts">
  import ArticleList from "../components/ArticleList.svelte";

  export let articles: object | null = null;
</script>

<ArticleList {articles} />

<style lang="scss">
</style>
