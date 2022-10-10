<script lang="ts">
  import { onMount } from "svelte";
  import { Link } from "svelte-routing";

  interface ArticleMetadata {
    title: string,
    excerpt: string,
    slug: string,
    published: Date,
    tags: string[],
  }

  interface Article {
    metadata: ArticleMetadata,
    html: string,
  }

  export let articles: Articles[];
  export let articleCount: number | null = null;

  onMount(async () => {
    const url = new URL("/api/articles", window.location.origin);
    if (articleCount) {
      url.search = new URLSearchParams({limit: articleCount});
    }

    const res = await fetch(url);
    articles = await res.json();
  });
</script>

{#if articles}
  {#each articles as article}
    {@const date = new Date(article.metadata.published).toISOString().slice(0, 10)}
    <article>
      <h3><Link to="article/{article.metadata.slug}">{article.metadata.title}</Link></h3>
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

    & h3 {
      border-bottom: 3px dotted $color-brand;
      position: relative;
      padding: 1rem 0;
      margin: .8rem 0;

      &:after {
        content: "";
        display: block;
        position: absolute;
        bottom: 2px;
        width: 100%;
        border-bottom: 3px dotted $color-brand;
      }
    }

    div, div a {
      color: lighten($color-brand, 12);
      margin: 1rem 0;
    }
  }
</style>
