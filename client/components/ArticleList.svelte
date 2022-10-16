<script lang="ts">
  import { onMount } from "svelte";
  import { Link } from "svelte-routing";
  import { HeadingType } from "./types";
  import type { ArticleInterface } from "./types";
  import HorizontalDashHeading from "./HorizontalDashHeading.svelte";

  export let articles: ArticleInterface[];
  export let tag: string | null = null;
  export let articleCount: number | null = null;

  onMount(async () => {
    const url = new URL("/api/articles", window.location.origin);
    const params = {
      ...(articleCount && { limit: String(articleCount) }),
      ...(tag && { tag }),
    };

    if (Object.keys(params).length !== 0) {
      url.search = new URLSearchParams(params).toString();
    }

    const res = await fetch(url);
    articles = await res.json();
  });
</script>

{#if articles}
  {#each articles as article}
    {@const date = new Date(article.metadata.published)
      .toISOString()
      .slice(0, 10)}
    <article>
      <HorizontalDashHeading heading={HeadingType.H3}>
        <Link to="article/{article.metadata.slug}"
          >{article.metadata.title}</Link
        >
      </HorizontalDashHeading>
      <div>
        <time datetime={date}>{date}</time>
        ::
        <span>
          {#each article.metadata.tags as tag, i}
            <Link to="article/tag/{tag}">#{tag}</Link
            >{#if i < article.metadata.tags.length - 1},&nbsp;{/if}
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

    :global {
      div + div,
      div + div a {
        color: lighten($color-brand, 12);
        margin: 1rem 0;
      }
    }
  }
</style>
