<svelte:options tag="code-block" />
<script lang="ts">
  import Highlight from "svelte-highlight";
  import * as languages from "svelte-highlight/languages";
  import * as styles from "svelte-highlight/styles";

  export let style: string = "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.6.0/styles/base16/paraiso.min.css";
  export let code: string;
  export let language: string;
</script>

<link rel="stylesheet" href="/static/build/bundle.css">

{#if style && styles[style]}
  <div>
    {@html styles[style]}
  </div>
{:else if style}
  <link rel="stylesheet" href="{style}">
{/if}

<Highlight language={languages[language]} {code} />
