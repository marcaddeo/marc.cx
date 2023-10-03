<svelte:options tag="code-block" />
<script lang="ts">
  import hljs from "highlight.js/lib/core";
  import Highlight from "svelte-highlight";
  import * as languages from "svelte-highlight/languages";
  import * as styles from "svelte-highlight/styles";

  export let style: string = "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.6.0/styles/base16/paraiso.min.css";
  export let code: string;
  export let language: string;

  // Add support for html language alias to svelte-highlight.
  $: if (language === "html") {
    // Other languages that might be used in an html document must be
    // registered ahead of time.
    for (const name of ["css", "javascript", "typescript"]) {
      let lang = languages[name];
      hljs.registerLanguage(lang.name, lang.register);
    }

    // There isn't an html language, just xml + css/js registered.
    language = "xml";
  }
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
