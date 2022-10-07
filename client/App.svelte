<script lang="ts">
  import { onMount } from "svelte";
  import { Router, Link, Route } from "svelte-routing";
  import Header from "./components/Header.svelte";
  import Footer from "./components/Footer.svelte";
  import Home from "./pages/Home.svelte";
  import Article from "./pages/Article.svelte";
  import Articles from "./pages/Articles.svelte";
  import Projects from "./pages/Projects.svelte";

  export let url: string = "";

  onMount(() => {
    fetch('https://jsonplaceholder.typicode.com/todos/1')
      .then(response => response.json())
      .then(json => console.log(json));
  });

</script>

<Header {url} />

<Router {url}>
  <main>
    <Route path="/" component="{Home}" />
    <Route path="article/:slug" component="{Article}" />
    <Route path="articles" component="{Articles}" />
    <Route path="projects" component="{Projects}" />
  </main>
</Router>

<Footer />

<style lang="scss">
  main {
    grid-area: main;
  }
</style>
