<script lang="ts">
  import { onMount } from "svelte";
  import { Link } from "svelte-routing";
  import { HorizontalDashHeading, HeadingType } from "./index";
  import type { ArticleInterface } from "./types";

  export let articles: ArticleInterface[];
  export let articleCount: number | null = null;

  onMount(async () => {
    const url = new URL("/api/articles", window.location.origin);
    if (articleCount) {
      url.search = (new URLSearchParams({limit: String(articleCount)})).toString();
    }

    const res = await fetch(url);
    articles = await res.json();
  });
</script>

{#if articles}
  {#each articles as article}
    {@const date = new Date(article.metadata.published).toISOString().slice(0, 10)}
    <article>
      <HorizontalDashHeading heading={HeadingType.H3}>
        <Link to="article/{article.metadata.slug}">{article.metadata.title}</Link>
      </HorizontalDashHeading>
      <div>
        <time datetime="{date}">{date}</time>
        ::
        <span>
        {#each article.metadata.tags as tag, i}
          <a href="/tag/{tag}">#{tag}</a>{#if i < (article.metadata.tags.length - 1)},&nbsp;{/if}
        {/each}
        </span>
      </div>
      <p>{article.metadata.excerpt}</p>
    </article>
  {/each}
{/if}

<style lang="scss">
  article {
    margin: 3rem 0;
    font-family: $font-fira-code;

    div, div a {
      color: lighten($color-brand, 12);
      margin: 1rem 0;
    }
  }
</style>
