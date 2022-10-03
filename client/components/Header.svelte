<script lang="ts">
  import { Router } from "svelte-routing";
  import NavLink from "./NavLink.svelte"

  export let url: string = "";
</script>

<header>
  <span>~/marc</span>
  <nav>
    <Router url="{url}">
      <NavLink to="/">Home</NavLink>
      <NavLink to="projects">Projects</NavLink>
    </Router>
    <a target="_blank" href="https://github.com/marcaddeo">GitHub</a>
  </nav>
</header>

<style>
  header {
    grid-area: header;
  }
</style>
