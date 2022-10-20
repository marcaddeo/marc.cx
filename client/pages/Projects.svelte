<script lang="ts">
  import { VerticalLineHeading, ProjectList } from "../components";
  import type { ProjectInterface } from "../components";

  export let projects: ProjectInterface[] | null = null;
</script>

<svelte:head>
  <title>Projects | marc.cx</title>
</svelte:head>

<div>
  <VerticalLineHeading>Projects</VerticalLineHeading>
</div>

<ProjectList {projects} />

<style lang="scss">
  div {
    margin: 3rem 0 4rem 0;
  }
</style>
