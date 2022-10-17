<svelte:options tag="code-block" />
<script lang="ts">
  import Highlight from "svelte-highlight";
  import * as languages from "svelte-highlight/languages";
  import * as styles from "svelte-highlight/styles";

  export let style = "github";
  export let code: string;
  export let language: string;
</script>

<link rel="stylesheet" href="/static/build/bundle.css">

<div>
  {#if styles[style]}
    {@html styles[style]}
  {/if}
</div>

<Highlight language={languages[language]} {code} />
