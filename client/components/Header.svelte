<script lang="ts">
  import { Router, Link } from "svelte-routing";

  export let url: string = "";

  let active = false;
</script>

<header>
  <Router {url}>
    <span><Link to="/">~/marc</Link></span>
    <button class:active on:click={() => active = !active} aria-hidden="true">
      <span></span>
      Menu
    </button>
    <nav class:active on:click={() => active = !active}>
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
    text-transform: lowercase;
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

  /* Desktop */
  @media screen and (min-width: 600px) {
    button {
      display: none;
    }

    nav {
      grid-area: header-right;
      text-align: right;
    }

    nav :global(a) {
      padding: 0 0.6rem;

      &:last-of-type {
        padding-right: 0;
      }
    }
  }

  /* Mobile */
  @media screen and (max-width: 600px) {
    header {
      grid-template-areas:
        "header-top-left header-top-right"
        "header-bottom header-bottom";
    }

    span {
      grid-area: header-top-left;
    }

    nav {
      grid-area: header-bottom;
      transform: translateY(20px);

      transition: all 0.5s;
      max-height: 0;
      overflow: hidden;
      visibility: hidden;

      & :global(a) {
        text-align: center;
        display: block;
        margin: 1rem 0;
      }
    }

    nav.active {
      visibility: visible;
      max-height: 1000px;
    }

    button {
      display: block;
      position: relative;
      width: 26px;
      height: 40px;
      font-size: 0;
      background: transparent;
      color: transparent;
      border: 0;
      transition-timing-function: linear;
      transition-duration: 0.15s;
      transition-property: opacity,filter;
      margin: -7px 0 0 auto;
      padding: 0 2px 3px 0;
      z-index: 100;
      grid-area: header-top-right;

      &:hover {
        cursor: pointer;
      }

      @media screen and (min-width: 600px) {
        display: none;
      }

      & span,
      & span::before,
      & span::after {
        position: absolute;
        width: 25px;
        height: 3px;
        transition-timing-function: ease;
        transition-duration: 0.15s;
        transition-property: transform;
        border-radius: 4px;
        background-color: darken($color-brand, 10%);
        left: 0;
        z-index: 41;
      }

      & span {
        transition-timing-function: cubic-bezier(0.55, 0.055, 0.675, 0.19);
        transition-duration: 75ms;
      }

      & span::before,
      & span::after {
        display: block;
        content: '';
      }

      & span::before {
        transition: top 75ms ease 0.12s,opacity 75ms ease;
        top: -8px;
      }

      & span::after {
        transition: bottom 75ms ease 0.12s,transform 75ms cubic-bezier(0.55, 0.055, 0.675, 0.19);
        bottom: -8px;
      }

      // Styles for when it turns into an X
      &.active span {
        transition-delay: 75ms;
        transform: translate3d(0, 0, 0) rotate(135deg);
        width: 23px;

        &::before {
          transition-delay: 0s;
          opacity: 0;
          transform: translate(0, 10px);
          width: 23px;
          top: -10px;
        }

        &::after {
          transition-delay: 75ms;
          transform: translate3d(0, -10.5px, 0) rotate(-270deg);
          width: 23px;
          bottom: -10px;
        }
      }
    }
  }
</style>
