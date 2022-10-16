<script lang="ts">
  import { HeadingType } from "./types";

  export let heading: HeadingType = HeadingType.H1;
</script>

<div>
  <svelte:element this={heading}>
    <slot />
  </svelte:element>
</div>

<style lang="scss">
  div :global {
    h1,
    h2,
    h3,
    h4,
    h5,
    h6 {
      border-bottom: 3px dotted $color-brand;
      position: relative;
      padding: 1rem 0;
      margin: 0.8rem 0;

      &:after {
        content: "";
        display: block;
        position: absolute;
        bottom: 2px;
        width: 100%;
        border-bottom: 3px dotted $color-brand;
      }
    }
  }
</style>
