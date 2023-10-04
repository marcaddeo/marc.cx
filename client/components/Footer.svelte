<script lang="ts">
  const currentYear = new Date().getFullYear();
</script>

<footer>
  <div>
    &copy; 2015-{currentYear} Marc Addeo
  </div>
  <div>
    <a href="mailto:hi@marc.cx">hi@marc.cx</a>
  </div>
</footer>

<style lang="scss">
  footer {
    grid-area: footer;
    display: grid;
    grid-template-areas: "footer-left footer-right";
    align-items: end;
    margin: 1rem 0;
  }

  div {
    grid-area: footer-left;
    color: $color-subtext;
  }

  div + div {
    grid-area: footer-right;
    text-align: right;
  }
</style>
