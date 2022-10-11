<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import { Header, Footer } from "./components";
  import { Home, Article, Articles, Projects, NotFound } from "./pages";

  export let url: string | null = null;
  export let props: object = {};
</script>

<Header url="{url}" />

<Router url="{url}">
  <main>
    <Route path="article/:slug" let:params>
      <Article slug="{params.slug}" {...props} />
    </Route>
    <Route path="articles">
      <Articles {...props} />
    </Route>
    <Route path="projects" component="{Projects}" />
    <Route path="/">
      <Home  {...props} />
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
