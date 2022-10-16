<script lang="ts">
  import { HeadingType } from "./types";

  export let heading: HeadingType = HeadingType.H1;
</script>

<div>
  <svelte:element this={heading}>
    <span>
      <slot />
    </span>
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
      display: grid;
      grid-template-areas: "text gradient";
      grid-template-columns: 0.4fr 0.6fr;
      padding: 0.3rem 0;
      font-family: $font-fira-code;

      span {
        grid-area: text;
        text-align: center;
        background: $color-brand;
        color: white;
      }

      &:after {
        content: "";
        grid-area: gradient;
        background: repeating-linear-gradient(
          90deg,
          $color-brand,
          $color-brand 2px,
          transparent 0,
          transparent 10px
        );
      }
    }
  }
</style>
