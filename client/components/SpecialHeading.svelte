<script context="module" lang="ts">
  export const enum HeadingType {
    H1 = "h1",
    H2 = "h2",
    H3 = "h3",
    H4 = "h4",
    H5 = "h5",
    H6 = "h6",
  };
</script>

<script lang="ts">
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
    h1, h2, h3, h4, h5, h6 {
      display: grid;
      grid-template-areas:
        "text gradient";
      grid-template-columns: .4fr .6fr;
      padding: .3rem 0;
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
