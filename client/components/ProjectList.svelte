<script lang="ts">
  import { onMount } from "svelte";
  import type { ProjectInterface } from "./types";

  export let projects: ProjectInterface[] | null = null;

  onMount(async () => {
    const res = await fetch("/api/projects");
    projects = await res.json();
  });
</script>

{#if projects}
  {#each projects as project}
    <article>
      <h3>{project.metadata.title}</h3>
      <div>
        <span>
        {#each project.metadata.tags as tag, i}
          #{tag}{#if i < (project.metadata.tags.length - 1)},&nbsp;{/if}
        {/each}
        </span>
      </div>
      <p>{@html project.html}</p>
    </article>
  {/each}
{/if}

<style lang="scss">
  article {
    margin: 3rem 0;
    font-family: $font-fira-code;

    & h3 {
      border-bottom: 3px dotted $color-brand;
      position: relative;
      padding: 1rem 0;
      margin: .8rem 0;

      &:after {
        content: "";
        display: block;
        position: absolute;
        bottom: 2px;
        width: 100%;
        border-bottom: 3px dotted $color-brand;
      }
    }

    div span  {
      display: block;
      color: lighten($color-brand, 12);
      margin: 1rem 0;
    }
  }
</style>
