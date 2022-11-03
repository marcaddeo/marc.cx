<script lang="ts">
  import { Router, Route } from "svelte-routing";
  import { Header, Footer } from "./components";
  import { Home, Article, Articles, Tag, Projects, NotFound } from "./pages";

  export let url: string | null = null;
  export let props: object = {};
</script>

<Header {url} />

<Router {url}>
  <main>
    <Route path="articles/tag/:tag" let:params>
      <Tag tag={params.tag} {...props} />
    </Route>
    <Route path="article/:slug" let:params>
      <Article slug={params.slug} {...props} />
    </Route>
    <Route path="articles">
      <Articles {...props} />
    </Route>
    <Route path="projects">
      <Projects {...props} />
    </Route>
    <Route path="/">
      <Home {...props} />
    </Route>
    <Route component={NotFound} />
  </main>
</Router>

<Footer />

<style lang="scss">
  main {
    grid-area: main;
  }
</style>
