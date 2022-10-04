<script lang="ts">
  import { Link } from "svelte-routing";

  export let articleCount: number;

  interface Article {
    title: string,
    excerpt: string,
    slug: string,
    published: Date,
    tags: string[],
  }

  const articles: Article[] = [
    {
      title: "Auto Renewing SSL Certs on NGINX with Let's Encrypt",
      excerpt: "How I'm using Let's Encrypt for free auto renewing SSL certificates on NGINX",
      slug: "auto-renewing-ssl-certs-on-nginx-with-lets-encrypt",
      published: new Date("2015-12-12"),
      tags: ["linux", "devops"],
    },
    {
      title: "Hello World",
      excerpt: "In which I make the first blog article on my new site. A short introduction to my blog, and the technologies behind it.",
      slug: "hello-world",
      published: new Date("2015-05-26"),
      tags: ["elixir"],
    },
  ].slice(0, articleCount);
</script>

{#each articles as article}
  {@const date = article.published.toISOString().slice(0, 10)}
  <article>
    <h3><Link to="article/{article.slug}">{article.title}</Link></h3>
    <div>
      <time datetime="{date}">{date}</time>
      :: 
      <span>
      {#each article.tags as tag, i}
        <a href="/tag/{tag}">#{tag}</a>{#if i < (article.tags.length - 1)},&nbsp;{/if}
      {/each}
      </span>
    </div>
    <p>{article.excerpt}</p>
  </article>
{/each}

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
