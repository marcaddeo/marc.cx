<script lang="ts">
  // @todo do we even need this NavLink component? The Link component will
  // automatically add aria-current="page" to the active links, so we could
  // just style off of that.
  import { Link } from "svelte-routing";

  export let to: string = "";

  interface LinkProps {
    location: string,
    href: string,
    isPartiallyCurrent: bool,
    isCurrent: bool,
  }

  function getProps({ location, href, isPartiallyCurrent, isCurrent }: LinkProps) {
    const isActive: bool = href === "/" ? isCurrent : isPartiallyCurrent || isCurrent;

    // The object returned here is spread on the anchor element's attributes.
    if (isActive) {
      return { class: "active" };
    }
    return {};
  }
</script>

<Link to="{to}" getProps="{getProps}">
  <slot />
</Link>

<style>
</style>
