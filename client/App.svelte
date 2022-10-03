<script lang="ts">
  import { Router, Link, Route } from "svelte-routing";
  import Header from "./components/Header.svelte"
  import Footer from "./components/Footer.svelte"
  import Home from "./pages/Home.svelte"
  import Projects from "./pages/Projects.svelte"

  export let url: string = "";
</script>

<Header {url} />

<Router {url}>
  <main>
    <Route path="/" component="{Home}" />
    <Route path="projects" component="{Projects}" />
  </main>
</Router>

<Footer />

<style>
  main {
    grid-area: main;
  }
</style>
