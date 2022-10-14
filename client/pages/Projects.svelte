<script lang="ts">
  import { VerticalLineHeading, ProjectList } from "../components";
  import type { ProjectInterface } from "../components";

  export let projects: ProjectInterface[] | null = null;
</script>

<svelte:head>
  <title>Projects | marc.cx</title>
</svelte:head>

<VerticalLineHeading>Projects</VerticalLineHeading>

<ProjectList {projects} />

<style lang="scss">
</style>
