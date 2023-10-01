<script lang="ts">
  import { Utterances } from "@codewithshin/svelte-utterances";
  import { Article } from "../components";
  import type { ArticleInterface } from "../components";
  import NotFound from "./NotFound.svelte";

  export let slug: string;
  export let article: ArticleInterface | null = null;

  $: component = article && article?.error ? NotFound : Article;
</script>

<svelte:head>
  {#if article?.metadata}
    <title>{article.metadata.title} | marc.cx</title>
  {/if}
</svelte:head>

<svelte:component this={component} {slug} bind:article />

{#if typeof component === typeof ArticleInterface}
  <Utterances reponame="marcaddeo/marc.cx" />
{/if}

<style lang="scss">
</style>
