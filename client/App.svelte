<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import { Header, Footer } from "./components";
  import { Home, Article, Articles, Projects } from "./pages";

  export let url: string | null = null;
  export let ssrContent = null;
</script>

<Header url="{url}" />

<Router url="{url}">
  <main>
    <Route path="article/:slug" let:params>
      <Article slug="{params.slug}" article="{JSON.parse(ssrContent)}" />
    </Route>
    <Route path="articles">
      <Articles articles="{JSON.parse(ssrContent)}" />
    </Route>
    <Route path="projects" component="{Projects}" />
    <Route path="/">
      <Home articles="{JSON.parse(ssrContent)}" />
    </Route>
  </main>
</Router>

<Footer />

<style lang="scss">
  main {
    grid-area: main;
  }
</style>
