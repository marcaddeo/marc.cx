<script lang="ts">
  import { ArticleList, SpecialHeading } from "../components";

  export let articles;
</script>

<svelte:head>
  <title>Home | marc.cx</title>
</svelte:head>

<div class="container">
  <section class="greeting-section">
    <div class="greeting-container">
      <div class="greeting">Nice to meet you; I'm</div>
      <div class="name"><span>👨🏻‍💻</span>Marc</div>
    </div>
    <div class="intro">
      <p>
        I'm a back-end web developer that does stuff and what-not, here's some
        more words that make me sound good and what not. I like to code with
        PHP, Rust, JavaScript/TypeScript, Svelte, and other shit and what-not.
        Maybe this is long enough now?
      </p>
    </div>
  </section>

  <section class="article-list">
    <SpecialHeading>Recent Articles</SpecialHeading>

    <ArticleList {articles} articleCount="3" />
  </section>
</div>

<style lang="scss">
  .container {
    display: grid;
    grid-template-areas:
      "greeting"
      "articles";
    grid-template-rows: .9fr 1fr;
  }

  .greeting-section {
    grid-area: greeting;
    display: grid;
    grid-template-columns: .9fr;
    grid-template-rows: .5fr .5fr;;
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
    }
  }

  .article-list {
    grid-area: articles;
  }
</style>
