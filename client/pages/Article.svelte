<script lang="ts">
  import { Article } from "../components";

  export let slug: string;
  export let article: object | null = null;
</script>

<Article {slug} {article} />

<style lang="scss">
</style>
