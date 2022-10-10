<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import { Header, Footer } from "./components";
  import { Home, Article, Articles, Projects, NotFound } from "./pages";

  export let url: string | null = null;
  export let ssrContent = null;
  if (ssrContent) {
    ssrContent = JSON.parse(ssrContent);
  }

  let articleComponent = Article;
  let articleComponentProps = {};
  if (ssrContent && ssrContent?.not_found) {
    articleComponent = NotFound;
  } else {
    articleComponentProps = {
      article: ssrContent,
    };
  }
</script>

<Header url="{url}" />

<Router url="{url}">
  <main>
    <Route path="article/:slug" let:params>
      <svelte:component this={articleComponent} slug="{params.slug}" {...articleComponentProps} />
    </Route>
    <Route path="articles">
      <Articles articles="{ssrContent}" />
    </Route>
    <Route path="projects" component="{Projects}" />
    <Route path="/">
      <Home articles="{ssrContent}" />
    </Route>
    <Route component="{NotFound}" />
  </main>
</Router>

<Footer />

<style lang="scss">
  main {
    grid-area: main;
  }
</style>
