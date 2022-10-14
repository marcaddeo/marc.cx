<script lang="ts">
  import { onMount } from "svelte";
  import { HorizontalDashHeading, HeadingType } from "./index";
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
      <HorizontalDashHeading heading={HeadingType.H3}>
        <a href="{project.metadata.link}" target="_blank">{project.metadata.title}</a>
      </HorizontalDashHeading>
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

    div span  {
      display: block;
      color: lighten($color-brand, 12);
      margin: 1rem 0;
    }
  }
</style>
