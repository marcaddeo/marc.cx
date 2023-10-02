<svelte:options tag="alert-block" />
<script lang="ts">
  export let type: string;
  export let value: string;
</script>

<div data-type="{type}">
  <header>
    <svg viewBox="0 0 16 16" version="1.1" width="16" height="16" aria-hidden="true">
    {#if type == "note"}
      <path d="M0 8a8 8 0 1 1 16 0A8 8 0 0 1 0 8Zm8-6.5a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13ZM6.5 7.75A.75.75 0 0 1 7.25 7h1a.75.75 0 0 1 .75.75v2.75h.25a.75.75 0 0 1 0 1.5h-2a.75.75 0 0 1 0-1.5h.25v-2h-.25a.75.75 0 0 1-.75-.75ZM8 6a1 1 0 1 1 0-2 1 1 0 0 1 0 2Z"></path>
    {/if}
    {#if type == "important"}
      <path d="M0 1.75C0 .784.784 0 1.75 0h12.5C15.216 0 16 .784 16 1.75v9.5A1.75 1.75 0 0 1 14.25 13H8.06l-2.573 2.573A1.458 1.458 0 0 1 3 14.543V13H1.75A1.75 1.75 0 0 1 0 11.25Zm1.75-.25a.25.25 0 0 0-.25.25v9.5c0 .138.112.25.25.25h2a.75.75 0 0 1 .75.75v2.19l2.72-2.72a.749.749 0 0 1 .53-.22h6.5a.25.25 0 0 0 .25-.25v-9.5a.25.25 0 0 0-.25-.25Zm7 2.25v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 9a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
    {/if}
    {#if type == "warning"}
      <path d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0 1 14.082 15H1.918a1.75 1.75 0 0 1-1.543-2.575Zm1.763.707a.25.25 0 0 0-.44 0L1.698 13.132a.25.25 0 0 0 .22.368h12.164a.25.25 0 0 0 .22-.368Zm.53 3.996v2.5a.75.75 0 0 1-1.5 0v-2.5a.75.75 0 0 1 1.5 0ZM9 11a1 1 0 1 1-2 0 1 1 0 0 1 2 0Z"></path>
    {/if}
    </svg>
    <span>{type}</span>
  </header>

  {@html value}
</div>

<style lang="scss">
  $colors: (
    note: #2f81f7,
    important: #a371f7,
    warning: #d29922,
  );

  $filters: (
    note: brightness(0) saturate(100%) invert(44%) sepia(74%) saturate(3440%) hue-rotate(201deg) brightness(100%) contrast(94%),
    important: brightness(0) saturate(100%) invert(50%) sepia(37%) saturate(5585%) hue-rotate(232deg) brightness(108%) contrast(94%),
    warning: brightness(0) saturate(100%) invert(65%) sepia(48%) saturate(699%) hue-rotate(1deg) brightness(88%) contrast(89%),
  );

  div {
    border-left: 0.25rem solid black;
    padding: 0 1rem;
    margin: 20px 0;

    :global(p) {
      color: red;
      &:first-of-type {
        margin-top: 0;
      }
      &:last-of-type {
        margin-bottom: 0;
      }
    }

    header span {
      &::first-letter {
        text-transform: capitalize;
      }

      display: inline-block;
    }
  }

  $types: note, important, warning;
  @each $type in $types {
    [data-type="#{$type}"] {
      border-left-color: map-get($colors, $type);

      header {
        color: map-get($colors, $type);

        svg {
          filter: map-get($filters, $type);
        }
      }
    }
  }
</style>
