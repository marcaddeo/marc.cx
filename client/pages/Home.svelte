<script lang="ts">
  import { ArticleList, VerticalLineHeading, HeadingType } from "../components";
  import type { ArticleInterface } from "../components";

  export let articles: ArticleInterface[] | null = null;
</script>

<svelte:head>
  <title>Home | marc.cx</title>
</svelte:head>

<div class="container">
  <section class="greeting-section">
    <div class="greeting-container">
      <h1>
        <span class="greeting">Nice to meet you; I'm</span>
        <span class="name"><span>👨🏻‍💻</span>Marc</span>
      </h1>
    </div>
    <div class="intro">
      <p>
        I'm the Director of Technology a
        <a href="https://mythicdigital.io">Mythic Digital</a>. In my free time
        I enjoy PC gaming, tinkering with and automating my Homelab with
        Ansible, coding with Rust, and Cooking.
      </p>
    </div>
  </section>

  <section class="article-list">
    <VerticalLineHeading heading={HeadingType.H2}
      >Recent Articles</VerticalLineHeading
    >

    <ArticleList {articles} articleCount={3} />
  </section>
</div>

<style lang="scss">
  .container {
    display: grid;
    grid-template-areas:
      "greeting"
      "articles";
    grid-template-rows: 0.8fr 1fr;
  }

  .greeting-section {
    grid-area: greeting;
    display: grid;
    grid-template-columns: 0.9fr;
    grid-template-rows: 0.5fr 0.5fr;
    grid-template-areas:
      "greeting"
      "intro";
    justify-content: center;
    justify-items: center;
    text-align: center;
    font-family: $font-roboto;

    .greeting-container {
      grid-area: greeting;
      margin: 3rem 0;
      color: $color-heading-dark;

      .greeting {
        display: block;
        font-family: $font-fira-code;
        letter-spacing: -2px;
        font-size: 2rem;
      }

      .name {
        font-size: 4rem;
      }

      .name span {
        margin-right: 1.25rem;
      }
    }

    .intro {
      grid-area: intro;
      font-family: $font-fira-code;
      text-align: justify;
    }
  }

  .article-list {
    grid-area: articles;
  }
</style>
