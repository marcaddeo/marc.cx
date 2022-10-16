<script lang="ts">
  import { Router, Link } from "svelte-routing";

  export let url: string = "";
</script>

<header>
  <Router {url}>
    <span><Link to="/">~/marc</Link></span>
    <nav>
      <Link to="/">Home</Link>
      <Link to="articles">Articles</Link>
      <Link to="projects">Projects</Link>
      <a target="_blank" href="https://github.com/marcaddeo">GitHub</a>
      <a target="_blank" href="https://www.linkedin.com/in/marc-addeo/"
        >LinkedIn</a
      >
    </nav>
  </Router>
</header>

<style lang="scss">
  header {
    grid-area: header;
    display: grid;
    grid-template-areas: "header-left header-right";
    margin: 1rem 0;
    font-family: $font-fira-code;
  }

  span {
    grid-area: header-left;
    color: $color-brand;
    font-size: 1rem;
  }

  nav {
    grid-area: header-right;
    text-align: right;
    text-transform: lowercase;
  }

  nav :global(a) {
    padding: 0 0.6rem;

    &:last-of-type {
      padding-right: 0;
    }
  }

  nav :global(a[aria-current]) {
    padding: 0;

    &:before {
      content: "[";
    }

    &:after {
      content: "]";
    }
  }

  @media screen and (max-width: 600px) {
    /* @todo make the nav not suck... */
  }
</style>
