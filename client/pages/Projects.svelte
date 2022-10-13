<script lang="ts">
  import { SpecialHeading, ProjectList } from "../components";
  import type { ProjectInterface } from "../components";

  export let projects: ProjectInterface[] | null = null;
</script>

<svelte:head>
  <title>Projects | marc.cx</title>
</svelte:head>

<SpecialHeading>Projects</SpecialHeading>

<ProjectList {projects} />

<style lang="scss">
</style>
