<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import Header from "./components/Header.svelte";
  import Footer from "./components/Footer.svelte";
  import Home from "./pages/Home.svelte";
  import Article from "./pages/Article.svelte";
  import Articles from "./pages/Articles.svelte";
  import Projects from "./pages/Projects.svelte";

  export let url: string | null = null;
  export let ssrContent: string | null = null;
</script>

<Header url="{url}" />

<Router url="{url}">
  <main>
    <Route path="article/:slug" let:params>
      <Article slug="{params.slug}" content="{ssrContent}" />
    </Route>
    <Route path="articles" component="{Articles}" />
    <Route path="projects" component="{Projects}" />
    <Route path="/" component="{Home}" />
  </main>
</Router>

<Footer />

<style lang="scss">
  main {
    grid-area: main;
  }
</style>
